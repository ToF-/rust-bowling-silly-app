use askama::Template;
use askama_web::WebTemplate;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use actix_web::web::ServiceConfig;

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
pub struct HomePage {
    version: String,
}

pub async fn home() -> HomePage {
    HomePage {
        version: "1.0.0".to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn routes(service_config: &mut ServiceConfig) {
    service_config.route("/", web::get().to(home));
}

#[cfg(test)]
mod tests {
    use actix_web::App;
    use actix_web::test;
    use crate::home;
    use speculoos::assert_that;
    use speculoos::prelude::StrAssertions;

    #[actix_web::test]
    async fn test_app_displays_hello_world() {
        let app = test::init_service(App::new().service(home)).await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("Hello World!");
    }
    #[actix_web::test]
    async fn test_app_displays_version() {
        let app = test::init_service(App::new().service(home)).await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("1.1.0");
    }
}
