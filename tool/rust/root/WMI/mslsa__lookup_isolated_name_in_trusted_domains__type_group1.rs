// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSLSA_LookupIsolatedNameInTrustedDomains_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSLSA_LookupIsolatedNameInTrustedDomains_TypeGroup1 {
    #[serde(flatten)]
    pub base: MSLSA_LookupIsolatedNameInTrustedDomains,

/// Client Network Address
    #[serde(rename = "ClientNetworkAddress")]
    pub client_network_address: Option<String>,

/// Isolated Name
    #[serde(rename = "IsolatedName")]
    pub isolated_name: Option<String>,
}

impl MSLSA_LookupIsolatedNameInTrustedDomains_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSLSA_LookupIsolatedNameInTrustedDomains::new(),
            client_network_address: None,
            isolated_name: None,
        }
    }


    /// Sets the value of ClientNetworkAddress
    pub fn set_client_network_address(&mut self, value: String) {
        self.client_network_address = Some(value);
    }

    /// Gets the value of ClientNetworkAddress
    pub fn get_client_network_address(&self) -> Option<&String> {
        self.client_network_address.as_ref()
    }

    /// Sets the value of IsolatedName
    pub fn set_isolated_name(&mut self, value: String) {
        self.isolated_name = Some(value);
    }

    /// Gets the value of IsolatedName
    pub fn get_isolated_name(&self) -> Option<&String> {
        self.isolated_name.as_ref()
    }
}

