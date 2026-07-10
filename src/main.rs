use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::get;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use actix_web::App;
    use speculoos::assert_that;
    use speculoos::prelude::StrAssertions;
    use crate::hello;

    #[actix_web::test]
    async fn test_app_displays_hello_world() {
        let app = test::init_service(App::new().service(hello)).await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("Hello World!");
    }
}
