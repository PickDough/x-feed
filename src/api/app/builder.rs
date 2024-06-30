use std::sync::Arc;
use std::sync::Mutex;

use actix_web::body::MessageBody;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::web;
use actix_web::App;
use actix_web::Error;

use messaging::message_producer::MessageProducer;
use persistance::database::MessageReadDatabase;

use super::message;

#[derive(Clone)]
pub struct AppState {
    pub message_store: MessageReadDatabase,
    pub producer: Arc<Mutex<MessageProducer>>,
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
        .app_data(web::Data::new(state));

    app
}
