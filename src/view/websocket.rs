
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Error;
use actix_web::ws;


use AppStateLike;
use pubsub::WsClientSession;



pub fn handler<S>(req: &HttpRequest<S>) -> Result<HttpResponse, Error>
    where
        S: AppStateLike + 'static,
{
    debug!("connection to the websocket");
    ws::start(req, WsClientSession::<S>::new())
}