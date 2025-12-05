// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerNetworkAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerNetworkAdapter {

/// 
    #[serde(rename = "Addresses")]
    pub addresses: Vec<String>,

/// 
    #[serde(rename = "ConnectionStatus")]
    pub connection_status: Option<u16>,

/// 
    #[serde(rename = "DHCPEnabled")]
    pub dhcpenabled: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_ServerNetworkAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            addresses: Vec::new(),
            connection_status: None,
            dhcpenabled: None,
            name: None,
        }
    }


    /// Sets the value of Addresses
    pub fn set_addresses(&mut self, value: Vec<String>) {
        self.addresses = value;
    }

    /// Gets the value of Addresses
    pub fn get_addresses(&self) -> &Vec<String> {
        &self.addresses
    }

    /// Sets the value of ConnectionStatus
    pub fn set_connection_status(&mut self, value: u16) {
        self.connection_status = Some(value);
    }

    /// Gets the value of ConnectionStatus
    pub fn get_connection_status(&self) -> Option<&u16> {
        self.connection_status.as_ref()
    }

    /// Sets the value of DHCPEnabled
    pub fn set_dhcpenabled(&mut self, value: bool) {
        self.dhcpenabled = Some(value);
    }

    /// Gets the value of DHCPEnabled
    pub fn get_dhcpenabled(&self) -> Option<&bool> {
        self.dhcpenabled.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

