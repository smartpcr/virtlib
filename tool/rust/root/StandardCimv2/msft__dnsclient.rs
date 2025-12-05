// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DNSClient struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DNSClient {
    #[serde(flatten)]
    pub base: CIM_DNSProtocolEndpoint,

/// 657
    #[serde(rename = "ConnectionSpecificSuffix")]
    pub connection_specific_suffix: Option<String>,

/// 658
    #[serde(rename = "ConnectionSpecificSuffixSearchList")]
    pub connection_specific_suffix_search_list: Vec<String>,

/// 656
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 655
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 659
    #[serde(rename = "RegisterThisConnectionsAddress")]
    pub register_this_connections_address: Option<bool>,

/// 660
    #[serde(rename = "UseSuffixWhenRegistering")]
    pub use_suffix_when_registering: Option<bool>,
}

impl MSFT_DNSClient {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DNSProtocolEndpoint::new(),
            connection_specific_suffix: None,
            connection_specific_suffix_search_list: Vec::new(),
            interface_alias: None,
            interface_index: None,
            register_this_connections_address: None,
            use_suffix_when_registering: None,
        }
    }


    /// Sets the value of ConnectionSpecificSuffix
    pub fn set_connection_specific_suffix(&mut self, value: String) {
        self.connection_specific_suffix = Some(value);
    }

    /// Gets the value of ConnectionSpecificSuffix
    pub fn get_connection_specific_suffix(&self) -> Option<&String> {
        self.connection_specific_suffix.as_ref()
    }

    /// Sets the value of ConnectionSpecificSuffixSearchList
    pub fn set_connection_specific_suffix_search_list(&mut self, value: Vec<String>) {
        self.connection_specific_suffix_search_list = value;
    }

    /// Gets the value of ConnectionSpecificSuffixSearchList
    pub fn get_connection_specific_suffix_search_list(&self) -> &Vec<String> {
        &self.connection_specific_suffix_search_list
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

    /// Sets the value of RegisterThisConnectionsAddress
    pub fn set_register_this_connections_address(&mut self, value: bool) {
        self.register_this_connections_address = Some(value);
    }

    /// Gets the value of RegisterThisConnectionsAddress
    pub fn get_register_this_connections_address(&self) -> Option<&bool> {
        self.register_this_connections_address.as_ref()
    }

    /// Sets the value of UseSuffixWhenRegistering
    pub fn set_use_suffix_when_registering(&mut self, value: bool) {
        self.use_suffix_when_registering = Some(value);
    }

    /// Gets the value of UseSuffixWhenRegistering
    pub fn get_use_suffix_when_registering(&self) -> Option<&bool> {
        self.use_suffix_when_registering.as_ref()
    }

/// 661

    /// * `return_value` -  (u32)
    pub fn register(&self) -> Result<(), WmiError> {
        self.invoke_method("Register", &[])

    }

}

