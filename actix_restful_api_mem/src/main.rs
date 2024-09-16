use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;

//賦予User特性
//Serialize提供序列化功能，可以將User轉換為JSON、XML等格式
//Deserialize提供反序列化功能，可以將JSON、XML等格式轉換為User
//Clone允許User可以使用.clone()複製
#[derive(Serialize, Deserialize, Clone)]
//建立User
struct User {
    id: Uuid, //使用UUID作為primary key
    name: String, //名稱
    email: String, //電子郵件
}

#[derive(Serialize,Deserialize)]
//建立UserDTO，用來接收前端傳來的資料
struct UserDTO {
    name: String,
    email: String,
}

//建立AppState，用來儲存User的資料
struct AppState {
    //Mutex允許多執行緒存取同一個資料，並且在同一時間只能有一個執行緒存取同一個資料
    users: Mutex<HashMap<Uuid, User>>,
}

//新增user的功能
async fn create_user(state: web::Data<AppState>, user: web::Json<UserDTO>) -> impl Responder {
    let new_user = User {
        id: Uuid::new_v4(), //產生一個新的UUID
        name: user.name.clone(),
        email: user.email.clone(),
    };
    //lock取得users的讀寫權，可以得到Result
    //unwrap從Result中取得值，如果Result是Error，會發生panic，讓程式終止
    //insert新增資料
    state.users.lock().unwrap().insert(new_user.id, new_user.clone());

    //回應剛剛新增的資料
    HttpResponse::Created().json(new_user)
}

//取得所有user的功能
async fn get_users(state: web::Data<AppState>) -> impl Responder {
    let users = state.users.lock().unwrap();
    //values取得users的所有資料
    //cloned將users的值全都複製一份
    //collect將cloned的值全都轉換為Vec
    let users_list: Vec<User> = users.values().cloned().collect();

    //回應剛剛取得的資料
    HttpResponse::Ok().json(users_list)
}

//取得單一user的功能
async fn get_user(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let users = state.users.lock().unwrap();
    //let Some(user)，用來確認Option是否有值，如果有值就回傳Some，並將值傳給user，沒有就回傳None
    //users.get(&user_id)，根據user_id來取得對應的user，如果有值就回傳Option<&User>，沒有就回傳None
    if let Some(user) = users.get(&user_id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

//更新單一user的功能
async fn update_user(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
    user_data: web::Json<UserDTO>,
) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&user_id) {
        user.name = user_data.name.clone();
        user.email = user_data.email.clone();
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

//刪除單一user的功能
async fn delete_user(state: web::Data<AppState>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    //remove刪除資料，並回傳Option
    //is_some判斷Option是否有值，有值就回傳true，沒有就回傳false
    if users.remove(&user_id).is_some() {
        HttpResponse::Ok().body("User deleted")
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //設定app_state，用來儲存User的資料
    let app_state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    //建立一個server，設定move讓子程式可以取得app_state的讀寫權
    HttpServer::new(move || {
        App::new()
            //讓子程式共享app_state
            .app_data(app_state.clone())
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::http::StatusCode;
    use actix_web::dev::ServiceResponse;

    //測試POST /users
    #[actix_web::test]
    async fn test_create_user() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(create_user))
        ).await;

        let new_user = UserDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
 
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::CREATED);

        let body: User = test::read_body_json(res).await;
        assert_eq!(body.name, "Test User");
        assert_eq!(body.email, "test@example.com");
    }

    //測試GET /users
    #[actix_web::test]
    async fn test_get_users() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(create_user))
                .route("/users", web::get().to(get_users))
        ).await;
        //先新增一個user，用來測試
        let new_user = UserDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
 
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let _res: ServiceResponse = test::call_service(&app, req).await;

        //以下是真正用來測試的部分
        let req2 = test::TestRequest::get().uri("/users").to_request();
        let res2 = test::call_service(&app, req2).await;

        assert_eq!(res2.status(), StatusCode::OK);

        let response_body: Vec<User> = test::read_body_json(res2).await;
        assert_eq!(response_body.len(), 1);
        assert_eq!(response_body[0].name, "Test User");
        assert_eq!(response_body[0].email, "test@example.com");
    }

    //測試GET /users/{id}
    #[actix_web::test]
    async fn test_get_user() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(create_user))
                .route("/users/{id}", web::get().to(get_user))
        ).await;
        //先新增一個user，用來測試
        let new_user = UserDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
 
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;

        //以下是真正用來測試的部分
        let body: User = test::read_body_json(res).await;
        //將uuid放置在路徑中
        let url_concat = format!("/users/{}", body.id);
        let req2 = test::TestRequest::get().uri(&url_concat).to_request();
        let res2 = test::call_service(&app, req2).await;

        assert_eq!(res2.status(), StatusCode::OK);

        let response_body: User = test::read_body_json(res2).await;
        assert_eq!(response_body.name, "Test User");
        assert_eq!(response_body.email, "test@example.com");
    }

    //測試UPDATE /users/{id}
    #[actix_web::test]
    async fn test_update_user() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(create_user))
                .route("/users/{id}", web::put().to(update_user))
        ).await;
        //先新增一個user，用來測試
        let new_user = UserDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
 
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;

        //以下是真正用來測試的部分
        let body: User = test::read_body_json(res).await;
        //將uuid放置在路徑中
        let url_concat = format!("/users/{}", body.id);
        let update_user = UserDTO {
            name: "Test User_updated".to_string(),
            email: "test_updated@example.com".to_string(),
        };
        let req2 = test::TestRequest::put()
            .uri(&url_concat)
            .set_json(&update_user)
            .to_request();
        let res2 = test::call_service(&app, req2).await;

        assert_eq!(res2.status(), StatusCode::OK);

        let response_body: User = test::read_body_json(res2).await;
        assert_eq!(response_body.name, "Test User_updated");
        assert_eq!(response_body.email, "test_updated@example.com");
    }

    //測試DELETE /users/{id}
    #[actix_web::test]
    async fn test_delete_user() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(create_user))
                .route("/users/{id}", web::delete().to(delete_user))
        ).await;
        //先新增一個user，用來測試
        let new_user = UserDTO {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
 
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;

        //以下是真正用來測試的部分
        let body: User = test::read_body_json(res).await;
        //將uuid放置在路徑中
        let url_concat = format!("/users/{}", body.id);
        let req2 = test::TestRequest::delete()
            .uri(&url_concat)
            .to_request();
        let res2 = test::call_service(&app, req2).await;

        assert_eq!(res2.status(), StatusCode::OK);

        let response_body = test::read_body(res2).await;
        assert_eq!(response_body, "User deleted");
    }
}