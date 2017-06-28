use serde;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;

use sns::errors::*;

#[derive(Debug, Deserialize)]
struct Notification {
    #[serde(rename = "Message")]
    message: String,
}

impl FromStr for AlarmDetails {
    type Err = serde_json::Error;

    fn from_str(str: &str) -> JsonResult<AlarmDetails> {
        let notification: JsonResult<Notification> = serde_json::from_str(str);
        notification.and_then(|n| serde_json::from_str(&n.message))
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct AlarmDetails {
    #[serde(rename = "AlarmName")]
    name: String,
    #[serde(rename = "AlarmDescription")]
    description: String,
    #[serde(rename = "NewStateValue", deserialize_with = "AlarmState::deserialize_alarm_state")]
    new_state: AlarmState,
    #[serde(rename = "NewStateReason")]
    reason: String,
    #[serde(rename = "StateChangeTime")]
    timestamp: String,
    #[serde(rename = "OldStateValue", deserialize_with = "AlarmState::deserialize_alarm_state")]
    previous_state: AlarmState,
    #[serde(rename = "Trigger")]
    trigger: AlarmTrigger,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AlarmTrigger {
    #[serde(rename = "MetricName")]
    metric_name: String,
    #[serde(rename = "Namespace")]
    namespace: String,
    #[serde(rename = "Statistic")]
    statistic: String,
    #[serde(rename = "Dimensions")]
    dimensions: Vec<Dimension>,
    #[serde(rename = "ComparisonOperator")]
    op: String,
    #[serde(rename = "Period")]
    period: u8,
    #[serde(rename = "EvaluationPeriods")]
    nb_periods: u8,
    #[serde(rename = "Threshold")]
    threshold: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AlarmState {
    Ok,
    Alarm,
    InsufficientData,
}

lazy_static! {
    static ref ALARM_STATES: HashMap<&'static str, AlarmState> = {
        let mut map = HashMap::new();
        map.insert("OK", AlarmState::Ok);
        map.insert("ALARM", AlarmState::Alarm);
        map.insert("INSUFFICIENT_DATA", AlarmState::InsufficientData);
        map
    };
}

impl FromStr for AlarmState {
    type Err = ParseEnumError<Self>;

    fn from_str(msg_type: &str) -> EnumResult<AlarmState> {
        ALARM_STATES.get(msg_type).cloned().ok_or(ParseEnumError {
            value: msg_type.to_string(),
            mapping: ALARM_STATES.clone(),
        })
    }
}

impl AlarmState {
    fn deserialize_alarm_state<'de, D>(de: D) -> Result<AlarmState, D::Error>
    where
        D: serde::Deserializer<'de>,
    {

        let deser_result = serde::Deserialize::deserialize(de)?;
        match deser_result {
            serde_json::Value::String(ref s) => s.parse().map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom("AlarmState expects a string")),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Dimension {
    name: String,
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alarm_state_from_string_matches() {
        let ok: AlarmState = "OK".parse().expect("Should not happen");
        let alarm: AlarmState = "ALARM".parse().expect("Should not happen");
        let insufficient_data: AlarmState = "INSUFFICIENT_DATA".parse().expect("Should not happen");

        assert_eq!(ok, AlarmState::Ok);
        assert_eq!(alarm, AlarmState::Alarm);
        assert_eq!(insufficient_data, AlarmState::InsufficientData);
    }

    #[test]
    fn test_alarm_state_from_string_no_match() {
        let no_match: EnumResult<AlarmState> = "foo".parse();
        assert_eq!(no_match.is_ok(), false);
    }

    #[test]
    fn test_parse_notification() {
        let json = "\
        {
            \"Type\" : \"Notification\",
            \"MessageId\" : \"d921a633-3dbb-528e-a15c-e978b55d6156\",
            \"TopicArn\" : \"arn:aws:sns:us-east-1:097958131044:jabber-all\",
            \"Subject\" : \"OK: \\\"[RTB-US] UnHealtyhHostCount\\\" in US - N. Virginia\",
            \"Message\" : \"{\\\"AlarmName\\\":\\\"[RTB-US] UnHealthyHostCount\\\",\\\"AlarmDescription\\\":\\\"UnHealthyHostCount\\\",\\\"AWSAccountId\\\":\\\"097958131044\\\",\\\"NewStateValue\\\":\\\"OK\\\",\\\"NewStateReason\\\":\\\"Threshold Crossed: 1 datapoint (0.4482758620689655) was not greater than or equal to the threshold (1.0).\\\",\\\"StateChangeTime\\\":\\\"2016-02-27T11:21:10.602+0000\\\",\\\"Region\\\":\\\"US - N. Virginia\\\",\\\"OldStateValue\\\":\\\"ALARM\\\",\\\"Trigger\\\":{\\\"MetricName\\\":\\\"UnHealthyHostCount\\\",\\\"Namespace\\\":\\\"AWS/ELB\\\",\\\"Statistic\\\":\\\"AVERAGE\\\",\\\"Unit\\\":null,\\\"Dimensions\\\":[{\\\"name\\\":\\\"LoadBalancerName\\\",\\\"value\\\":\\\"rtb\\\"}],\\\"Period\\\":60,\\\"EvaluationPeriods\\\":5,\\\"ComparisonOperator\\\":\\\"GreaterThanOrEqualToThreshold\\\",\\\"Threshold\\\":1.0}}\",
            \"Timestamp\" : \"2016-02-27T11:21:10.645Z\",
            \"SignatureVersion\" : \"1\",
            \"Signature\" : \"c/em6S4BTVjDaZSb9CY2xaZN0CcBrMpjt1oAVkrRWxgSyeRYle7eZfPKqkOqGfDDAISieK+iNvVWmJwsDAGYhzaXGt4hCTzQ5wzO7/Agxkh6LCYZhqiR39JV7NH78mpLqV0PHgI85Rte05Ou1y8BDy+YgyTn6Cyx2Gyldpa3YhOexQQ+HZLO7zTfdOL9pIU0a+xes1VAgcu0W5h7k3oyMKscJvW3EmteXQu+3psieDDs4tjFd8xsTfxMHkkjYlj/4sN/pkn+3aWW12dX8rUey3TrBtxtiEsuiagh6qiAVDNYI8zpfcn7efzVYoskM2uS7gWJ0jRdvYQuz1C5OSuncQ==\",
            \"SigningCertURL\" : \"https://sns.us-east-1.amazonaws.com/SimpleNotificationService-bb750dd426d95ee9390147a5624348ee.pem\",
            \"UnsubscribeURL\" : \"https://sns.us-east-1.amazonaws.com/?Action=Unsubscribe&SubscriptionArn=arn:aws:sns:us-east-1:097958131044:jabber-all:37f2da3a-76fd-4e5b-ac6a-a01ce32b8ea7\"
        }";

        let result: JsonResult<AlarmDetails> = json.parse();
        assert_eq!(result.is_ok(), true);

        let details = result.unwrap();
        let trigger = details.trigger;

        assert_eq!(details.name, "[RTB-US] UnHealthyHostCount");
        assert_eq!(details.description, "UnHealthyHostCount");
        assert_eq!(details.new_state, AlarmState::Ok);
        assert_eq!(
            details.reason,
            "Threshold Crossed: 1 datapoint (0.4482758620689655) was not greater than or equal to the threshold (1.0)."
        );
        assert_eq!(details.previous_state, AlarmState::Alarm);

        assert_eq!(trigger.metric_name, "UnHealthyHostCount");
        assert_eq!(trigger.namespace, "AWS/ELB");
        assert_eq!(trigger.statistic, "AVERAGE");
        assert_eq!(trigger.period, 60);
        assert_eq!(trigger.nb_periods, 5);
        assert_eq!(trigger.op, "GreaterThanOrEqualToThreshold");
        assert_eq!(trigger.threshold, 1.0);

        assert_eq!(trigger.dimensions[0].name, "LoadBalancerName");
        assert_eq!(trigger.dimensions[0].value, "rtb");

    }

    #[test]
    fn test_notification_fail_if_message_not_a_string() {
        let json = "\
        {\
            \"Type\" : \"Notification\",
            \"MessageId\" : \"e8af1a0c-3b6b-5608-ab09-570f57d6cc65\",
            \"TopicArn\" : \"arn:aws:sns:us-east-1:097958131044:jabber-all\",
            \"Subject\" : \"OK: \\\"[TRACKING US] CPU Utilisation >70% for 5 min\\\" in US - N. Virginia\",
            \"Message\" : 10,
            \"Timestamp\" : \"2014-12-11T09:57:17.756Z\",
            \"SignatureVersion\" : \"1\",
            \"Signature\" : \"nSs8D+hDzheuCMq4K5d3zjbb/OL1oP+UQLyM9Qn6/OADKUEJ8TRTmOXA/e6nzjuIT8mt3/Km59bAhO7GuwtwH/VjmlwyCbpO4LKoAUorNiQI0MCCOSWP7rF08Bm1GKnB6XQpu12l/Eig5CGoke86joJpKb4jaMlhGWsk6TVlQduA/Lu0NhW+3iJGIHtgxbFxjnokH18fItZJDalibASkhrUjw9/Ey31xkiojqM7Um+KO78fvo6kEUiTQpGo5cRGXm20u5FGnrARqmwXyMzSY9VFvy9HPxSqnqp/tHh4iCq5Ij0euhy3+S30dYYwC/1cZfq+1dLF1yY4o/WutvFCfOQ==\",
            \"SigningCertURL\" : \"https://sns.us-east-1.amazonaws.com/SimpleNotificationService-d6d679a1d18e95c2f9ffcf11f4f9e198.pem\",
            \"UnsubscribeURL\" : \"https://sns.us-east-1.amazonaws.com/?Action=Unsubscribe&SubscriptionArn=arn:aws:sns:us-east-1:097958131044:jabber-all:37f2da3a-76fd-4e5b-ac6a-a01ce32b8ea7\"
        }";

        let result: JsonResult<AlarmDetails> = json.parse();
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_notification_fail_if_message_not_json() {
        let json = "\
        {\
            \"Type\" : \"Notification\",
            \"MessageId\" : \"e8af1a0c-3b6b-5608-ab09-570f57d6cc65\",
            \"TopicArn\" : \"arn:aws:sns:us-east-1:097958131044:jabber-all\",
            \"Subject\" : \"OK: \\\"[TRACKING US] CPU Utilisation >70% for 5 min\\\" in US - N. Virginia\",
            \"Message\" : \"\",
            \"Timestamp\" : \"2014-12-11T09:57:17.756Z\",
            \"SignatureVersion\" : \"1\",
            \"Signature\" : \"nSs8D+hDzheuCMq4K5d3zjbb/OL1oP+UQLyM9Qn6/OADKUEJ8TRTmOXA/e6nzjuIT8mt3/Km59bAhO7GuwtwH/VjmlwyCbpO4LKoAUorNiQI0MCCOSWP7rF08Bm1GKnB6XQpu12l/Eig5CGoke86joJpKb4jaMlhGWsk6TVlQduA/Lu0NhW+3iJGIHtgxbFxjnokH18fItZJDalibASkhrUjw9/Ey31xkiojqM7Um+KO78fvo6kEUiTQpGo5cRGXm20u5FGnrARqmwXyMzSY9VFvy9HPxSqnqp/tHh4iCq5Ij0euhy3+S30dYYwC/1cZfq+1dLF1yY4o/WutvFCfOQ==\",
            \"SigningCertURL\" : \"https://sns.us-east-1.amazonaws.com/SimpleNotificationService-d6d679a1d18e95c2f9ffcf11f4f9e198.pem\",
            \"UnsubscribeURL\" : \"https://sns.us-east-1.amazonaws.com/?Action=Unsubscribe&SubscriptionArn=arn:aws:sns:us-east-1:097958131044:jabber-all:37f2da3a-76fd-4e5b-ac6a-a01ce32b8ea7\"
        }";

        let result: JsonResult<AlarmDetails> = json.parse();
        assert_eq!(result.is_ok(), false);
    }
}
