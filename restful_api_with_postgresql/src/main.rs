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
