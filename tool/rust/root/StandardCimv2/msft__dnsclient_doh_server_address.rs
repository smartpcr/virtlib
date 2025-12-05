// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DNSClientDohServerAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DNSClientDohServerAddress {
    #[serde(flatten)]
    pub base: CIM_RemoteServiceAccessPoint,

/// 751
    #[serde(rename = "AllowFallbackToUdp")]
    pub allow_fallback_to_udp: Option<bool>,

/// 752
    #[serde(rename = "AutoUpgrade")]
    pub auto_upgrade: Option<bool>,

/// 750
    #[serde(rename = "DohTemplate")]
    pub doh_template: Option<String>,

/// 749
    #[serde(rename = "ServerAddress")]
    pub server_address: Option<String>,
}

impl MSFT_DNSClientDohServerAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RemoteServiceAccessPoint::new(),
            allow_fallback_to_udp: None,
            auto_upgrade: None,
            doh_template: None,
            server_address: None,
        }
    }


    /// Sets the value of AllowFallbackToUdp
    pub fn set_allow_fallback_to_udp(&mut self, value: bool) {
        self.allow_fallback_to_udp = Some(value);
    }

    /// Gets the value of AllowFallbackToUdp
    pub fn get_allow_fallback_to_udp(&self) -> Option<&bool> {
        self.allow_fallback_to_udp.as_ref()
    }

    /// Sets the value of AutoUpgrade
    pub fn set_auto_upgrade(&mut self, value: bool) {
        self.auto_upgrade = Some(value);
    }

    /// Gets the value of AutoUpgrade
    pub fn get_auto_upgrade(&self) -> Option<&bool> {
        self.auto_upgrade.as_ref()
    }

    /// Sets the value of DohTemplate
    pub fn set_doh_template(&mut self, value: String) {
        self.doh_template = Some(value);
    }

    /// Gets the value of DohTemplate
    pub fn get_doh_template(&self) -> Option<&String> {
        self.doh_template.as_ref()
    }

    /// Sets the value of ServerAddress
    pub fn set_server_address(&mut self, value: String) {
        self.server_address = Some(value);
    }

    /// Gets the value of ServerAddress
    pub fn get_server_address(&self) -> Option<&String> {
        self.server_address.as_ref()
    }
}

