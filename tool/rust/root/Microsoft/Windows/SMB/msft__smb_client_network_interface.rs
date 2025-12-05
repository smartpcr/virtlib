// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbClientNetworkInterface struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbClientNetworkInterface {

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "IpAddresses")]
    pub ip_addresses: Vec<String>,

/// 
    #[serde(rename = "LinkSpeed")]
    pub link_speed: Option<u64>,

/// 
    #[serde(rename = "RdmaCapable")]
    pub rdma_capable: Option<bool>,

/// 
    #[serde(rename = "RssCapable")]
    pub rss_capable: Option<bool>,
}

impl MSFT_SmbClientNetworkInterface {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            friendly_name: None,
            interface_index: None,
            ip_addresses: Vec::new(),
            link_speed: None,
            rdma_capable: None,
            rss_capable: None,
        }
    }


    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of IpAddresses
    pub fn set_ip_addresses(&mut self, value: Vec<String>) {
        self.ip_addresses = value;
    }

    /// Gets the value of IpAddresses
    pub fn get_ip_addresses(&self) -> &Vec<String> {
        &self.ip_addresses
    }

    /// Sets the value of LinkSpeed
    pub fn set_link_speed(&mut self, value: u64) {
        self.link_speed = Some(value);
    }

    /// Gets the value of LinkSpeed
    pub fn get_link_speed(&self) -> Option<&u64> {
        self.link_speed.as_ref()
    }

    /// Sets the value of RdmaCapable
    pub fn set_rdma_capable(&mut self, value: bool) {
        self.rdma_capable = Some(value);
    }

    /// Gets the value of RdmaCapable
    pub fn get_rdma_capable(&self) -> Option<&bool> {
        self.rdma_capable.as_ref()
    }

    /// Sets the value of RssCapable
    pub fn set_rss_capable(&mut self, value: bool) {
        self.rss_capable = Some(value);
    }

    /// Gets the value of RssCapable
    pub fn get_rss_capable(&self) -> Option<&bool> {
        self.rss_capable.as_ref()
    }
}

