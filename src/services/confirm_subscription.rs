use futures::Future;
use hyper::{Client, Error as HyperError, StatusCode, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use model::SubscriptionConfirmation;

#[derive(Debug)]
pub enum SubscriptionConfirmationError {
    BadStatus(StatusCode),
    HttpError(HyperError),
}

pub fn confirm_subscription(
    client: &Client<HttpsConnector<HttpConnector>>,
    core: &mut Core,
    sub_confirmation: &SubscriptionConfirmation,
) -> Result<(), SubscriptionConfirmationError> {
    // unwrap is safe here, as the URL sent by SNS is considered to be well-formed.
    let uri: Uri = sub_confirmation.subscribe_url.parse().expect(
        "Subscribe URL should be well formed",
    );
    let future_res = client.get(uri).then(|res| match res {
        Err(hyper_error) => Err(SubscriptionConfirmationError::HttpError(hyper_error)),
        Ok(resp) => {
            let status = resp.status();
            match status {
                StatusCode::Ok => Ok(()),
                _ => Err(SubscriptionConfirmationError::BadStatus(status)),
            }
        }
    });
    core.run(future_res)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn core() -> Core {
        return Core::new().expect("Failed to init Tokio event loop");
    }

    fn https_client(core: &Core) -> Client<HttpsConnector<HttpConnector>> {
        let handle = core.handle();
        Client::configure()
            .connector(HttpsConnector::new(4, &handle).expect("Failed to create HTTPS connector"))
            .build(&handle)
    }

    fn stub_sub_confirmation(url: &str) -> SubscriptionConfirmation {
        SubscriptionConfirmation {
            topic_arn: "".to_string(),
            subscribe_url: url.to_string(),
            timestamp: "".to_string(),
        }
    }

    #[test]
    fn test_subscription_confirmation_valid_url() {
        let mut core = &mut core();
        let client = &https_client(&core);
        let stub = stub_sub_confirmation("https://example.org/");

        let res = confirm_subscription(client, core, &stub);
        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_subscription_confirmation_valid_url_but_invalid_status_code() {
        let mut core = &mut core();
        let client = &https_client(&core);
        let stub = stub_sub_confirmation("https://google.com/foo");

        let res = confirm_subscription(client, core, &stub);
        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_subscription_confirmation_invalid_url() {
        let mut core = &mut core();
        let client = &https_client(&core);
        let stub = stub_sub_confirmation("https://example.c/");

        let res = confirm_subscription(client, core, &stub);
        assert_eq!(res.is_ok(), false);
    }


}
