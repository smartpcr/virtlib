// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TargetPortal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TargetPortal {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,

/// 
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,

/// 
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u32>,

/// 
    #[serde(rename = "SubnetMask")]
    pub subnet_mask: Option<String>,
}

impl MSFT_TargetPortal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            ipv4_address: None,
            ipv6_address: None,
            port_number: None,
            subnet_mask: None,
        }
    }


    /// Sets the value of IPv4Address
    pub fn set_ipv4_address(&mut self, value: String) {
        self.ipv4_address = Some(value);
    }

    /// Gets the value of IPv4Address
    pub fn get_ipv4_address(&self) -> Option<&String> {
        self.ipv4_address.as_ref()
    }

    /// Sets the value of IPv6Address
    pub fn set_ipv6_address(&mut self, value: String) {
        self.ipv6_address = Some(value);
    }

    /// Gets the value of IPv6Address
    pub fn get_ipv6_address(&self) -> Option<&String> {
        self.ipv6_address.as_ref()
    }

    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u32) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u32> {
        self.port_number.as_ref()
    }

    /// Sets the value of SubnetMask
    pub fn set_subnet_mask(&mut self, value: String) {
        self.subnet_mask = Some(value);
    }

    /// Gets the value of SubnetMask
    pub fn get_subnet_mask(&self) -> Option<&String> {
        self.subnet_mask.as_ref()
    }
}

