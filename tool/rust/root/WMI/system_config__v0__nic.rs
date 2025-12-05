// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V0_NIC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V0_NIC {
    #[serde(flatten)]
    pub base: SystemConfig_V0,

/// 
    #[serde(rename = "Data")]
    pub data: Option<u32>,

/// 
    #[serde(rename = "DhcpServer")]
    pub dhcp_server: Option<i32>,

/// 
    #[serde(rename = "DnsServer1")]
    pub dns_server1: Option<i32>,

/// 
    #[serde(rename = "DnsServer2")]
    pub dns_server2: Option<i32>,

/// 
    #[serde(rename = "DnsServer3")]
    pub dns_server3: Option<i32>,

/// 
    #[serde(rename = "DnsServer4")]
    pub dns_server4: Option<i32>,

/// 
    #[serde(rename = "Gateway")]
    pub gateway: Option<i32>,

/// 
    #[serde(rename = "Index")]
    pub index: Option<u32>,

/// 
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<i32>,

/// 
    #[serde(rename = "NICName")]
    pub nicname: Vec<char>,

/// 
    #[serde(rename = "PhysicalAddr")]
    pub physical_addr: Vec<char>,

/// 
    #[serde(rename = "PhysicalAddrLen")]
    pub physical_addr_len: Option<u32>,

/// 
    #[serde(rename = "PrimaryWinsServer")]
    pub primary_wins_server: Option<i32>,

/// 
    #[serde(rename = "SecondaryWinsServer")]
    pub secondary_wins_server: Option<i32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u32>,

/// 
    #[serde(rename = "SubnetMask")]
    pub subnet_mask: Option<i32>,
}

impl SystemConfig_V0_NIC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V0::new(),
            data: None,
            dhcp_server: None,
            dns_server1: None,
            dns_server2: None,
            dns_server3: None,
            dns_server4: None,
            gateway: None,
            index: None,
            ip_address: None,
            nicname: Vec::new(),
            physical_addr: Vec::new(),
            physical_addr_len: None,
            primary_wins_server: None,
            secondary_wins_server: None,
            size: None,
            subnet_mask: None,
        }
    }


    /// Sets the value of Data
    pub fn set_data(&mut self, value: u32) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&u32> {
        self.data.as_ref()
    }

    /// Sets the value of DhcpServer
    pub fn set_dhcp_server(&mut self, value: i32) {
        self.dhcp_server = Some(value);
    }

    /// Gets the value of DhcpServer
    pub fn get_dhcp_server(&self) -> Option<&i32> {
        self.dhcp_server.as_ref()
    }

    /// Sets the value of DnsServer1
    pub fn set_dns_server1(&mut self, value: i32) {
        self.dns_server1 = Some(value);
    }

    /// Gets the value of DnsServer1
    pub fn get_dns_server1(&self) -> Option<&i32> {
        self.dns_server1.as_ref()
    }

    /// Sets the value of DnsServer2
    pub fn set_dns_server2(&mut self, value: i32) {
        self.dns_server2 = Some(value);
    }

    /// Gets the value of DnsServer2
    pub fn get_dns_server2(&self) -> Option<&i32> {
        self.dns_server2.as_ref()
    }

    /// Sets the value of DnsServer3
    pub fn set_dns_server3(&mut self, value: i32) {
        self.dns_server3 = Some(value);
    }

    /// Gets the value of DnsServer3
    pub fn get_dns_server3(&self) -> Option<&i32> {
        self.dns_server3.as_ref()
    }

    /// Sets the value of DnsServer4
    pub fn set_dns_server4(&mut self, value: i32) {
        self.dns_server4 = Some(value);
    }

    /// Gets the value of DnsServer4
    pub fn get_dns_server4(&self) -> Option<&i32> {
        self.dns_server4.as_ref()
    }

    /// Sets the value of Gateway
    pub fn set_gateway(&mut self, value: i32) {
        self.gateway = Some(value);
    }

    /// Gets the value of Gateway
    pub fn get_gateway(&self) -> Option<&i32> {
        self.gateway.as_ref()
    }

    /// Sets the value of Index
    pub fn set_index(&mut self, value: u32) {
        self.index = Some(value);
    }

    /// Gets the value of Index
    pub fn get_index(&self) -> Option<&u32> {
        self.index.as_ref()
    }

    /// Sets the value of IpAddress
    pub fn set_ip_address(&mut self, value: i32) {
        self.ip_address = Some(value);
    }

    /// Gets the value of IpAddress
    pub fn get_ip_address(&self) -> Option<&i32> {
        self.ip_address.as_ref()
    }

    /// Sets the value of NICName
    pub fn set_nicname(&mut self, value: Vec<char>) {
        self.nicname = value;
    }

    /// Gets the value of NICName
    pub fn get_nicname(&self) -> &Vec<char> {
        &self.nicname
    }

    /// Sets the value of PhysicalAddr
    pub fn set_physical_addr(&mut self, value: Vec<char>) {
        self.physical_addr = value;
    }

    /// Gets the value of PhysicalAddr
    pub fn get_physical_addr(&self) -> &Vec<char> {
        &self.physical_addr
    }

    /// Sets the value of PhysicalAddrLen
    pub fn set_physical_addr_len(&mut self, value: u32) {
        self.physical_addr_len = Some(value);
    }

    /// Gets the value of PhysicalAddrLen
    pub fn get_physical_addr_len(&self) -> Option<&u32> {
        self.physical_addr_len.as_ref()
    }

    /// Sets the value of PrimaryWinsServer
    pub fn set_primary_wins_server(&mut self, value: i32) {
        self.primary_wins_server = Some(value);
    }

    /// Gets the value of PrimaryWinsServer
    pub fn get_primary_wins_server(&self) -> Option<&i32> {
        self.primary_wins_server.as_ref()
    }

    /// Sets the value of SecondaryWinsServer
    pub fn set_secondary_wins_server(&mut self, value: i32) {
        self.secondary_wins_server = Some(value);
    }

    /// Gets the value of SecondaryWinsServer
    pub fn get_secondary_wins_server(&self) -> Option<&i32> {
        self.secondary_wins_server.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u32) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u32> {
        self.size.as_ref()
    }

    /// Sets the value of SubnetMask
    pub fn set_subnet_mask(&mut self, value: i32) {
        self.subnet_mask = Some(value);
    }

    /// Gets the value of SubnetMask
    pub fn get_subnet_mask(&self) -> Option<&i32> {
        self.subnet_mask.as_ref()
    }
}

