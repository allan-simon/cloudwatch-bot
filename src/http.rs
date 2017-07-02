use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use rocket::{self, Request, Rocket};
use std::sync::atomic::AtomicPtr;
use tokio_core::reactor::Core;

struct TokioCore(AtomicPtr<Core>);
struct HttpsClient(AtomicPtr<Client<HttpsConnector<HttpConnector>>>);

pub(crate) fn setup_server() -> Rocket {
    let mut core = &mut Core::new().expect("Failed to init Tokio event loop");
    let handle = core.handle();
    let http_client = &mut Client::configure()
        .connector(HttpsConnector::new(4, &handle).expect("Failed to create HTTPS connector"))
        .build(&handle);

    rocket::ignite()
        .mount("/", routes![health_check_route])
        .catch(errors![not_found])
        .manage(TokioCore(AtomicPtr::new(core)))
        .manage(HttpsClient(AtomicPtr::new(http_client)))
}

#[error(404)]
pub(crate) fn not_found(req: &Request) -> String {
    format!("Route {} not found", req.uri())
}

#[get("/ping")]
pub(crate) fn health_check_route() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::*;
    use rocket::testing::MockRequest;

    #[test]
    fn test_health_check() {
        let rocket = setup_server();
        let mut req = MockRequest::new(Method::Get, "/ping");
        let response = req.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
    }
}
