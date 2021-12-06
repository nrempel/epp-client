//! Types for EPP host update request

use epp_client_macros::*;

use super::XMLNS;
use crate::common::{ElementName, HostAddr, HostStatus, NoExtension, StringValue};
use crate::request::Transaction;
use serde::{Deserialize, Serialize};

impl Transaction<NoExtension> for HostUpdate {
    type Response = ();
    type ExtensionResponse = NoExtension;
}

impl HostUpdate {
    pub fn new(name: &str) -> Self {
        Self {
            host: HostUpdateRequestData {
                xmlns: XMLNS.to_string(),
                name: name.into(),
                add: None,
                remove: None,
                change_info: None,
            },
        }
    }

    /// Sets the data for the &lt;chg&gt; element of the host update
    pub fn info(&mut self, info: HostChangeInfo) {
        self.host.change_info = Some(info);
    }

    /// Sets the data for the &lt;add&gt; element of the host update
    pub fn add(&mut self, add: HostAddRemove) {
        self.host.add = Some(add);
    }

    /// Sets the data for the &lt;rem&gt; element of the host update
    pub fn remove(&mut self, remove: HostAddRemove) {
        self.host.remove = Some(remove);
    }
}

/// Type for data under the &lt;chg&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostChangeInfo {
    /// The new name for the host
    #[serde(rename = "host:name", alias = "name")]
    pub name: StringValue,
}

/// Type for data under the &lt;add&gt; and &lt;rem&gt; tags
#[derive(Serialize, Deserialize, Debug)]
pub struct HostAddRemove {
    /// The IP addresses to be added to or removed from the host
    #[serde(rename = "host:addr", alias = "addr")]
    pub addresses: Option<Vec<HostAddr>>,
    /// The statuses to be added to or removed from the host
    #[serde(rename = "host:status", alias = "status")]
    pub statuses: Option<Vec<HostStatus>>,
}

/// Type for data under the host &lt;update&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct HostUpdateRequestData {
    /// XML namespace for host commands
    #[serde(rename = "xmlns:host", alias = "xmlns")]
    xmlns: String,
    /// The name of the host
    #[serde(rename = "host:name", alias = "name")]
    name: StringValue,
    /// The IP addresses and statuses to be added to the host
    #[serde(rename = "host:add", alias = "add")]
    add: Option<HostAddRemove>,
    /// The IP addresses and statuses to be removed from the host
    #[serde(rename = "host:rem", alias = "rem")]
    remove: Option<HostAddRemove>,
    /// The host details that need to be updated
    #[serde(rename = "host:chg", alias = "chg")]
    change_info: Option<HostChangeInfo>,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "update")]
/// Type for EPP XML &lt;update&gt; command for hosts
pub struct HostUpdate {
    /// The instance holding the data for the host to be updated
    #[serde(rename = "host:update", alias = "update")]
    host: HostUpdateRequestData,
}
