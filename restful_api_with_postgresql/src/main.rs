use actix_web::{web, App, HttpServer};

mod db;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::create_pool();

    HttpServer::new(move || {
        App::new()
            //每個request都有獨立的連接池
            .app_data(web::Data::new(pool.clone()))
            .route("/todos", web::post().to(handlers::add_todo))
            .route("/todos", web::get().to(handlers::get_todos))
            .route("/todos/{id}", web::get().to(handlers::get_todo))
            .route("/todos/{id}", web::put().to(handlers::update_todo))
            .route("/todos/{id}", web::delete().to(handlers::delete_todo))
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
    use crate::models::{Todo, TodoDTO};

    //測試POST /todos
    #[actix_web::test]
    async fn test_create_todo() {
        let pool = db::create_pool();

        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/todos", web::post().to(handlers::add_todo))
        ).await;

        let new_todo = TodoDTO {
            title: "Test Title".to_string(),
            completed: false,
        };
 
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&new_todo)
            .to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::CREATED);

        let body: Todo = test::read_body_json(res).await;
        assert_eq!(body.title, "Test Title");
        assert_eq!(body.completed, false);
    }

    //測試GET /todos
    #[actix_web::test]
    async fn test_get_todos() {
        let pool = db::create_pool();

        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/todos", web::post().to(handlers::add_todo))
                .route("/todos", web::get().to(handlers::get_todos))
        ).await;

        let new_todo = TodoDTO {
            title: "Test Title".to_string(),
            completed: false,
        };
 
        let req_new = test::TestRequest::post()
            .uri("/todos")
            .set_json(&new_todo)
            .to_request();
        let _res_new: ServiceResponse = test::call_service(&app, req_new).await;

        //真正的測試
        let req = test::TestRequest::get().uri("/todos").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let response_body: Vec<Todo> = test::read_body_json(res).await;
        assert_eq!(response_body[response_body.len() - 1].title, "Test Title");
        assert_eq!(response_body[response_body.len() - 1].completed, false);
    }

    //測試GET /todos/{id}
    #[actix_web::test]
    async fn test_get_todo() {
        let pool = db::create_pool();

        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/todos", web::post().to(handlers::add_todo))
                .route("/todos/{id}", web::get().to(handlers::get_todo))
        ).await;

        let new_todo = TodoDTO {
            title: "Test Title".to_string(),
            completed: false,
        };
 
        let req_new = test::TestRequest::post()
            .uri("/todos")
            .set_json(&new_todo)
            .to_request();
        let res_new: ServiceResponse = test::call_service(&app, req_new).await;
        let body: Todo = test::read_body_json(res_new).await;

        //真正的測試
        let url_concat = format!("/todos/{}", body.id);
        let req = test::TestRequest::get().uri(&url_concat).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let response_body: Todo = test::read_body_json(res).await;
        assert_eq!(response_body.title, "Test Title");
        assert_eq!(response_body.completed, false);
    }

    //測試PUT /todos/{id}
    #[actix_web::test]
    async fn test_update_todo() {
        let pool = db::create_pool();

        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/todos", web::post().to(handlers::add_todo))
                .route("/todos/{id}", web::put().to(handlers::update_todo))
        ).await;

        let new_todo = TodoDTO {
            title: "Test Title".to_string(),
            completed: false,
        };
 
        let req_new = test::TestRequest::post()
            .uri("/todos")
            .set_json(&new_todo)
            .to_request();
        let res_new: ServiceResponse = test::call_service(&app, req_new).await;
        let body: Todo = test::read_body_json(res_new).await;

        //真正的測試
        let url_concat = format!("/todos/{}", body.id);

        let update_todo = TodoDTO {
            title: "Test Title_update".to_string(),
            completed: true,
        };
        let req = test::TestRequest::put().uri(&url_concat).set_json(&update_todo).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let response_body: Todo = test::read_body_json(res).await;
        assert_eq!(response_body.title, "Test Title_update");
        assert_eq!(response_body.completed, true);
    }

    //測試DELETE /todos/{id}
    #[actix_web::test]
    async fn test_delete_todo() {
        let pool = db::create_pool();

        let app = test::init_service(
            App::new()
            .app_data(web::Data::new(pool.clone()))
                .route("/todos", web::post().to(handlers::add_todo))
                .route("/todos/{id}", web::delete().to(handlers::delete_todo))
        ).await;

        let new_todo = TodoDTO {
            title: "Test Title".to_string(),
            completed: false,
        };
 
        let req_new = test::TestRequest::post()
            .uri("/todos")
            .set_json(&new_todo)
            .to_request();
        let res_new: ServiceResponse = test::call_service(&app, req_new).await;
        let body: Todo = test::read_body_json(res_new).await;

        //真正的測試
        let url_concat = format!("/todos/{}", body.id);
        let req = test::TestRequest::delete().uri(&url_concat).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let response_body = test::read_body(res).await;
        assert_eq!(response_body, "Todo deleted");
    }
}