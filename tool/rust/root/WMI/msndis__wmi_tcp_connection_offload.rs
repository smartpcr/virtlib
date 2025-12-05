// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiTcpConnectionOffload struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiTcpConnectionOffload {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Encapsulation")]
    pub encapsulation: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "SupportIp4")]
    pub support_ip4: Option<u32>,

/// 
    #[serde(rename = "SupportIp6")]
    pub support_ip6: Option<u32>,

/// 
    #[serde(rename = "SupportIp6ExtensionHeaders")]
    pub support_ip6_extension_headers: Option<u32>,

/// 
    #[serde(rename = "SupportSack")]
    pub support_sack: Option<u32>,

/// 
    #[serde(rename = "TcpConnectionOffloadCapacity")]
    pub tcp_connection_offload_capacity: Option<u32>,
}

impl MSNdis_WmiTcpConnectionOffload {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            encapsulation: None,
            flags: None,
            header: None,
            support_ip4: None,
            support_ip6: None,
            support_ip6_extension_headers: None,
            support_sack: None,
            tcp_connection_offload_capacity: None,
        }
    }


    /// Sets the value of Encapsulation
    pub fn set_encapsulation(&mut self, value: u32) {
        self.encapsulation = Some(value);
    }

    /// Gets the value of Encapsulation
    pub fn get_encapsulation(&self) -> Option<&u32> {
        self.encapsulation.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of SupportIp4
    pub fn set_support_ip4(&mut self, value: u32) {
        self.support_ip4 = Some(value);
    }

    /// Gets the value of SupportIp4
    pub fn get_support_ip4(&self) -> Option<&u32> {
        self.support_ip4.as_ref()
    }

    /// Sets the value of SupportIp6
    pub fn set_support_ip6(&mut self, value: u32) {
        self.support_ip6 = Some(value);
    }

    /// Gets the value of SupportIp6
    pub fn get_support_ip6(&self) -> Option<&u32> {
        self.support_ip6.as_ref()
    }

    /// Sets the value of SupportIp6ExtensionHeaders
    pub fn set_support_ip6_extension_headers(&mut self, value: u32) {
        self.support_ip6_extension_headers = Some(value);
    }

    /// Gets the value of SupportIp6ExtensionHeaders
    pub fn get_support_ip6_extension_headers(&self) -> Option<&u32> {
        self.support_ip6_extension_headers.as_ref()
    }

    /// Sets the value of SupportSack
    pub fn set_support_sack(&mut self, value: u32) {
        self.support_sack = Some(value);
    }

    /// Gets the value of SupportSack
    pub fn get_support_sack(&self) -> Option<&u32> {
        self.support_sack.as_ref()
    }

    /// Sets the value of TcpConnectionOffloadCapacity
    pub fn set_tcp_connection_offload_capacity(&mut self, value: u32) {
        self.tcp_connection_offload_capacity = Some(value);
    }

    /// Gets the value of TcpConnectionOffloadCapacity
    pub fn get_tcp_connection_offload_capacity(&self) -> Option<&u32> {
        self.tcp_connection_offload_capacity.as_ref()
    }
}

