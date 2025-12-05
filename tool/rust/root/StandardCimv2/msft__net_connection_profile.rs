// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetConnectionProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetConnectionProfile {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "DomainAuthenticationKind")]
    pub domain_authentication_kind: Option<u32>,

/// 
    #[serde(rename = "InterfaceAlias")]
    pub interface_alias: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "IPv4Connectivity")]
    pub ipv4_connectivity: Option<u32>,

/// 
    #[serde(rename = "IPv6Connectivity")]
    pub ipv6_connectivity: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NetworkCategory")]
    pub network_category: Option<u32>,
}

impl MSFT_NetConnectionProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            domain_authentication_kind: None,
            interface_alias: None,
            interface_index: None,
            ipv4_connectivity: None,
            ipv6_connectivity: None,
            name: None,
            network_category: None,
        }
    }


    /// Sets the value of DomainAuthenticationKind
    pub fn set_domain_authentication_kind(&mut self, value: u32) {
        self.domain_authentication_kind = Some(value);
    }

    /// Gets the value of DomainAuthenticationKind
    pub fn get_domain_authentication_kind(&self) -> Option<&u32> {
        self.domain_authentication_kind.as_ref()
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

    /// Sets the value of IPv4Connectivity
    pub fn set_ipv4_connectivity(&mut self, value: u32) {
        self.ipv4_connectivity = Some(value);
    }

    /// Gets the value of IPv4Connectivity
    pub fn get_ipv4_connectivity(&self) -> Option<&u32> {
        self.ipv4_connectivity.as_ref()
    }

    /// Sets the value of IPv6Connectivity
    pub fn set_ipv6_connectivity(&mut self, value: u32) {
        self.ipv6_connectivity = Some(value);
    }

    /// Gets the value of IPv6Connectivity
    pub fn get_ipv6_connectivity(&self) -> Option<&u32> {
        self.ipv6_connectivity.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NetworkCategory
    pub fn set_network_category(&mut self, value: u32) {
        self.network_category = Some(value);
    }

    /// Gets the value of NetworkCategory
    pub fn get_network_category(&self) -> Option<&u32> {
        self.network_category.as_ref()
    }
}

