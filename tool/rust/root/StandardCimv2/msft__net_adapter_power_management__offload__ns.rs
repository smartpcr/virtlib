// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterPowerManagement_Offload_NS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterPowerManagement_Offload_NS {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterPowerManagement_Offload,

/// 
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

/// 
    #[serde(rename = "RemoteIPv6Address")]
    pub remote_ipv6_address: Option<String>,

/// 
    #[serde(rename = "SolicitedNodeIPv6Address")]
    pub solicited_node_ipv6_address: Option<String>,

/// 
    #[serde(rename = "TargetIPv6Addresses")]
    pub target_ipv6_addresses: Vec<String>,
}

impl MSFT_NetAdapterPowerManagement_Offload_NS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterPowerManagement_Offload::new(),
            mac_address: None,
            remote_ipv6_address: None,
            solicited_node_ipv6_address: None,
            target_ipv6_addresses: Vec::new(),
        }
    }


    /// Sets the value of MacAddress
    pub fn set_mac_address(&mut self, value: String) {
        self.mac_address = Some(value);
    }

    /// Gets the value of MacAddress
    pub fn get_mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    /// Sets the value of RemoteIPv6Address
    pub fn set_remote_ipv6_address(&mut self, value: String) {
        self.remote_ipv6_address = Some(value);
    }

    /// Gets the value of RemoteIPv6Address
    pub fn get_remote_ipv6_address(&self) -> Option<&String> {
        self.remote_ipv6_address.as_ref()
    }

    /// Sets the value of SolicitedNodeIPv6Address
    pub fn set_solicited_node_ipv6_address(&mut self, value: String) {
        self.solicited_node_ipv6_address = Some(value);
    }

    /// Gets the value of SolicitedNodeIPv6Address
    pub fn get_solicited_node_ipv6_address(&self) -> Option<&String> {
        self.solicited_node_ipv6_address.as_ref()
    }

    /// Sets the value of TargetIPv6Addresses
    pub fn set_target_ipv6_addresses(&mut self, value: Vec<String>) {
        self.target_ipv6_addresses = value;
    }

    /// Gets the value of TargetIPv6Addresses
    pub fn get_target_ipv6_addresses(&self) -> &Vec<String> {
        &self.target_ipv6_addresses
    }
}

