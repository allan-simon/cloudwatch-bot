use std::collections::HashMap;
use std::str::FromStr;

use super::errors::{EnumResult, ParseEnumError};

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
        let no_match = MessageType::from_str("foo");
        assert_eq!(no_match.is_ok(), false);
    }
}
