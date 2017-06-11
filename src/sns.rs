use errors::*;
use hyper::client::Client;
use hyper::header::ContentType;
use hyper::status::StatusCode;

use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use serde_json;

pub(crate) enum MessageType {
  Notification,
  SubscriptionConfirmation,
  UnsubscribeConfirmation,
}

impl MessageType {
  fn from_string(msg_type: &str) -> Option<MessageType> {
    match msg_type {
      "Notification" => Some(MessageType::Notification),
      "SubscriptionConfirmation" => Some(MessageType::SubscriptionConfirmation),
      "UnsubscribeConfirmation" => Some(MessageType::UnsubscribeConfirmation),
      _ => None,
    }
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for MessageType {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    request
      .headers()
      .get_one("x-amz-sns-message-type")
      .and_then(MessageType::from_string)
      .ok_or(())
      .into_outcome()
  }
}

#[derive(Debug, Deserialize)]
pub(crate) struct SubscriptionConfirmation {
  #[serde(rename = "TopicArn")]
  topic_arn: String,
  #[serde(rename = "SubscribeURL")]
  pub subscribe_url: String,
}

impl SubscriptionConfirmation {
  pub fn parse(str: &str) -> Result<SubscriptionConfirmation> {
    serde_json::from_str(str).map_err(CloudwatchNotifierError::Json)
  }
}

pub(crate) fn confirm_subscription(url: &str) -> Result<()> {
  let resp = Client::new().post(url).header(ContentType::json()).send()?;
  match resp.status {
    StatusCode::Ok => Ok(()),
    _ => Err(CloudwatchNotifierError::AutoSubscriptionBadStatus(url.to_string(), resp.status)),
  }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Notification {
  #[serde(rename = "AlarmName")]
  alarm_name: String,
  #[serde(rename = "AlarmDescription")]
  alarm_description: String,
  #[serde(rename = "Region")]
  region: String,
  #[serde(rename = "NewStateValue")]
  new_state_value: String,
}

impl Notification {
  pub fn parse(str: &str) -> Result<Notification> {
    let msg: serde_json::Value = serde_json::from_str(str)?;
    serde_json::from_str(str).map_err(CloudwatchNotifierError::Json)
  }
}