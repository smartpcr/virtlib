// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnServerAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnServerAddress {

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "ServerAddress")]
    pub server_address: Option<String>,
}

impl VpnServerAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            friendly_name: None,
            server_address: None,
        }
    }


    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
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

