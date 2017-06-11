use errors::*;
use slack_api::chat;
use sns::Notification;

#[derive(FromForm)]
pub(crate) struct Options {
  slack: Option<String>,
}

pub(crate) fn send_notification(options: Options, notification: Notification) -> Result<()> {
  options
    .slack
    .map(|channel| send_to_slack(channel, notification));
  Ok(())
}

fn send_to_slack(channel: String, notification: Notification) {}
