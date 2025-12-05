// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DOCurrentStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DOCurrentStatus {
    #[serde(flatten)]
    pub base: MSFT_DOBaseStatus,

/// 
    #[serde(rename = "CacheServerConnections")]
    pub cache_server_connections: Option<u32>,

/// 
    #[serde(rename = "CacheSizeBytes")]
    pub cache_size_bytes: Option<u64>,

/// 
    #[serde(rename = "CdnConnections")]
    pub cdn_connections: Option<u32>,

/// 
    #[serde(rename = "CpuUsagePct")]
    pub cpu_usage_pct: Option<f32>,

/// 
    #[serde(rename = "DeviceProfile")]
    pub device_profile: Option<u32>,

/// 
    #[serde(rename = "DiskAvailableBytes")]
    pub disk_available_bytes: Option<u64>,

/// 
    #[serde(rename = "DiskTotalBytes")]
    pub disk_total_bytes: Option<u64>,

/// 
    #[serde(rename = "GroupConnections")]
    pub group_connections: Option<u32>,

/// 
    #[serde(rename = "InternetConnections")]
    pub internet_connections: Option<u32>,

/// 
    #[serde(rename = "LanConnections")]
    pub lan_connections: Option<u32>,

/// 
    #[serde(rename = "LinkLocalConnections")]
    pub link_local_connections: Option<u32>,

/// 
    #[serde(rename = "MemUsageKBytes")]
    pub mem_usage_kbytes: Option<u64>,

/// 
    #[serde(rename = "PeerInfoCount")]
    pub peer_info_count: Option<u32>,

/// 
    #[serde(rename = "Swarms")]
    pub swarms: Option<u32>,
}

impl MSFT_DOCurrentStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DOBaseStatus::new(),
            cache_server_connections: None,
            cache_size_bytes: None,
            cdn_connections: None,
            cpu_usage_pct: None,
            device_profile: None,
            disk_available_bytes: None,
            disk_total_bytes: None,
            group_connections: None,
            internet_connections: None,
            lan_connections: None,
            link_local_connections: None,
            mem_usage_kbytes: None,
            peer_info_count: None,
            swarms: None,
        }
    }


    /// Sets the value of CacheServerConnections
    pub fn set_cache_server_connections(&mut self, value: u32) {
        self.cache_server_connections = Some(value);
    }

    /// Gets the value of CacheServerConnections
    pub fn get_cache_server_connections(&self) -> Option<&u32> {
        self.cache_server_connections.as_ref()
    }

    /// Sets the value of CacheSizeBytes
    pub fn set_cache_size_bytes(&mut self, value: u64) {
        self.cache_size_bytes = Some(value);
    }

    /// Gets the value of CacheSizeBytes
    pub fn get_cache_size_bytes(&self) -> Option<&u64> {
        self.cache_size_bytes.as_ref()
    }

    /// Sets the value of CdnConnections
    pub fn set_cdn_connections(&mut self, value: u32) {
        self.cdn_connections = Some(value);
    }

    /// Gets the value of CdnConnections
    pub fn get_cdn_connections(&self) -> Option<&u32> {
        self.cdn_connections.as_ref()
    }

    /// Sets the value of CpuUsagePct
    pub fn set_cpu_usage_pct(&mut self, value: f32) {
        self.cpu_usage_pct = Some(value);
    }

    /// Gets the value of CpuUsagePct
    pub fn get_cpu_usage_pct(&self) -> Option<&f32> {
        self.cpu_usage_pct.as_ref()
    }

    /// Sets the value of DeviceProfile
    pub fn set_device_profile(&mut self, value: u32) {
        self.device_profile = Some(value);
    }

    /// Gets the value of DeviceProfile
    pub fn get_device_profile(&self) -> Option<&u32> {
        self.device_profile.as_ref()
    }

    /// Sets the value of DiskAvailableBytes
    pub fn set_disk_available_bytes(&mut self, value: u64) {
        self.disk_available_bytes = Some(value);
    }

    /// Gets the value of DiskAvailableBytes
    pub fn get_disk_available_bytes(&self) -> Option<&u64> {
        self.disk_available_bytes.as_ref()
    }

    /// Sets the value of DiskTotalBytes
    pub fn set_disk_total_bytes(&mut self, value: u64) {
        self.disk_total_bytes = Some(value);
    }

    /// Gets the value of DiskTotalBytes
    pub fn get_disk_total_bytes(&self) -> Option<&u64> {
        self.disk_total_bytes.as_ref()
    }

    /// Sets the value of GroupConnections
    pub fn set_group_connections(&mut self, value: u32) {
        self.group_connections = Some(value);
    }

    /// Gets the value of GroupConnections
    pub fn get_group_connections(&self) -> Option<&u32> {
        self.group_connections.as_ref()
    }

    /// Sets the value of InternetConnections
    pub fn set_internet_connections(&mut self, value: u32) {
        self.internet_connections = Some(value);
    }

    /// Gets the value of InternetConnections
    pub fn get_internet_connections(&self) -> Option<&u32> {
        self.internet_connections.as_ref()
    }

    /// Sets the value of LanConnections
    pub fn set_lan_connections(&mut self, value: u32) {
        self.lan_connections = Some(value);
    }

    /// Gets the value of LanConnections
    pub fn get_lan_connections(&self) -> Option<&u32> {
        self.lan_connections.as_ref()
    }

    /// Sets the value of LinkLocalConnections
    pub fn set_link_local_connections(&mut self, value: u32) {
        self.link_local_connections = Some(value);
    }

    /// Gets the value of LinkLocalConnections
    pub fn get_link_local_connections(&self) -> Option<&u32> {
        self.link_local_connections.as_ref()
    }

    /// Sets the value of MemUsageKBytes
    pub fn set_mem_usage_kbytes(&mut self, value: u64) {
        self.mem_usage_kbytes = Some(value);
    }

    /// Gets the value of MemUsageKBytes
    pub fn get_mem_usage_kbytes(&self) -> Option<&u64> {
        self.mem_usage_kbytes.as_ref()
    }

    /// Sets the value of PeerInfoCount
    pub fn set_peer_info_count(&mut self, value: u32) {
        self.peer_info_count = Some(value);
    }

    /// Gets the value of PeerInfoCount
    pub fn get_peer_info_count(&self) -> Option<&u32> {
        self.peer_info_count.as_ref()
    }

    /// Sets the value of Swarms
    pub fn set_swarms(&mut self, value: u32) {
        self.swarms = Some(value);
    }

    /// Gets the value of Swarms
    pub fn get_swarms(&self) -> Option<&u32> {
        self.swarms.as_ref()
    }
}

