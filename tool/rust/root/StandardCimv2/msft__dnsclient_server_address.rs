// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DNSClientServerAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DNSClientServerAddress {
    #[serde(flatten)]
    pub base: CIM_RemoteServiceAccessPoint,

/// 747
    #[serde(rename = "AddressFamily")]
    pub address_family: Option<u16>,

/// 656
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 655
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 746
    #[serde(rename = "ServerAddresses")]
    pub server_addresses: Vec<String>,
}

impl MSFT_DNSClientServerAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RemoteServiceAccessPoint::new(),
            address_family: None,
            interface_alias: None,
            interface_index: None,
            server_addresses: Vec::new(),
        }
    }


    /// Sets the value of AddressFamily
    pub fn set_address_family(&mut self, value: u16) {
        self.address_family = Some(value);
    }

    /// Gets the value of AddressFamily
    pub fn get_address_family(&self) -> Option<&u16> {
        self.address_family.as_ref()
    }

    /// Sets the value of InterfaceAlias
    pub fn set_interface_alias(&mut self, value: String) {
        self.interface_alias = Some(value);
    }

    /// Gets the value of InterfaceAlias
    pub fn get_interface_alias(&self) -> Option<&String> {
        self.interface_alias.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of ServerAddresses
    pub fn set_server_addresses(&mut self, value: Vec<String>) {
        self.server_addresses = value;
    }

    /// Gets the value of ServerAddresses
    pub fn get_server_addresses(&self) -> &Vec<String> {
        &self.server_addresses
    }
}

