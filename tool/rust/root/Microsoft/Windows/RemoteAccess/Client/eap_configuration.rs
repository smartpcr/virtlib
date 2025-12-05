// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// EapConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EapConfiguration {

/// 
    #[serde(rename = "EapConfigXmlStream")]
    pub eap_config_xml_stream: Option<String>,
}

impl EapConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            eap_config_xml_stream: None,
        }
    }


    /// Sets the value of EapConfigXmlStream
    pub fn set_eap_config_xml_stream(&mut self, value: String) {
        self.eap_config_xml_stream = Some(value);
    }

    /// Gets the value of EapConfigXmlStream
    pub fn get_eap_config_xml_stream(&self) -> Option<&String> {
        self.eap_config_xml_stream.as_ref()
    }
}

