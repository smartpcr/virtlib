// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_NIC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_NIC {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "DnsServerAddresses")]
    pub dns_server_addresses: Option<String>,

/// 
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Option<String>,

/// 
    #[serde(rename = "Ipv4Index")]
    pub ipv4_index: Option<u32>,

/// 
    #[serde(rename = "Ipv6Index")]
    pub ipv6_index: Option<u32>,

/// 
    #[serde(rename = "NICDescription")]
    pub nicdescription: Option<String>,

/// 
    #[serde(rename = "PhysicalAddr")]
    pub physical_addr: Option<u64>,

/// 
    #[serde(rename = "PhysicalAddrLen")]
    pub physical_addr_len: Option<u32>,
}

impl SystemConfig_V2_NIC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            dns_server_addresses: None,
            ip_addresses: None,
            ipv4_index: None,
            ipv6_index: None,
            nicdescription: None,
            physical_addr: None,
            physical_addr_len: None,
        }
    }


    /// Sets the value of DnsServerAddresses
    pub fn set_dns_server_addresses(&mut self, value: String) {
        self.dns_server_addresses = Some(value);
    }

    /// Gets the value of DnsServerAddresses
    pub fn get_dns_server_addresses(&self) -> Option<&String> {
        self.dns_server_addresses.as_ref()
    }

    /// Sets the value of IpAddresses
    pub fn set_ip_addresses(&mut self, value: String) {
        self.ip_addresses = Some(value);
    }

    /// Gets the value of IpAddresses
    pub fn get_ip_addresses(&self) -> Option<&String> {
        self.ip_addresses.as_ref()
    }

    /// Sets the value of Ipv4Index
    pub fn set_ipv4_index(&mut self, value: u32) {
        self.ipv4_index = Some(value);
    }

    /// Gets the value of Ipv4Index
    pub fn get_ipv4_index(&self) -> Option<&u32> {
        self.ipv4_index.as_ref()
    }

    /// Sets the value of Ipv6Index
    pub fn set_ipv6_index(&mut self, value: u32) {
        self.ipv6_index = Some(value);
    }

    /// Gets the value of Ipv6Index
    pub fn get_ipv6_index(&self) -> Option<&u32> {
        self.ipv6_index.as_ref()
    }

    /// Sets the value of NICDescription
    pub fn set_nicdescription(&mut self, value: String) {
        self.nicdescription = Some(value);
    }

    /// Gets the value of NICDescription
    pub fn get_nicdescription(&self) -> Option<&String> {
        self.nicdescription.as_ref()
    }

    /// Sets the value of PhysicalAddr
    pub fn set_physical_addr(&mut self, value: u64) {
        self.physical_addr = Some(value);
    }

    /// Gets the value of PhysicalAddr
    pub fn get_physical_addr(&self) -> Option<&u64> {
        self.physical_addr.as_ref()
    }

    /// Sets the value of PhysicalAddrLen
    pub fn set_physical_addr_len(&mut self, value: u32) {
        self.physical_addr_len = Some(value);
    }

    /// Gets the value of PhysicalAddrLen
    pub fn get_physical_addr_len(&self) -> Option<&u32> {
        self.physical_addr_len.as_ref()
    }
}

