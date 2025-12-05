// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_IcsSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_IcsSettings {

/// 
    #[serde(rename = "DhcpEnabled")]
    pub dhcp_enabled: Option<bool>,

/// 
    #[serde(rename = "DnsEnabled")]
    pub dns_enabled: Option<bool>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

impl HNet_IcsSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dhcp_enabled: None,
            dns_enabled: None,
            id: None,
        }
    }


    /// Sets the value of DhcpEnabled
    pub fn set_dhcp_enabled(&mut self, value: bool) {
        self.dhcp_enabled = Some(value);
    }

    /// Gets the value of DhcpEnabled
    pub fn get_dhcp_enabled(&self) -> Option<&bool> {
        self.dhcp_enabled.as_ref()
    }

    /// Sets the value of DnsEnabled
    pub fn set_dns_enabled(&mut self, value: bool) {
        self.dns_enabled = Some(value);
    }

    /// Gets the value of DnsEnabled
    pub fn get_dns_enabled(&self) -> Option<&bool> {
        self.dns_enabled.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }
}

