// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DNSProtocolEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DNSProtocolEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// 650
    #[serde(rename = "DHCPOptionsToUse")]
    pub dhcpoptions_to_use: Vec<DNSProtocolEndpoint_DHCPOptionsToUse>,

/// 649
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
}

impl CIM_DNSProtocolEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            dhcpoptions_to_use: Vec::new(),
            hostname: None,
        }
    }


    /// Sets the value of DHCPOptionsToUse
    pub fn set_dhcpoptions_to_use(&mut self, value: Vec<DNSProtocolEndpoint_DHCPOptionsToUse>) {
        self.dhcpoptions_to_use = value;
    }

    /// Gets the value of DHCPOptionsToUse
    pub fn get_dhcpoptions_to_use(&self) -> &Vec<DNSProtocolEndpoint_DHCPOptionsToUse> {
        &self.dhcpoptions_to_use
    }

    /// Sets the value of Hostname
    pub fn set_hostname(&mut self, value: String) {
        self.hostname = Some(value);
    }

    /// Gets the value of Hostname
    pub fn get_hostname(&self) -> Option<&String> {
        self.hostname.as_ref()
    }
}

