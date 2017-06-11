use errors::*;
use notify;
use rocket::{Data, Request};
use sns::{self, MessageType, Notification, SubscriptionConfirmation};
use std::io::Read;

#[error(404)]
fn not_found(req: &Request) -> String {
  format!("Route {} not found", req.uri())
}

#[get("/ping")]
fn health_check_route() -> &'static str {
  "OK"
}

#[post("/notify?<options>", data = "<data>")]
fn notify_route(options: notify::Options, msg_type: MessageType, data: Data) -> Result<()> {
  let mut body_str = String::new();
  data.open().read_to_string(&mut body_str)?;
  match msg_type {
    MessageType::SubscriptionConfirmation => {
      let sc = SubscriptionConfirmation::parse(&body_str)?;
      sns::confirm_subscription(sc.subscribe_url.as_ref())
    }

    MessageType::Notification => {
      let notification = Notification::parse(&body_str)?;
      notify::send_notification(options, notification)
    },
    _ => Ok(()), // TODO
  }
}
