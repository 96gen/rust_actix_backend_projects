use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

//使用GET方法，前往http://127.0.0.1:8080/，回應Hello world!
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

//使用POST方法，前往http://127.0.0.1:8080/echo，回應Hello 輸入的字串
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", req_body))
}

//回應Hey there!
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

//使用actix-web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //建立一個server
    HttpServer::new(|| {
        App::new()
            //service是一個方法，可以用來設定路由
            .service(hello)
            .service(echo)
            //手動設定路由，路徑是/hey
            .route("/hey", web::get().to(manual_hello))
    })
    //設定server在127.0.0.1:8080
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//測試
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App};
    use actix_web::test;
    use actix_web::http::StatusCode;
    use actix_web::dev::ServiceResponse;

    //測試GET /是否回應Hello world!
    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        let body = test::read_body(res).await;
        assert_eq!(body, "Hello world!");
    }

    //測試POST /echo是否回應傳入的內容
    #[actix_web::test]
    async fn test_echo() {
        let app = test::init_service(App::new().service(echo)).await;
        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload("test")
            .to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        let body = test::read_body(res).await;
        assert_eq!(body, "Hello test!");
    }

    //測試GET /hey是否回應Hey there!
    #[actix_web::test]
    async fn test_manual_hello() {
        let app = test::init_service(App::new().route("/hey", web::get().to(manual_hello))).await;
        let req = test::TestRequest::get().uri("/hey").to_request();
        let res: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);

        let body = test::read_body(res).await;
        assert_eq!(body, "Hey there!");
    }
}