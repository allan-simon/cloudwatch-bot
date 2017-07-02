use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use rocket::{self, Data, Request, Rocket, State};
use std::io::Read;
use std::sync::atomic::{AtomicPtr, Ordering};
use tokio_core::reactor::Core;

use model::*;
use services;

struct TokioCore(AtomicPtr<Core>);
struct HttpsClient(AtomicPtr<Client<HttpsConnector<HttpConnector>>>);

pub(crate) fn setup_server() -> Rocket {
    let mut core = &mut Core::new().expect("Failed to init Tokio event loop");
    let handle = core.handle();
    let http_client = &mut Client::configure()
        .connector(HttpsConnector::new(4, &handle).expect("Failed to create HTTPS connector"))
        .build(&handle);

    rocket::ignite()
        .mount("/", routes![health_check_route, notify_route])
        .catch(errors![not_found])
        .manage(TokioCore(AtomicPtr::new(core)))
        .manage(HttpsClient(AtomicPtr::new(http_client)))
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("Route {} not found", req.uri())
}

#[get("/ping")]
fn health_check_route() -> &'static str {
    "OK"
}

#[post("/notify", data = "<request_body>")]
#[allow(unused)]
fn notify_route(
    msg_type: MessageType,
    request_body: Data,
    managed_core: State<TokioCore>,
    managed_http_client: State<HttpsClient>,
) {
    let mut core = unsafe { &mut *managed_core.0.load(Ordering::Relaxed) };
    let http_client = unsafe { &*managed_http_client.0.load(Ordering::Relaxed) };
    let mut body_str = &mut String::new();
    request_body.open().read_to_string(body_str).unwrap();
    match msg_type {
        MessageType::SubscriptionConfirmation => {
            let msg: SubscriptionConfirmation = body_str.parse().unwrap();
            services::confirm_subscription(http_client, core, &msg);
        }
        _ => (),
    }
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
