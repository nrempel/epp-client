//! Types for EPP message ack request

use crate::common::NoExtension;
use crate::request::{Command, Transaction};
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for MessageAck {}

impl Command for MessageAck {
    type Response = String;
    const COMMAND: &'static str = "poll";
}

#[derive(Serialize, Deserialize, Debug)]
/// Type for EPP XML &lt;poll&gt; command for message ack
pub struct MessageAck {
    /// The type of operation to perform
    /// The value is "ack" for message acknowledgement
    op: String,
    /// The ID of the message to be acknowledged
    #[serde(rename = "msgID")]
    message_id: String,
}

impl MessageAck {
    pub fn new(message_id: u32) -> Self {
        Self {
            op: "ack".to_string(),
            message_id: message_id.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MessageAck;
    use crate::request::Transaction;
    use crate::tests::{get_xml, SUCCESS_MSG, SVTRID};

    #[test]
    fn message_ack() {
        let xml = get_xml("response/message/ack.xml").unwrap();
        let object = MessageAck::deserialize_response(xml.as_str()).unwrap();

        let msg = object.message_queue().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(msg.count, 4);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
