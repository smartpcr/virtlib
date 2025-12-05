// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesMachineConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesMachineConfiguration {

/// 
    #[serde(rename = "BackgroundSyncEnabled")]
    pub background_sync_enabled: Option<bool>,

/// 
    #[serde(rename = "BackgroundSyncParams")]
    pub background_sync_params: Option<Win32_OfflineFilesBackgroundSync>,

/// 
    #[serde(rename = "DiskSpaceLimitEnabled")]
    pub disk_space_limit_enabled: Option<bool>,

/// 
    #[serde(rename = "DiskSpaceLimitParams")]
    pub disk_space_limit_params: Option<Win32_OfflineFilesDiskSpaceLimit>,

/// 
    #[serde(rename = "EconomicalAdminPinningEnabled")]
    pub economical_admin_pinning_enabled: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "ExcludedFileTypes")]
    pub excluded_file_types: Vec<String>,

/// 
    #[serde(rename = "IsConfiguredByWMI")]
    pub is_configured_by_wmi: Option<bool>,

/// 
    #[serde(rename = "MakeAvailableOfflineButtonRemoved")]
    pub make_available_offline_button_removed: Option<bool>,

/// 
    #[serde(rename = "OfflineFilesCacheEncrypted")]
    pub offline_files_cache_encrypted: Option<bool>,

/// 
    #[serde(rename = "SlowLinkEnabled")]
    pub slow_link_enabled: Option<bool>,

/// 
    #[serde(rename = "SlowLinkParams")]
    pub slow_link_params: Vec<String>,

/// 
    #[serde(rename = "SyncOnCostedNetworkEnabled")]
    pub sync_on_costed_network_enabled: Option<bool>,

/// 
    #[serde(rename = "TransparentCachingLatencyThreshold")]
    pub transparent_caching_latency_threshold: Option<u32>,

/// 
    #[serde(rename = "WorkOfflineButtonRemoved")]
    pub work_offline_button_removed: Option<bool>,
}

impl Win32_OfflineFilesMachineConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            background_sync_enabled: None,
            background_sync_params: None,
            disk_space_limit_enabled: None,
            disk_space_limit_params: None,
            economical_admin_pinning_enabled: None,
            enabled: None,
            excluded_file_types: Vec::new(),
            is_configured_by_wmi: None,
            make_available_offline_button_removed: None,
            offline_files_cache_encrypted: None,
            slow_link_enabled: None,
            slow_link_params: Vec::new(),
            sync_on_costed_network_enabled: None,
            transparent_caching_latency_threshold: None,
            work_offline_button_removed: None,
        }
    }


    /// Sets the value of BackgroundSyncEnabled
    pub fn set_background_sync_enabled(&mut self, value: bool) {
        self.background_sync_enabled = Some(value);
    }

    /// Gets the value of BackgroundSyncEnabled
    pub fn get_background_sync_enabled(&self) -> Option<&bool> {
        self.background_sync_enabled.as_ref()
    }

    /// Sets the value of BackgroundSyncParams
    pub fn set_background_sync_params(&mut self, value: Win32_OfflineFilesBackgroundSync) {
        self.background_sync_params = Some(value);
    }

    /// Gets the value of BackgroundSyncParams
    pub fn get_background_sync_params(&self) -> Option<&Win32_OfflineFilesBackgroundSync> {
        self.background_sync_params.as_ref()
    }

    /// Sets the value of DiskSpaceLimitEnabled
    pub fn set_disk_space_limit_enabled(&mut self, value: bool) {
        self.disk_space_limit_enabled = Some(value);
    }

    /// Gets the value of DiskSpaceLimitEnabled
    pub fn get_disk_space_limit_enabled(&self) -> Option<&bool> {
        self.disk_space_limit_enabled.as_ref()
    }

    /// Sets the value of DiskSpaceLimitParams
    pub fn set_disk_space_limit_params(&mut self, value: Win32_OfflineFilesDiskSpaceLimit) {
        self.disk_space_limit_params = Some(value);
    }

    /// Gets the value of DiskSpaceLimitParams
    pub fn get_disk_space_limit_params(&self) -> Option<&Win32_OfflineFilesDiskSpaceLimit> {
        self.disk_space_limit_params.as_ref()
    }

    /// Sets the value of EconomicalAdminPinningEnabled
    pub fn set_economical_admin_pinning_enabled(&mut self, value: bool) {
        self.economical_admin_pinning_enabled = Some(value);
    }

    /// Gets the value of EconomicalAdminPinningEnabled
    pub fn get_economical_admin_pinning_enabled(&self) -> Option<&bool> {
        self.economical_admin_pinning_enabled.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of ExcludedFileTypes
    pub fn set_excluded_file_types(&mut self, value: Vec<String>) {
        self.excluded_file_types = value;
    }

    /// Gets the value of ExcludedFileTypes
    pub fn get_excluded_file_types(&self) -> &Vec<String> {
        &self.excluded_file_types
    }

    /// Sets the value of IsConfiguredByWMI
    pub fn set_is_configured_by_wmi(&mut self, value: bool) {
        self.is_configured_by_wmi = Some(value);
    }

    /// Gets the value of IsConfiguredByWMI
    pub fn get_is_configured_by_wmi(&self) -> Option<&bool> {
        self.is_configured_by_wmi.as_ref()
    }

    /// Sets the value of MakeAvailableOfflineButtonRemoved
    pub fn set_make_available_offline_button_removed(&mut self, value: bool) {
        self.make_available_offline_button_removed = Some(value);
    }

    /// Gets the value of MakeAvailableOfflineButtonRemoved
    pub fn get_make_available_offline_button_removed(&self) -> Option<&bool> {
        self.make_available_offline_button_removed.as_ref()
    }

    /// Sets the value of OfflineFilesCacheEncrypted
    pub fn set_offline_files_cache_encrypted(&mut self, value: bool) {
        self.offline_files_cache_encrypted = Some(value);
    }

    /// Gets the value of OfflineFilesCacheEncrypted
    pub fn get_offline_files_cache_encrypted(&self) -> Option<&bool> {
        self.offline_files_cache_encrypted.as_ref()
    }

    /// Sets the value of SlowLinkEnabled
    pub fn set_slow_link_enabled(&mut self, value: bool) {
        self.slow_link_enabled = Some(value);
    }

    /// Gets the value of SlowLinkEnabled
    pub fn get_slow_link_enabled(&self) -> Option<&bool> {
        self.slow_link_enabled.as_ref()
    }

    /// Sets the value of SlowLinkParams
    pub fn set_slow_link_params(&mut self, value: Vec<String>) {
        self.slow_link_params = value;
    }

    /// Gets the value of SlowLinkParams
    pub fn get_slow_link_params(&self) -> &Vec<String> {
        &self.slow_link_params
    }

    /// Sets the value of SyncOnCostedNetworkEnabled
    pub fn set_sync_on_costed_network_enabled(&mut self, value: bool) {
        self.sync_on_costed_network_enabled = Some(value);
    }

    /// Gets the value of SyncOnCostedNetworkEnabled
    pub fn get_sync_on_costed_network_enabled(&self) -> Option<&bool> {
        self.sync_on_costed_network_enabled.as_ref()
    }

    /// Sets the value of TransparentCachingLatencyThreshold
    pub fn set_transparent_caching_latency_threshold(&mut self, value: u32) {
        self.transparent_caching_latency_threshold = Some(value);
    }

    /// Gets the value of TransparentCachingLatencyThreshold
    pub fn get_transparent_caching_latency_threshold(&self) -> Option<&u32> {
        self.transparent_caching_latency_threshold.as_ref()
    }

    /// Sets the value of WorkOfflineButtonRemoved
    pub fn set_work_offline_button_removed(&mut self, value: bool) {
        self.work_offline_button_removed = Some(value);
    }

    /// Gets the value of WorkOfflineButtonRemoved
    pub fn get_work_offline_button_removed(&self) -> Option<&bool> {
        self.work_offline_button_removed.as_ref()
    }
}

