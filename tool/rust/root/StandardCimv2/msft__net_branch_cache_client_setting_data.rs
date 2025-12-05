// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheClientSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheClientSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCacheSettingData,

/// 
    #[serde(rename = "CurrentClientMode")]
    pub current_client_mode: Option<u32>,

/// 
    #[serde(rename = "DistributedCachingIsEnabled")]
    pub distributed_caching_is_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheDiscoveryEnabled")]
    pub hosted_cache_discovery_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheServerList")]
    pub hosted_cache_server_list: Vec<String>,

/// 
    #[serde(rename = "HostedCacheVersion")]
    pub hosted_cache_version: Option<u32>,

/// 
    #[serde(rename = "MinimumSmbLatencyInMilliseconds")]
    pub minimum_smb_latency_in_milliseconds: Option<u32>,

/// 
    #[serde(rename = "PreferredContentInformationVersion")]
    pub preferred_content_information_version: Option<u32>,

/// 
    #[serde(rename = "ServeDistributedCachingPeersOnBatteryPower")]
    pub serve_distributed_caching_peers_on_battery_power: Option<bool>,
}

impl MSFT_NetBranchCacheClientSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCacheSettingData::new(),
            current_client_mode: None,
            distributed_caching_is_enabled: None,
            hosted_cache_discovery_enabled: None,
            hosted_cache_server_list: Vec::new(),
            hosted_cache_version: None,
            minimum_smb_latency_in_milliseconds: None,
            preferred_content_information_version: None,
            serve_distributed_caching_peers_on_battery_power: None,
        }
    }


    /// Sets the value of CurrentClientMode
    pub fn set_current_client_mode(&mut self, value: u32) {
        self.current_client_mode = Some(value);
    }

    /// Gets the value of CurrentClientMode
    pub fn get_current_client_mode(&self) -> Option<&u32> {
        self.current_client_mode.as_ref()
    }

    /// Sets the value of DistributedCachingIsEnabled
    pub fn set_distributed_caching_is_enabled(&mut self, value: bool) {
        self.distributed_caching_is_enabled = Some(value);
    }

    /// Gets the value of DistributedCachingIsEnabled
    pub fn get_distributed_caching_is_enabled(&self) -> Option<&bool> {
        self.distributed_caching_is_enabled.as_ref()
    }

    /// Sets the value of HostedCacheDiscoveryEnabled
    pub fn set_hosted_cache_discovery_enabled(&mut self, value: bool) {
        self.hosted_cache_discovery_enabled = Some(value);
    }

    /// Gets the value of HostedCacheDiscoveryEnabled
    pub fn get_hosted_cache_discovery_enabled(&self) -> Option<&bool> {
        self.hosted_cache_discovery_enabled.as_ref()
    }

    /// Sets the value of HostedCacheServerList
    pub fn set_hosted_cache_server_list(&mut self, value: Vec<String>) {
        self.hosted_cache_server_list = value;
    }

    /// Gets the value of HostedCacheServerList
    pub fn get_hosted_cache_server_list(&self) -> &Vec<String> {
        &self.hosted_cache_server_list
    }

    /// Sets the value of HostedCacheVersion
    pub fn set_hosted_cache_version(&mut self, value: u32) {
        self.hosted_cache_version = Some(value);
    }

    /// Gets the value of HostedCacheVersion
    pub fn get_hosted_cache_version(&self) -> Option<&u32> {
        self.hosted_cache_version.as_ref()
    }

    /// Sets the value of MinimumSmbLatencyInMilliseconds
    pub fn set_minimum_smb_latency_in_milliseconds(&mut self, value: u32) {
        self.minimum_smb_latency_in_milliseconds = Some(value);
    }

    /// Gets the value of MinimumSmbLatencyInMilliseconds
    pub fn get_minimum_smb_latency_in_milliseconds(&self) -> Option<&u32> {
        self.minimum_smb_latency_in_milliseconds.as_ref()
    }

    /// Sets the value of PreferredContentInformationVersion
    pub fn set_preferred_content_information_version(&mut self, value: u32) {
        self.preferred_content_information_version = Some(value);
    }

    /// Gets the value of PreferredContentInformationVersion
    pub fn get_preferred_content_information_version(&self) -> Option<&u32> {
        self.preferred_content_information_version.as_ref()
    }

    /// Sets the value of ServeDistributedCachingPeersOnBatteryPower
    pub fn set_serve_distributed_caching_peers_on_battery_power(&mut self, value: bool) {
        self.serve_distributed_caching_peers_on_battery_power = Some(value);
    }

    /// Gets the value of ServeDistributedCachingPeersOnBatteryPower
    pub fn get_serve_distributed_caching_peers_on_battery_power(&self) -> Option<&bool> {
        self.serve_distributed_caching_peers_on_battery_power.as_ref()
    }
}

