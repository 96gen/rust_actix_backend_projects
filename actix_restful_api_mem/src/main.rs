use actix_web::{web, App, HttpServer};
use state::AppState;
use std::collections::HashMap;
use std::sync::Mutex;

mod models;
mod state;
mod handlers;

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
            .route("/users", web::post().to(handlers::create_user))
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user))
            .route("/users/{id}", web::put().to(handlers::update_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
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
    use crate::models::{User, UserDTO};

    //測試POST /users
    #[actix_web::test]
    async fn test_create_user() {
        let app_state = web::Data::new(AppState {
            users: Mutex::new(HashMap::new()),
        });

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .route("/users", web::post().to(handlers::create_user))
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
                .route("/users", web::post().to(handlers::create_user))
                .route("/users", web::get().to(handlers::get_users))
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
                .route("/users", web::post().to(handlers::create_user))
                .route("/users/{id}", web::get().to(handlers::get_user))
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
                .route("/users", web::post().to(handlers::create_user))
                .route("/users/{id}", web::put().to(handlers::update_user))
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
                .route("/users", web::post().to(handlers::create_user))
                .route("/users/{id}", web::delete().to(handlers::delete_user))
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