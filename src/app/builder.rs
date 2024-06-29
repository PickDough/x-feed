use actix_web::body::MessageBody;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::web;
use actix_web::App;
use actix_web::Error;
use sea_orm::DatabaseConnection;

use super::message;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub fn build_app(
    state: AppState,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let app = App::new()
        .service(message::fetch_message)
        .service(message::post_message)
        .service(message::streams)
        .app_data(web::Data::new(state.clone()));

    app
}
