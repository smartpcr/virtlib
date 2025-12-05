// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheStatus {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "BranchCacheIsEnabled")]
    pub branch_cache_is_enabled: Option<bool>,

/// 
    #[serde(rename = "BranchCacheServiceStartType")]
    pub branch_cache_service_start_type: Option<u32>,

/// 
    #[serde(rename = "BranchCacheServiceStatus")]
    pub branch_cache_service_status: Option<u32>,

/// 
    #[serde(rename = "ClientConfiguration")]
    pub client_configuration: Option<MSFT_NetBranchCacheClientSettingData>,

/// 
    #[serde(rename = "ContentServerConfiguration")]
    pub content_server_configuration: Option<MSFT_NetBranchCacheContentServerSettingData>,

/// 
    #[serde(rename = "DataCache")]
    pub data_cache: Option<MSFT_NetBranchCacheDataCache>,

/// 
    #[serde(rename = "HashCache")]
    pub hash_cache: Option<MSFT_NetBranchCacheHashCache>,

/// 
    #[serde(rename = "HostedCacheServerConfiguration")]
    pub hosted_cache_server_configuration: Option<MSFT_NetBranchCacheHostedCacheServerSettingData>,

/// 
    #[serde(rename = "NetworkConfiguration")]
    pub network_configuration: Option<MSFT_NetBranchCacheNetworkSettingData>,
}

impl MSFT_NetBranchCacheStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            branch_cache_is_enabled: None,
            branch_cache_service_start_type: None,
            branch_cache_service_status: None,
            client_configuration: None,
            content_server_configuration: None,
            data_cache: None,
            hash_cache: None,
            hosted_cache_server_configuration: None,
            network_configuration: None,
        }
    }


    /// Sets the value of BranchCacheIsEnabled
    pub fn set_branch_cache_is_enabled(&mut self, value: bool) {
        self.branch_cache_is_enabled = Some(value);
    }

    /// Gets the value of BranchCacheIsEnabled
    pub fn get_branch_cache_is_enabled(&self) -> Option<&bool> {
        self.branch_cache_is_enabled.as_ref()
    }

    /// Sets the value of BranchCacheServiceStartType
    pub fn set_branch_cache_service_start_type(&mut self, value: u32) {
        self.branch_cache_service_start_type = Some(value);
    }

    /// Gets the value of BranchCacheServiceStartType
    pub fn get_branch_cache_service_start_type(&self) -> Option<&u32> {
        self.branch_cache_service_start_type.as_ref()
    }

    /// Sets the value of BranchCacheServiceStatus
    pub fn set_branch_cache_service_status(&mut self, value: u32) {
        self.branch_cache_service_status = Some(value);
    }

    /// Gets the value of BranchCacheServiceStatus
    pub fn get_branch_cache_service_status(&self) -> Option<&u32> {
        self.branch_cache_service_status.as_ref()
    }

    /// Sets the value of ClientConfiguration
    pub fn set_client_configuration(&mut self, value: MSFT_NetBranchCacheClientSettingData) {
        self.client_configuration = Some(value);
    }

    /// Gets the value of ClientConfiguration
    pub fn get_client_configuration(&self) -> Option<&MSFT_NetBranchCacheClientSettingData> {
        self.client_configuration.as_ref()
    }

    /// Sets the value of ContentServerConfiguration
    pub fn set_content_server_configuration(&mut self, value: MSFT_NetBranchCacheContentServerSettingData) {
        self.content_server_configuration = Some(value);
    }

    /// Gets the value of ContentServerConfiguration
    pub fn get_content_server_configuration(&self) -> Option<&MSFT_NetBranchCacheContentServerSettingData> {
        self.content_server_configuration.as_ref()
    }

    /// Sets the value of DataCache
    pub fn set_data_cache(&mut self, value: MSFT_NetBranchCacheDataCache) {
        self.data_cache = Some(value);
    }

    /// Gets the value of DataCache
    pub fn get_data_cache(&self) -> Option<&MSFT_NetBranchCacheDataCache> {
        self.data_cache.as_ref()
    }

    /// Sets the value of HashCache
    pub fn set_hash_cache(&mut self, value: MSFT_NetBranchCacheHashCache) {
        self.hash_cache = Some(value);
    }

    /// Gets the value of HashCache
    pub fn get_hash_cache(&self) -> Option<&MSFT_NetBranchCacheHashCache> {
        self.hash_cache.as_ref()
    }

    /// Sets the value of HostedCacheServerConfiguration
    pub fn set_hosted_cache_server_configuration(&mut self, value: MSFT_NetBranchCacheHostedCacheServerSettingData) {
        self.hosted_cache_server_configuration = Some(value);
    }

    /// Gets the value of HostedCacheServerConfiguration
    pub fn get_hosted_cache_server_configuration(&self) -> Option<&MSFT_NetBranchCacheHostedCacheServerSettingData> {
        self.hosted_cache_server_configuration.as_ref()
    }

    /// Sets the value of NetworkConfiguration
    pub fn set_network_configuration(&mut self, value: MSFT_NetBranchCacheNetworkSettingData) {
        self.network_configuration = Some(value);
    }

    /// Gets the value of NetworkConfiguration
    pub fn get_network_configuration(&self) -> Option<&MSFT_NetBranchCacheNetworkSettingData> {
        self.network_configuration.as_ref()
    }
}

