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

#[derive(Serialize, Deserialize)]
pub struct ChangeForm {
    action: String,
}

pub(crate) async fn change(
    form: web::Form<ChangeForm>,
    state: web::Data<AppState>,
) -> impl Responder {
    let mut game = state.game.lock().unwrap();
    match form.action.as_str() {
        "*" => { 
            game.initialize();
        }
        "/" => {
            game.spare();
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

    use actix_http::Request;
    use actix_http::body::MessageBody;
    use actix_web::dev::Service;
    use actix_web::dev::ServiceFactory;
    use actix_web::dev::ServiceRequest;

    use crate::AppState;
    use crate::ChangeForm;
    use crate::Mutex;
    use crate::bowling::Game;
    use crate::routes;
    use crate::test_fixtures::app::init_app;
    use actix_web::Error;
    use actix_web::dev::ServiceResponse;
    use scraper::{Html, Selector};

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

    async fn score_after_action<
        T: Service<Request, Response = ServiceResponse<impl MessageBody>, Error = Error>,
    >(
        service: T,
        action: &str,
    ) -> String {
        test::TestRequest::post()
            .uri("/change")
            .set_form(ChangeForm {
                action: action.to_string(),
            })
            .send_request(&service)
            .await;
        let request = test::TestRequest::default().to_request();
        let body = test::call_and_read_body(&service, request).await;
        let html = String::from_utf8(body.to_vec()).unwrap();
        let document = Html::parse_document(&html);
        let selector = Selector::parse("#score").unwrap();
        let div = document.select(&selector).next().unwrap();
        div.text().collect::<String>()
    }

    #[actix_web::test]
    async fn test_buttons_increase_the_score() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let score = &score_after_action(&service, "1").await;
        assert_that(score).contains("1");
        let score = &score_after_action(&service, "3").await;
        assert_that(score).contains("4");
        let score = &score_after_action(&service, "5").await;
        assert_that(score).contains("9");
        let score = &score_after_action(&service, "3").await;
        assert_that(score).contains("12");
    }
    #[actix_web::test]
    async fn test_spare_button_change_score_with_bonus() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let score = &score_after_action(&service, "1").await;
        assert_that(score).contains("1");
        let score = &score_after_action(&service, "/").await;
        assert_that(score).contains("10");
        let score = &score_after_action(&service, "5").await;
        assert_that(score).contains("20");
    }
    #[actix_web::test]
    async fn test_strike_button_change_score_with_bonus() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let score = &score_after_action(&service, "X").await;
        assert_that(score).contains("10");
        let score = &score_after_action(&service, "5").await;
        let score = &score_after_action(&service, "4").await;
        assert_that(score).contains("28");
    }
    #[actix_web::test]
    async fn test_spare_button_cant_change_score_on_new_frame() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let score = &score_after_action(&service, "/").await;
        assert_that(score).contains("0");
    }
    #[actix_web::test]
    async fn test_strike_button_cant_change_score_on_new_frame() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let score = &score_after_action(&service, "3").await;
        let score = &score_after_action(&service, "X").await;
        assert_that(score).contains("3");
    }
    #[actix_web::test]
    async fn test_star_button_renitialize_the_score() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let _ = &score_after_action(&service, "2").await;
        let _ = &score_after_action(&service, "7").await;
        let _ = &score_after_action(&service, "2").await;
        let _ = &score_after_action(&service, "7").await;
        let _ = &score_after_action(&service, "2").await;
        let score = &score_after_action(&service, "7").await;
        assert_that(score).contains("27");
        let score = &score_after_action(&service, "*").await;
        assert_that(score).contains("0");
    }
    #[actix_web::test]
    async fn test_action_cant_add_roll_exceeding_ten() {
        let state = AppState {
            game: Mutex::new(Game::new()),
        };
        let service = test::init_service(init_app(state)).await;
        let _ = &score_after_action(&service, "7").await;
        let score = &score_after_action(&service, "7").await;
        assert_that(score).contains("7");
    }
}
