// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DeliveryOptimizationExtendedConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DeliveryOptimizationExtendedConfig {
    #[serde(flatten)]
    pub base: MSFT_DOBaseStatus,

/// 
    #[serde(rename = "BatteryPctToSeed")]
    pub battery_pct_to_seed: Option<u32>,

/// 
    #[serde(rename = "BatteryPctToSeedProvider")]
    pub battery_pct_to_seed_provider: Option<DeliveryOptimizationExtendedConfig_BatteryPctToSeedProvider>,

/// 
    #[serde(rename = "MinTotalDiskSize")]
    pub min_total_disk_size: Option<u32>,

/// 
    #[serde(rename = "MinTotalDiskSizeProvider")]
    pub min_total_disk_size_provider: Option<DeliveryOptimizationExtendedConfig_MinTotalDiskSizeProvider>,

/// 
    #[serde(rename = "MinTotalRAM")]
    pub min_total_ram: Option<u32>,

/// 
    #[serde(rename = "MinTotalRAMProvider")]
    pub min_total_ramprovider: Option<DeliveryOptimizationExtendedConfig_MinTotalRAMProvider>,

/// 
    #[serde(rename = "SetHoursToLimitDownloadBackground")]
    pub set_hours_to_limit_download_background: Option<String>,

/// 
    #[serde(rename = "SetHoursToLimitDownloadBackgroundProvider")]
    pub set_hours_to_limit_download_background_provider: Option<DeliveryOptimizationExtendedConfig_SetHoursToLimitDownloadBackgroundProvider>,

/// 
    #[serde(rename = "SetHoursToLimitDownloadForeground")]
    pub set_hours_to_limit_download_foreground: Option<String>,

/// 
    #[serde(rename = "SetHoursToLimitDownloadForegroundProvider")]
    pub set_hours_to_limit_download_foreground_provider: Option<DeliveryOptimizationExtendedConfig_SetHoursToLimitDownloadForegroundProvider>,

/// 
    #[serde(rename = "VpnKeywords")]
    pub vpn_keywords: Option<String>,

/// 
    #[serde(rename = "VpnKeywordsProvider")]
    pub vpn_keywords_provider: Option<DeliveryOptimizationExtendedConfig_VpnKeywordsProvider>,

/// 
    #[serde(rename = "VpnPeerCachingAllowed")]
    pub vpn_peer_caching_allowed: Option<bool>,

/// 
    #[serde(rename = "VpnPeerCachingAllowedProvider")]
    pub vpn_peer_caching_allowed_provider: Option<DeliveryOptimizationExtendedConfig_VpnPeerCachingAllowedProvider>,

/// 
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,

/// 
    #[serde(rename = "WorkingDirectoryProvider")]
    pub working_directory_provider: Option<DeliveryOptimizationExtendedConfig_WorkingDirectoryProvider>,
}

impl MSFT_DeliveryOptimizationExtendedConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DOBaseStatus::new(),
            battery_pct_to_seed: None,
            battery_pct_to_seed_provider: None,
            min_total_disk_size: None,
            min_total_disk_size_provider: None,
            min_total_ram: None,
            min_total_ramprovider: None,
            set_hours_to_limit_download_background: None,
            set_hours_to_limit_download_background_provider: None,
            set_hours_to_limit_download_foreground: None,
            set_hours_to_limit_download_foreground_provider: None,
            vpn_keywords: None,
            vpn_keywords_provider: None,
            vpn_peer_caching_allowed: None,
            vpn_peer_caching_allowed_provider: None,
            working_directory: None,
            working_directory_provider: None,
        }
    }


    /// Sets the value of BatteryPctToSeed
    pub fn set_battery_pct_to_seed(&mut self, value: u32) {
        self.battery_pct_to_seed = Some(value);
    }

    /// Gets the value of BatteryPctToSeed
    pub fn get_battery_pct_to_seed(&self) -> Option<&u32> {
        self.battery_pct_to_seed.as_ref()
    }

    /// Sets the value of BatteryPctToSeedProvider
    pub fn set_battery_pct_to_seed_provider(&mut self, value: DeliveryOptimizationExtendedConfig_BatteryPctToSeedProvider) {
        self.battery_pct_to_seed_provider = Some(value);
    }

    /// Gets the value of BatteryPctToSeedProvider
    pub fn get_battery_pct_to_seed_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_BatteryPctToSeedProvider> {
        self.battery_pct_to_seed_provider.as_ref()
    }

    /// Sets the value of MinTotalDiskSize
    pub fn set_min_total_disk_size(&mut self, value: u32) {
        self.min_total_disk_size = Some(value);
    }

    /// Gets the value of MinTotalDiskSize
    pub fn get_min_total_disk_size(&self) -> Option<&u32> {
        self.min_total_disk_size.as_ref()
    }

    /// Sets the value of MinTotalDiskSizeProvider
    pub fn set_min_total_disk_size_provider(&mut self, value: DeliveryOptimizationExtendedConfig_MinTotalDiskSizeProvider) {
        self.min_total_disk_size_provider = Some(value);
    }

    /// Gets the value of MinTotalDiskSizeProvider
    pub fn get_min_total_disk_size_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_MinTotalDiskSizeProvider> {
        self.min_total_disk_size_provider.as_ref()
    }

    /// Sets the value of MinTotalRAM
    pub fn set_min_total_ram(&mut self, value: u32) {
        self.min_total_ram = Some(value);
    }

    /// Gets the value of MinTotalRAM
    pub fn get_min_total_ram(&self) -> Option<&u32> {
        self.min_total_ram.as_ref()
    }

    /// Sets the value of MinTotalRAMProvider
    pub fn set_min_total_ramprovider(&mut self, value: DeliveryOptimizationExtendedConfig_MinTotalRAMProvider) {
        self.min_total_ramprovider = Some(value);
    }

    /// Gets the value of MinTotalRAMProvider
    pub fn get_min_total_ramprovider(&self) -> Option<&DeliveryOptimizationExtendedConfig_MinTotalRAMProvider> {
        self.min_total_ramprovider.as_ref()
    }

    /// Sets the value of SetHoursToLimitDownloadBackground
    pub fn set_set_hours_to_limit_download_background(&mut self, value: String) {
        self.set_hours_to_limit_download_background = Some(value);
    }

    /// Gets the value of SetHoursToLimitDownloadBackground
    pub fn get_set_hours_to_limit_download_background(&self) -> Option<&String> {
        self.set_hours_to_limit_download_background.as_ref()
    }

    /// Sets the value of SetHoursToLimitDownloadBackgroundProvider
    pub fn set_set_hours_to_limit_download_background_provider(&mut self, value: DeliveryOptimizationExtendedConfig_SetHoursToLimitDownloadBackgroundProvider) {
        self.set_hours_to_limit_download_background_provider = Some(value);
    }

    /// Gets the value of SetHoursToLimitDownloadBackgroundProvider
    pub fn get_set_hours_to_limit_download_background_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_SetHoursToLimitDownloadBackgroundProvider> {
        self.set_hours_to_limit_download_background_provider.as_ref()
    }

    /// Sets the value of SetHoursToLimitDownloadForeground
    pub fn set_set_hours_to_limit_download_foreground(&mut self, value: String) {
        self.set_hours_to_limit_download_foreground = Some(value);
    }

    /// Gets the value of SetHoursToLimitDownloadForeground
    pub fn get_set_hours_to_limit_download_foreground(&self) -> Option<&String> {
        self.set_hours_to_limit_download_foreground.as_ref()
    }

    /// Sets the value of SetHoursToLimitDownloadForegroundProvider
    pub fn set_set_hours_to_limit_download_foreground_provider(&mut self, value: DeliveryOptimizationExtendedConfig_SetHoursToLimitDownloadForegroundProvider) {
        self.set_hours_to_limit_download_foreground_provider = Some(value);
    }

    /// Gets the value of SetHoursToLimitDownloadForegroundProvider
    pub fn get_set_hours_to_limit_download_foreground_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_SetHoursToLimitDownloadForegroundProvider> {
        self.set_hours_to_limit_download_foreground_provider.as_ref()
    }

    /// Sets the value of VpnKeywords
    pub fn set_vpn_keywords(&mut self, value: String) {
        self.vpn_keywords = Some(value);
    }

    /// Gets the value of VpnKeywords
    pub fn get_vpn_keywords(&self) -> Option<&String> {
        self.vpn_keywords.as_ref()
    }

    /// Sets the value of VpnKeywordsProvider
    pub fn set_vpn_keywords_provider(&mut self, value: DeliveryOptimizationExtendedConfig_VpnKeywordsProvider) {
        self.vpn_keywords_provider = Some(value);
    }

    /// Gets the value of VpnKeywordsProvider
    pub fn get_vpn_keywords_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_VpnKeywordsProvider> {
        self.vpn_keywords_provider.as_ref()
    }

    /// Sets the value of VpnPeerCachingAllowed
    pub fn set_vpn_peer_caching_allowed(&mut self, value: bool) {
        self.vpn_peer_caching_allowed = Some(value);
    }

    /// Gets the value of VpnPeerCachingAllowed
    pub fn get_vpn_peer_caching_allowed(&self) -> Option<&bool> {
        self.vpn_peer_caching_allowed.as_ref()
    }

    /// Sets the value of VpnPeerCachingAllowedProvider
    pub fn set_vpn_peer_caching_allowed_provider(&mut self, value: DeliveryOptimizationExtendedConfig_VpnPeerCachingAllowedProvider) {
        self.vpn_peer_caching_allowed_provider = Some(value);
    }

    /// Gets the value of VpnPeerCachingAllowedProvider
    pub fn get_vpn_peer_caching_allowed_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_VpnPeerCachingAllowedProvider> {
        self.vpn_peer_caching_allowed_provider.as_ref()
    }

    /// Sets the value of WorkingDirectory
    pub fn set_working_directory(&mut self, value: String) {
        self.working_directory = Some(value);
    }

    /// Gets the value of WorkingDirectory
    pub fn get_working_directory(&self) -> Option<&String> {
        self.working_directory.as_ref()
    }

    /// Sets the value of WorkingDirectoryProvider
    pub fn set_working_directory_provider(&mut self, value: DeliveryOptimizationExtendedConfig_WorkingDirectoryProvider) {
        self.working_directory_provider = Some(value);
    }

    /// Gets the value of WorkingDirectoryProvider
    pub fn get_working_directory_provider(&self) -> Option<&DeliveryOptimizationExtendedConfig_WorkingDirectoryProvider> {
        self.working_directory_provider.as_ref()
    }
}

