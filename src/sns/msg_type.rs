use std::collections::HashMap;
use std::str::FromStr;

use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome, Request};
use sns::errors::*;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub(crate) enum MessageType {
    Notification,
    SubscriptionConfirmation,
    UnsubscribeConfirmation,
}

lazy_static! {
    static ref MESSAGE_TYPES: HashMap<&'static str, MessageType> = {
        let mut map = HashMap::new();
        map.insert("Notification", MessageType::Notification);
        map.insert("SubscriptionConfirmation", MessageType::SubscriptionConfirmation);
        map.insert("UnsubscribeConfirmation", MessageType::UnsubscribeConfirmation);
        map
    };
}

impl FromStr for MessageType {
    type Err = ParseEnumError<Self>;

    fn from_str(msg_type: &str) -> EnumResult<MessageType> {
        MESSAGE_TYPES.get(msg_type).cloned().ok_or(ParseEnumError {
            value: msg_type.to_string(),
            mapping: MESSAGE_TYPES.clone(),
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for MessageType {
    type Error = ParseEnumError<Self>;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        request
            .headers()
            .get_one("x-amz-sns-message-type")
            .ok_or(ParseEnumError {
                value: "".to_string(),
                mapping: MESSAGE_TYPES.clone(),
            })
            .and_then(MessageType::from_str)
            .into_outcome()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_from_string_matches() {
        let notification: MessageType = "Notification".parse().expect("should not happen");
        let sub_confirmation: MessageType = "SubscriptionConfirmation".parse().expect(
            "should not happen",
        );
        let unsub_confirmation: MessageType = "UnsubscribeConfirmation".parse().expect(
            "should not happen",
        );

        assert_eq!(notification, MessageType::Notification);
        assert_eq!(sub_confirmation, MessageType::SubscriptionConfirmation);
        assert_eq!(unsub_confirmation, MessageType::UnsubscribeConfirmation);
    }

    #[test]
    fn test_message_type_from_string_no_match() {
        let no_match: EnumResult<MessageType> = "foo".parse();
        assert_eq!(no_match.is_ok(), false);
    }
}
