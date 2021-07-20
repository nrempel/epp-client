use quick_xml::de::from_str;
use quick_xml::se;
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

use crate::epp::object::EppObject;
use crate::epp::xml::{EppXml, EPP_XML_HEADER};

impl<T: Serialize + DeserializeOwned> EppXml for EppObject<T> {
    type Object = EppObject<T>;

    fn serialize(&self) -> Result<String, Box<dyn Error>> {
        let epp_xml = format!("{}\r\n{}", EPP_XML_HEADER, se::to_string(self)?);

        Ok(epp_xml)
    }

    fn deserialize(epp_xml: &str) -> Result<Self::Object, Box<dyn Error>> {
        match from_str(epp_xml) {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("epp-client Deserialization Error: {}", e).into()),
        }
    }
}
