
use actix_web::App;
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Logger;
use actix_web::web;
use crate::routes;

pub(crate) fn app(config: &mut web::ServiceConfig, data: AppData) {
    config
        .app_data(data)
        .configure(routes);
}

pub fn init_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new().wrap(Logger::default()).configure(app)
}
