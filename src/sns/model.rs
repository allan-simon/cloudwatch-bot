#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub(crate) enum MessageType {
    Notification,
    SubscriptionConfirmation,
    UnsubscribeConfirmation,
}

impl MessageType {
    #[allow(dead_code)]
    fn from_string(msg_type: &str) -> Option<MessageType> {
        match msg_type {
            "Notification" => Some(MessageType::Notification),
            "SubscriptionConfirmation" => Some(MessageType::SubscriptionConfirmation),
            "UnsubscribeConfirmation" => Some(MessageType::UnsubscribeConfirmation),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_from_string_matches() {
        let notification = MessageType::from_string("Notification").unwrap();
        let sub_confirmation = MessageType::from_string("SubscriptionConfirmation").unwrap();
        let unsub_confirmation = MessageType::from_string("UnsubscribeConfirmation").unwrap();

        assert_eq!(notification, MessageType::Notification);
        assert_eq!(sub_confirmation, MessageType::SubscriptionConfirmation);
        assert_eq!(unsub_confirmation, MessageType::UnsubscribeConfirmation);
    }

    #[test]
    fn test_message_type_from_string_no_match() {
        let no_match = MessageType::from_string("foo");
        assert_eq!(no_match.is_some(), false);
    }
}
