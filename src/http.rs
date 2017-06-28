use rocket::{self, Request, Rocket};

pub(crate) fn setup_server() -> Rocket {
    rocket::ignite()
        .mount("/", routes![health_check_route])
        .catch(errors![not_found])
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
    use rocket::testing::MockRequest;
    use rocket::http::*;

    #[test]
    fn test_health_check() {
        let rocket = setup_server();
        let mut req = MockRequest::new(Method::Get, "/ping");
        let response = req.dispatch_with(&rocket);
        assert_eq!(response.status(), Status::Ok);
    }
}
