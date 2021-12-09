//! Types for EPP responses

use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Debug;

use crate::common::StringValue;
use crate::xml::EppXml;

/// Type corresponding to the <undef> tag an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct Undef;

/// Type corresponding to the <value> tag under <extValue> in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct ResultValue {
    /// The XML namespace for the <value> tag
    #[serde(rename = "xmlns:epp")]
    xmlns: String,
    /// The <undef> element
    pub undef: Undef,
}

/// Type corresponding to the <extValue> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct ExtValue {
    /// Data under the <value> tag
    pub value: ResultValue,
    /// Data under the <reason> tag
    pub reason: StringValue,
}

/// Type corresponding to the <result> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct EppResult {
    /// The result code
    pub code: u16,
    /// The result message
    #[serde(rename = "msg")]
    pub message: StringValue,
    /// Data under the <extValue> tag
    #[serde(rename = "extValue")]
    pub ext_value: Option<ExtValue>,
}

/// Type corresponding to the <trID> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct ResponseTRID {
    /// The client TRID
    #[serde(rename = "clTRID")]
    pub client_tr_id: Option<StringValue>,
    /// The server TRID
    #[serde(rename = "svTRID")]
    pub server_tr_id: StringValue,
}

/// Type corresponding to the <msgQ> tag in an EPP response XML
#[derive(Deserialize, Debug, PartialEq)]
pub struct MessageQueue {
    /// The message count
    pub count: u32,
    /// The message ID
    pub id: String,
    /// The message date
    #[serde(rename = "qDate")]
    pub date: Option<StringValue>,
    /// The message text
    #[serde(rename = "msg")]
    pub message: Option<StringValue>,
}

#[derive(Deserialize, Debug, PartialEq)]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// containing an &lt;extension&gt; tag
pub struct Response<D, E> {
    /// Data under the <result> tag
    pub result: EppResult,
    /// Data under the <msgQ> tag
    #[serde(rename = "msgQ")]
    pub message_queue: Option<MessageQueue>,
    #[serde(rename = "resData")]
    /// Data under the &lt;resData&gt; tag
    pub res_data: Option<D>,
    /// Data under the &lt;extension&gt; tag
    pub extension: Option<E>,
    /// Data under the <trID> tag
    #[serde(rename = "trID")]
    pub tr_ids: ResponseTRID,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename = "epp")]
pub struct ResponseDocument<D, E> {
    #[serde(rename = "response")]
    pub data: Response<D, E>,
}

impl<D: DeserializeOwned, E: DeserializeOwned> EppXml for ResponseDocument<D, E> {}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "epp")]
pub struct ResultDocument {
    #[serde(rename = "response")]
    pub data: ResponseStatus,
}

impl EppXml for ResultDocument {}

#[derive(Deserialize, Debug, PartialEq)]
/// Type corresponding to the &lt;response&gt; tag in an EPP response XML
/// without <msgQ> or &lt;resData&gt; sections. Generally used for error handling
pub struct ResponseStatus {
    /// Data under the <result> tag
    pub result: EppResult,
    #[serde(rename = "trID")]
    /// Data under the <trID> tag
    pub tr_ids: ResponseTRID,
}

impl<T, E> Response<T, E> {
    /// Returns the data under the corresponding &lt;resData&gt; from the EPP XML
    pub fn res_data(&self) -> Option<&T> {
        match &self.res_data {
            Some(res_data) => Some(res_data),
            None => None,
        }
    }
    /// Returns the data under the corresponding <msgQ> from the EPP XML
    pub fn message_queue(&self) -> Option<&MessageQueue> {
        match &self.message_queue {
            Some(queue) => Some(queue),
            None => None,
        }
    }
}
