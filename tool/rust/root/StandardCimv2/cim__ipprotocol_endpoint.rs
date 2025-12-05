// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_IPProtocolEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_IPProtocolEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "AddressOrigin")]
    pub address_origin: Option<u16>,

/// 
    #[serde(rename = "AddressType")]
    pub address_type: Option<u16>,

/// 
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,

/// 
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,

/// 
    #[serde(rename = "IPVersionSupport")]
    pub ipversion_support: Option<u16>,

/// 
    #[serde(rename = "PrefixLength")]
    pub prefix_length: Option<u8>,

/// 
    #[serde(rename = "SubnetMask")]
    pub subnet_mask: Option<String>,
}

impl CIM_IPProtocolEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            address: None,
            address_origin: None,
            address_type: None,
            ipv4_address: None,
            ipv6_address: None,
            ipversion_support: None,
            prefix_length: None,
            subnet_mask: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of AddressOrigin
    pub fn set_address_origin(&mut self, value: u16) {
        self.address_origin = Some(value);
    }

    /// Gets the value of AddressOrigin
    pub fn get_address_origin(&self) -> Option<&u16> {
        self.address_origin.as_ref()
    }

    /// Sets the value of AddressType
    pub fn set_address_type(&mut self, value: u16) {
        self.address_type = Some(value);
    }

    /// Gets the value of AddressType
    pub fn get_address_type(&self) -> Option<&u16> {
        self.address_type.as_ref()
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

    /// Sets the value of IPVersionSupport
    pub fn set_ipversion_support(&mut self, value: u16) {
        self.ipversion_support = Some(value);
    }

    /// Gets the value of IPVersionSupport
    pub fn get_ipversion_support(&self) -> Option<&u16> {
        self.ipversion_support.as_ref()
    }

    /// Sets the value of PrefixLength
    pub fn set_prefix_length(&mut self, value: u8) {
        self.prefix_length = Some(value);
    }

    /// Gets the value of PrefixLength
    pub fn get_prefix_length(&self) -> Option<&u8> {
        self.prefix_length.as_ref()
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

