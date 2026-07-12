use actix_web::App;
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Logger;
use actix_web::web;

use crate::AppState;
use crate::routes;

pub(crate) fn init_app(
    data: AppState,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .wrap(Logger::default())
        .app_data(web::Data::new(data))
        .configure(routes)
}
