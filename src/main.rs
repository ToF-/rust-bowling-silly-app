use actix_files::Files;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
pub(crate) use actix_web::http::header;
use actix_web::web;
use actix_web::web::ServiceConfig;
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

use crate::bowling::Game;

mod bowling;

pub(crate) struct AppState {
    game: Mutex<Game>,
}

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
pub struct HomePage {
    score: String,
}

pub(crate) async fn home(state: web::Data<AppState>) -> impl Responder {
    let game = state.game.lock().unwrap();
    HomePage {
        score: game.score().to_string(),
    }
}

#[derive(Serialize,Deserialize)]
pub struct ChangeForm {
    action: String,
}

pub(crate) async fn change(form: web::Form<ChangeForm>, state: web::Data<AppState>) -> impl Responder {
    let mut game = state.game.lock().unwrap();
    match form.action.as_str() {
        "/" => {
            game.add_roll(10);
        }
        "X" => {
            game.add_roll(10);
        }
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            let roll: usize = form.action.as_str().parse().unwrap();
            game.add_roll(roll);
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
            .service(Files::new("/static", "static"))
            .app_data(web::Data::new(AppState {
                game: Mutex::new(Game::new()),
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
mod test_fixtures;

#[cfg(test)]
mod tests {
    use actix_web::App;
    use actix_web::test;
    use actix_web::web;
    use speculoos::assert_that;
    use speculoos::prelude::StrAssertions;

    use actix_http::body::MessageBody;
    use actix_web::dev::ServiceFactory;
    use actix_web::dev::ServiceRequest;
use actix_web::dev::Service;
use actix_http::Request;

    use actix_web::dev::ServiceResponse;
    use actix_web::Error;
    use crate::AppState;
    use crate::ChangeForm;
    use crate::Mutex;
    use crate::bowling::Game;
use crate::routes;
    use crate::test_fixtures::app::init_app;

    #[actix_web::test]
    async fn test_app_displays_the_word_score() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let app = test::init_service(init_app(state)).await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("score");
    }
    #[actix_web::test]
    async fn test_app_displays_a_score() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState {
                    game: Mutex::new(Game::new()),
                }))
                .configure(routes),
        )
        .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&app, request)
            .await
            .escape_ascii()
            .to_string();
        assert_that(&body).contains("0");
    }

async fn body_after_action<T: Service<Request, Response=ServiceResponse::<impl MessageBody>, Error = Error>>(service: T,
            action: &str) -> String {
        test::TestRequest::post()
            .uri("/change")
            .set_form(ChangeForm {
                action: action.to_string()
            })
            .send_request(&service)
            .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&service, request)
            .await
            .escape_ascii()
            .to_string();
        body
    }
    #[actix_web::test]
    async fn test_buttons_increase_the_score() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let body = &body_after_action(&service, "1").await;
        assert_that(body).contains("1");
        let body = &body_after_action(&service, "3").await;
        assert_that(body).contains("4");
        let body = &body_after_action(&service, "5").await;
        assert_that(body).contains("9");
        let body = &body_after_action(&service, "3").await;
        assert_that(body).contains("12");
    }
}
