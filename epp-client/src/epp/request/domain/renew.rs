//! Types for EPP domain renew request

use epp_client_macros::*;

use crate::epp::object::data::Period;
use crate::epp::object::{ElementName, EppObject, StringValue, StringValueTrait};
use crate::epp::request::Command;
use crate::epp::xml::EPP_DOMAIN_XMLNS;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for domain &lt;renew&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use chrono::NaiveDate;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppDomainRenew, EppDomainRenewResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create a date object to set the current expiry date
///     let exp_date = NaiveDate::from_ymd(2022, 7, 27);
///
///     // Create an EppDomainRenew instance
///     let domain_renew = EppDomainRenew::new("eppdev-100.com", exp_date, 1, generate_client_tr_id(&client).as_str());
///
///     // send it to the registry and receive a response of type EppDomainRenewResponse
///     let response = client.transact::<_, EppDomainRenewResponse>(&domain_renew).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```

pub type EppDomainRenew = EppObject<Command<DomainRenew>>;

/// Type for data under the domain &lt;renew&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct DomainRenewData {
    /// XML namespace for domain commands
    pub xmlns: String,
    /// The name of the domain to be renewed
    pub name: StringValue,
    /// The current expiry date of the domain in 'Y-m-d' format
    #[serde(rename = "curExpDate")]
    pub current_expiry_date: StringValue,
    /// The period of renewal
    pub period: Period,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "renew")]
/// Type for EPP XML &lt;renew&gt; command for domains
pub struct DomainRenew {
    /// The data under the &lt;renew&gt; tag for the domain renewal
    #[serde(rename = "renew")]
    pub domain: DomainRenewData,
}

impl EppDomainRenew {
    /// Creates a new EppObject for domain renew corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(
        name: &str,
        current_expiry_date: NaiveDate,
        years: u16,
        client_tr_id: &str,
    ) -> EppDomainRenew {
        let exp_date_str = current_expiry_date
            .format("%Y-%m-%d")
            .to_string()
            .to_string_value();

        EppObject::build(Command::<DomainRenew>::new(
            DomainRenew {
                domain: DomainRenewData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.to_string_value(),
                    current_expiry_date: exp_date_str,
                    period: Period::new(years),
                },
            },
            client_tr_id,
        ))
    }

    pub fn set_period(&mut self, period: Period) {
        self.data.command.domain.period = period;
    }
}
