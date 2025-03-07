//! Types for EPP domain renew request

use super::{Period, XMLNS};
use crate::common::{NoExtension, StringValue};
use crate::request::{Command, Transaction};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

impl<'a> Transaction<NoExtension> for DomainRenew<'a> {}

impl<'a> Command for DomainRenew<'a> {
    type Response = DomainRenewResponse;
    const COMMAND: &'static str = "renew";
}

impl<'a> DomainRenew<'a> {
    pub fn new(name: &'a str, current_expiry_date: NaiveDate, years: u16) -> Self {
        let exp_date_str = current_expiry_date.format("%Y-%m-%d").to_string().into();
        Self {
            domain: DomainRenewRequestData {
                xmlns: XMLNS,
                name: name.into(),
                current_expiry_date: exp_date_str,
                period: Period::new(years),
            },
        }
    }
}

// Request

/// Type for data under the domain &lt;renew&gt; tag
#[derive(Serialize, Debug)]
pub struct DomainRenewRequestData<'a> {
    /// XML namespace for domain commands
    #[serde(rename = "xmlns:domain")]
    xmlns: &'a str,
    /// The name of the domain to be renewed
    #[serde(rename = "domain:name")]
    name: StringValue<'a>,
    /// The current expiry date of the domain in 'Y-m-d' format
    #[serde(rename = "domain:curExpDate")]
    current_expiry_date: StringValue<'a>,
    /// The period of renewal
    #[serde(rename = "domain:period")]
    period: Period,
}

#[derive(Serialize, Debug)]
/// Type for EPP XML &lt;renew&gt; command for domains
pub struct DomainRenew<'a> {
    /// The data under the &lt;renew&gt; tag for the domain renewal
    #[serde(rename = "domain:renew")]
    domain: DomainRenewRequestData<'a>,
}

// Response

/// Type that represents the &lt;renData&gt; tag for domain renew response
#[derive(Deserialize, Debug)]
pub struct DomainRenewResponseData {
    /// The name of the domain
    pub name: StringValue<'static>,
    /// The new expiry date after renewal
    #[serde(rename = "exDate")]
    pub expiring_at: Option<StringValue<'static>>,
}

/// Type that represents the &lt;resData&gt; tag for domain renew response
#[derive(Deserialize, Debug)]
pub struct DomainRenewResponse {
    /// Data under the &lt;renData&gt; tag
    #[serde(rename = "renData")]
    pub renew_data: DomainRenewResponseData,
}

#[cfg(test)]
mod tests {
    use super::DomainRenew;
    use crate::common::NoExtension;
    use crate::request::Transaction;
    use crate::tests::{get_xml, CLTRID, SUCCESS_MSG, SVTRID};
    use chrono::NaiveDate;

    #[test]
    fn command() {
        let xml = get_xml("request/domain/renew.xml").unwrap();

        let exp_date = NaiveDate::from_ymd(2022, 7, 23);
        let object = DomainRenew::new("eppdev.com", exp_date, 1);

        let serialized =
            <DomainRenew as Transaction<NoExtension>>::serialize_request(&object, None, CLTRID)
                .unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn response() {
        let xml = get_xml("response/domain/renew.xml").unwrap();
        let object =
            <DomainRenew as Transaction<NoExtension>>::deserialize_response(xml.as_str()).unwrap();

        let result = object.res_data().unwrap();

        assert_eq!(object.result.code, 1000);
        assert_eq!(object.result.message, SUCCESS_MSG.into());
        assert_eq!(result.renew_data.name, "eppdev-1.com".into());
        assert_eq!(
            *result.renew_data.expiring_at.as_ref().unwrap(),
            "2024-07-23T15:31:20.0Z".into()
        );
        assert_eq!(object.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.tr_ids.server_tr_id, SVTRID.into());
    }
}
