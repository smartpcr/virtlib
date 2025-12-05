// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnectionTriggerTrustedNetwork struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnectionTriggerTrustedNetwork {

/// 
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "DnsSuffix")]
    pub dns_suffix: Vec<String>,
}

impl VpnConnectionTriggerTrustedNetwork {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_name: None,
            dns_suffix: Vec::new(),
        }
    }


    /// Sets the value of ConnectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of ConnectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }

    /// Sets the value of DnsSuffix
    pub fn set_dns_suffix(&mut self, value: Vec<String>) {
        self.dns_suffix = value;
    }

    /// Gets the value of DnsSuffix
    pub fn get_dns_suffix(&self) -> &Vec<String> {
        &self.dns_suffix
    }
}

