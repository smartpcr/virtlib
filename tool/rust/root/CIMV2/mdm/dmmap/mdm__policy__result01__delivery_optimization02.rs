// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_DeliveryOptimization02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_DeliveryOptimization02 {

/// 
    #[serde(rename = "DOAbsoluteMaxCacheSize")]
    pub doabsolute_max_cache_size: Option<i32>,

/// 
    #[serde(rename = "DOAllowVPNPeerCaching")]
    pub doallow_vpnpeer_caching: Option<i32>,

/// 
    #[serde(rename = "DOCacheHost")]
    pub docache_host: Option<String>,

/// 
    #[serde(rename = "DOCacheHostSource")]
    pub docache_host_source: Option<i32>,

/// 
    #[serde(rename = "DODelayBackgroundDownloadFromHttp")]
    pub dodelay_background_download_from_http: Option<i32>,

/// 
    #[serde(rename = "DODelayCacheServerFallbackBackground")]
    pub dodelay_cache_server_fallback_background: Option<i32>,

/// 
    #[serde(rename = "DODelayCacheServerFallbackForeground")]
    pub dodelay_cache_server_fallback_foreground: Option<i32>,

/// 
    #[serde(rename = "DODelayForegroundDownloadFromHttp")]
    pub dodelay_foreground_download_from_http: Option<i32>,

/// 
    #[serde(rename = "DODownloadMode")]
    pub dodownload_mode: Option<i32>,

/// 
    #[serde(rename = "DOGroupId")]
    pub dogroup_id: Option<String>,

/// 
    #[serde(rename = "DOGroupIdSource")]
    pub dogroup_id_source: Option<i32>,

/// 
    #[serde(rename = "DOMaxBackgroundDownloadBandwidth")]
    pub domax_background_download_bandwidth: Option<i32>,

/// 
    #[serde(rename = "DOMaxCacheAge")]
    pub domax_cache_age: Option<i32>,

/// 
    #[serde(rename = "DOMaxCacheSize")]
    pub domax_cache_size: Option<i32>,

/// 
    #[serde(rename = "DOMaxForegroundDownloadBandwidth")]
    pub domax_foreground_download_bandwidth: Option<i32>,

/// 
    #[serde(rename = "DOMinBackgroundQos")]
    pub domin_background_qos: Option<i32>,

/// 
    #[serde(rename = "DOMinBatteryPercentageAllowedToUpload")]
    pub domin_battery_percentage_allowed_to_upload: Option<i32>,

/// 
    #[serde(rename = "DOMinDiskSizeAllowedToPeer")]
    pub domin_disk_size_allowed_to_peer: Option<i32>,

/// 
    #[serde(rename = "DOMinFileSizeToCache")]
    pub domin_file_size_to_cache: Option<i32>,

/// 
    #[serde(rename = "DOMinRAMAllowedToPeer")]
    pub domin_ramallowed_to_peer: Option<i32>,

/// 
    #[serde(rename = "DOModifyCacheDrive")]
    pub domodify_cache_drive: Option<String>,

/// 
    #[serde(rename = "DOMonthlyUploadDataCap")]
    pub domonthly_upload_data_cap: Option<i32>,

/// 
    #[serde(rename = "DOPercentageMaxBackgroundBandwidth")]
    pub dopercentage_max_background_bandwidth: Option<i32>,

/// 
    #[serde(rename = "DOPercentageMaxForegroundBandwidth")]
    pub dopercentage_max_foreground_bandwidth: Option<i32>,

/// 
    #[serde(rename = "DORestrictPeerSelectionBy")]
    pub dorestrict_peer_selection_by: Option<i32>,

/// 
    #[serde(rename = "DOSetHoursToLimitBackgroundDownloadBandwidth")]
    pub doset_hours_to_limit_background_download_bandwidth: Option<String>,

/// 
    #[serde(rename = "DOSetHoursToLimitForegroundDownloadBandwidth")]
    pub doset_hours_to_limit_foreground_download_bandwidth: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_DeliveryOptimization02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            doabsolute_max_cache_size: None,
            doallow_vpnpeer_caching: None,
            docache_host: None,
            docache_host_source: None,
            dodelay_background_download_from_http: None,
            dodelay_cache_server_fallback_background: None,
            dodelay_cache_server_fallback_foreground: None,
            dodelay_foreground_download_from_http: None,
            dodownload_mode: None,
            dogroup_id: None,
            dogroup_id_source: None,
            domax_background_download_bandwidth: None,
            domax_cache_age: None,
            domax_cache_size: None,
            domax_foreground_download_bandwidth: None,
            domin_background_qos: None,
            domin_battery_percentage_allowed_to_upload: None,
            domin_disk_size_allowed_to_peer: None,
            domin_file_size_to_cache: None,
            domin_ramallowed_to_peer: None,
            domodify_cache_drive: None,
            domonthly_upload_data_cap: None,
            dopercentage_max_background_bandwidth: None,
            dopercentage_max_foreground_bandwidth: None,
            dorestrict_peer_selection_by: None,
            doset_hours_to_limit_background_download_bandwidth: None,
            doset_hours_to_limit_foreground_download_bandwidth: None,
            instance_id: None,
            parent_id: None,
        }
    }


    /// Sets the value of DOAbsoluteMaxCacheSize
    pub fn set_doabsolute_max_cache_size(&mut self, value: i32) {
        self.doabsolute_max_cache_size = Some(value);
    }

    /// Gets the value of DOAbsoluteMaxCacheSize
    pub fn get_doabsolute_max_cache_size(&self) -> Option<&i32> {
        self.doabsolute_max_cache_size.as_ref()
    }

    /// Sets the value of DOAllowVPNPeerCaching
    pub fn set_doallow_vpnpeer_caching(&mut self, value: i32) {
        self.doallow_vpnpeer_caching = Some(value);
    }

    /// Gets the value of DOAllowVPNPeerCaching
    pub fn get_doallow_vpnpeer_caching(&self) -> Option<&i32> {
        self.doallow_vpnpeer_caching.as_ref()
    }

    /// Sets the value of DOCacheHost
    pub fn set_docache_host(&mut self, value: String) {
        self.docache_host = Some(value);
    }

    /// Gets the value of DOCacheHost
    pub fn get_docache_host(&self) -> Option<&String> {
        self.docache_host.as_ref()
    }

    /// Sets the value of DOCacheHostSource
    pub fn set_docache_host_source(&mut self, value: i32) {
        self.docache_host_source = Some(value);
    }

    /// Gets the value of DOCacheHostSource
    pub fn get_docache_host_source(&self) -> Option<&i32> {
        self.docache_host_source.as_ref()
    }

    /// Sets the value of DODelayBackgroundDownloadFromHttp
    pub fn set_dodelay_background_download_from_http(&mut self, value: i32) {
        self.dodelay_background_download_from_http = Some(value);
    }

    /// Gets the value of DODelayBackgroundDownloadFromHttp
    pub fn get_dodelay_background_download_from_http(&self) -> Option<&i32> {
        self.dodelay_background_download_from_http.as_ref()
    }

    /// Sets the value of DODelayCacheServerFallbackBackground
    pub fn set_dodelay_cache_server_fallback_background(&mut self, value: i32) {
        self.dodelay_cache_server_fallback_background = Some(value);
    }

    /// Gets the value of DODelayCacheServerFallbackBackground
    pub fn get_dodelay_cache_server_fallback_background(&self) -> Option<&i32> {
        self.dodelay_cache_server_fallback_background.as_ref()
    }

    /// Sets the value of DODelayCacheServerFallbackForeground
    pub fn set_dodelay_cache_server_fallback_foreground(&mut self, value: i32) {
        self.dodelay_cache_server_fallback_foreground = Some(value);
    }

    /// Gets the value of DODelayCacheServerFallbackForeground
    pub fn get_dodelay_cache_server_fallback_foreground(&self) -> Option<&i32> {
        self.dodelay_cache_server_fallback_foreground.as_ref()
    }

    /// Sets the value of DODelayForegroundDownloadFromHttp
    pub fn set_dodelay_foreground_download_from_http(&mut self, value: i32) {
        self.dodelay_foreground_download_from_http = Some(value);
    }

    /// Gets the value of DODelayForegroundDownloadFromHttp
    pub fn get_dodelay_foreground_download_from_http(&self) -> Option<&i32> {
        self.dodelay_foreground_download_from_http.as_ref()
    }

    /// Sets the value of DODownloadMode
    pub fn set_dodownload_mode(&mut self, value: i32) {
        self.dodownload_mode = Some(value);
    }

    /// Gets the value of DODownloadMode
    pub fn get_dodownload_mode(&self) -> Option<&i32> {
        self.dodownload_mode.as_ref()
    }

    /// Sets the value of DOGroupId
    pub fn set_dogroup_id(&mut self, value: String) {
        self.dogroup_id = Some(value);
    }

    /// Gets the value of DOGroupId
    pub fn get_dogroup_id(&self) -> Option<&String> {
        self.dogroup_id.as_ref()
    }

    /// Sets the value of DOGroupIdSource
    pub fn set_dogroup_id_source(&mut self, value: i32) {
        self.dogroup_id_source = Some(value);
    }

    /// Gets the value of DOGroupIdSource
    pub fn get_dogroup_id_source(&self) -> Option<&i32> {
        self.dogroup_id_source.as_ref()
    }

    /// Sets the value of DOMaxBackgroundDownloadBandwidth
    pub fn set_domax_background_download_bandwidth(&mut self, value: i32) {
        self.domax_background_download_bandwidth = Some(value);
    }

    /// Gets the value of DOMaxBackgroundDownloadBandwidth
    pub fn get_domax_background_download_bandwidth(&self) -> Option<&i32> {
        self.domax_background_download_bandwidth.as_ref()
    }

    /// Sets the value of DOMaxCacheAge
    pub fn set_domax_cache_age(&mut self, value: i32) {
        self.domax_cache_age = Some(value);
    }

    /// Gets the value of DOMaxCacheAge
    pub fn get_domax_cache_age(&self) -> Option<&i32> {
        self.domax_cache_age.as_ref()
    }

    /// Sets the value of DOMaxCacheSize
    pub fn set_domax_cache_size(&mut self, value: i32) {
        self.domax_cache_size = Some(value);
    }

    /// Gets the value of DOMaxCacheSize
    pub fn get_domax_cache_size(&self) -> Option<&i32> {
        self.domax_cache_size.as_ref()
    }

    /// Sets the value of DOMaxForegroundDownloadBandwidth
    pub fn set_domax_foreground_download_bandwidth(&mut self, value: i32) {
        self.domax_foreground_download_bandwidth = Some(value);
    }

    /// Gets the value of DOMaxForegroundDownloadBandwidth
    pub fn get_domax_foreground_download_bandwidth(&self) -> Option<&i32> {
        self.domax_foreground_download_bandwidth.as_ref()
    }

    /// Sets the value of DOMinBackgroundQos
    pub fn set_domin_background_qos(&mut self, value: i32) {
        self.domin_background_qos = Some(value);
    }

    /// Gets the value of DOMinBackgroundQos
    pub fn get_domin_background_qos(&self) -> Option<&i32> {
        self.domin_background_qos.as_ref()
    }

    /// Sets the value of DOMinBatteryPercentageAllowedToUpload
    pub fn set_domin_battery_percentage_allowed_to_upload(&mut self, value: i32) {
        self.domin_battery_percentage_allowed_to_upload = Some(value);
    }

    /// Gets the value of DOMinBatteryPercentageAllowedToUpload
    pub fn get_domin_battery_percentage_allowed_to_upload(&self) -> Option<&i32> {
        self.domin_battery_percentage_allowed_to_upload.as_ref()
    }

    /// Sets the value of DOMinDiskSizeAllowedToPeer
    pub fn set_domin_disk_size_allowed_to_peer(&mut self, value: i32) {
        self.domin_disk_size_allowed_to_peer = Some(value);
    }

    /// Gets the value of DOMinDiskSizeAllowedToPeer
    pub fn get_domin_disk_size_allowed_to_peer(&self) -> Option<&i32> {
        self.domin_disk_size_allowed_to_peer.as_ref()
    }

    /// Sets the value of DOMinFileSizeToCache
    pub fn set_domin_file_size_to_cache(&mut self, value: i32) {
        self.domin_file_size_to_cache = Some(value);
    }

    /// Gets the value of DOMinFileSizeToCache
    pub fn get_domin_file_size_to_cache(&self) -> Option<&i32> {
        self.domin_file_size_to_cache.as_ref()
    }

    /// Sets the value of DOMinRAMAllowedToPeer
    pub fn set_domin_ramallowed_to_peer(&mut self, value: i32) {
        self.domin_ramallowed_to_peer = Some(value);
    }

    /// Gets the value of DOMinRAMAllowedToPeer
    pub fn get_domin_ramallowed_to_peer(&self) -> Option<&i32> {
        self.domin_ramallowed_to_peer.as_ref()
    }

    /// Sets the value of DOModifyCacheDrive
    pub fn set_domodify_cache_drive(&mut self, value: String) {
        self.domodify_cache_drive = Some(value);
    }

    /// Gets the value of DOModifyCacheDrive
    pub fn get_domodify_cache_drive(&self) -> Option<&String> {
        self.domodify_cache_drive.as_ref()
    }

    /// Sets the value of DOMonthlyUploadDataCap
    pub fn set_domonthly_upload_data_cap(&mut self, value: i32) {
        self.domonthly_upload_data_cap = Some(value);
    }

    /// Gets the value of DOMonthlyUploadDataCap
    pub fn get_domonthly_upload_data_cap(&self) -> Option<&i32> {
        self.domonthly_upload_data_cap.as_ref()
    }

    /// Sets the value of DOPercentageMaxBackgroundBandwidth
    pub fn set_dopercentage_max_background_bandwidth(&mut self, value: i32) {
        self.dopercentage_max_background_bandwidth = Some(value);
    }

    /// Gets the value of DOPercentageMaxBackgroundBandwidth
    pub fn get_dopercentage_max_background_bandwidth(&self) -> Option<&i32> {
        self.dopercentage_max_background_bandwidth.as_ref()
    }

    /// Sets the value of DOPercentageMaxForegroundBandwidth
    pub fn set_dopercentage_max_foreground_bandwidth(&mut self, value: i32) {
        self.dopercentage_max_foreground_bandwidth = Some(value);
    }

    /// Gets the value of DOPercentageMaxForegroundBandwidth
    pub fn get_dopercentage_max_foreground_bandwidth(&self) -> Option<&i32> {
        self.dopercentage_max_foreground_bandwidth.as_ref()
    }

    /// Sets the value of DORestrictPeerSelectionBy
    pub fn set_dorestrict_peer_selection_by(&mut self, value: i32) {
        self.dorestrict_peer_selection_by = Some(value);
    }

    /// Gets the value of DORestrictPeerSelectionBy
    pub fn get_dorestrict_peer_selection_by(&self) -> Option<&i32> {
        self.dorestrict_peer_selection_by.as_ref()
    }

    /// Sets the value of DOSetHoursToLimitBackgroundDownloadBandwidth
    pub fn set_doset_hours_to_limit_background_download_bandwidth(&mut self, value: String) {
        self.doset_hours_to_limit_background_download_bandwidth = Some(value);
    }

    /// Gets the value of DOSetHoursToLimitBackgroundDownloadBandwidth
    pub fn get_doset_hours_to_limit_background_download_bandwidth(&self) -> Option<&String> {
        self.doset_hours_to_limit_background_download_bandwidth.as_ref()
    }

    /// Sets the value of DOSetHoursToLimitForegroundDownloadBandwidth
    pub fn set_doset_hours_to_limit_foreground_download_bandwidth(&mut self, value: String) {
        self.doset_hours_to_limit_foreground_download_bandwidth = Some(value);
    }

    /// Gets the value of DOSetHoursToLimitForegroundDownloadBandwidth
    pub fn get_doset_hours_to_limit_foreground_download_bandwidth(&self) -> Option<&String> {
        self.doset_hours_to_limit_foreground_download_bandwidth.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

