use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::get;
use actix_web::http::header;
use actix_web::web;
use actix_web::web::ServiceConfig;
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

struct AppState {
    counter: Mutex<usize>,
}

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
pub struct HomePage {
    counter: String,
}

pub async fn home(state: web::Data<AppState>) -> impl Responder {
    let counter: usize = *state.counter.lock().unwrap();
    HomePage {
        counter: counter.to_string(),
    }
}

#[derive(Serialize,Deserialize)]
pub struct ChangeForm {
    action: String,
}

pub async fn change(form: web::Form<ChangeForm>, state: web::Data<AppState>) -> impl Responder {
    let mut counter = state.counter.lock().unwrap();
    match form.action.as_str() {
        "increase" => {
            *counter += 1;
        }
        "decrease" => {
            if *counter > 0 {
                *counter -= 1;
            }
        }
        _ => {}
    }

    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                counter: Mutex::new(0),
            }))
            .configure(routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn routes(service_config: &mut ServiceConfig) {
    service_config.route("/", web::get().to(home));
    service_config.route("/change", web::post().to(change));
}

#[cfg(test)]
mod tests {
    use crate::AppState;
    use crate::Mutex;
    use crate::home;
    use crate::routes;
    use crate::ChangeForm;
    use actix_web::App;
    use actix_web::test;
    use actix_web::web;
    use speculoos::assert_that;
    use speculoos::prelude::StrAssertions;

    #[actix_web::test]
    async fn test_app_displays_hello_world() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    counter: Mutex::new(0),
                }))
                .configure(routes),
        )
        .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("Hello World!");
    }
    #[actix_web::test]
    async fn test_app_displays_a_counter() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    counter: Mutex::new(4807),
                }))
                .configure(routes),
        )
        .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("4807");
    }
    #[actix_web::test]
    async fn test_app_increases_the_counter() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    counter: Mutex::new(41),
                }))
                .configure(routes),
        )
        .await;
        let changeRequest = test::TestRequest::post()
            .uri("/change")
            .set_form(ChangeForm {
                action: "increase".to_string(),
            })
            .send_request(&app)
            .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("42");
    }
    #[actix_web::test]
    async fn test_app_decreases_the_counter() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    counter: Mutex::new(41),
                }))
                .configure(routes),
        )
        .await;
        let changeRequest = test::TestRequest::post()
            .uri("/change")
            .set_form(ChangeForm {
                action: "decrease".to_string(),
            })
            .send_request(&app)
            .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("40");
    }
}
