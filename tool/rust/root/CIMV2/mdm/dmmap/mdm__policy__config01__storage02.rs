// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Storage02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Storage02 {

/// 
    #[serde(rename = "AllowDiskHealthModelUpdates")]
    pub allow_disk_health_model_updates: Option<i32>,

/// 
    #[serde(rename = "AllowStorageSenseGlobal")]
    pub allow_storage_sense_global: Option<i32>,

/// 
    #[serde(rename = "AllowStorageSenseTemporaryFilesCleanup")]
    pub allow_storage_sense_temporary_files_cleanup: Option<i32>,

/// 
    #[serde(rename = "ConfigStorageSenseCloudContentDehydrationThreshold")]
    pub config_storage_sense_cloud_content_dehydration_threshold: Option<i32>,

/// 
    #[serde(rename = "ConfigStorageSenseDownloadsCleanupThreshold")]
    pub config_storage_sense_downloads_cleanup_threshold: Option<i32>,

/// 
    #[serde(rename = "ConfigStorageSenseGlobalCadence")]
    pub config_storage_sense_global_cadence: Option<i32>,

/// 
    #[serde(rename = "ConfigStorageSenseRecycleBinCleanupThreshold")]
    pub config_storage_sense_recycle_bin_cleanup_threshold: Option<i32>,

/// 
    #[serde(rename = "EnhancedStorageDevices")]
    pub enhanced_storage_devices: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RemovableDiskDenyWriteAccess")]
    pub removable_disk_deny_write_access: Option<i32>,

/// 
    #[serde(rename = "WPDDevicesDenyReadAccessPerDevice")]
    pub wpddevices_deny_read_access_per_device: Option<String>,

/// 
    #[serde(rename = "WPDDevicesDenyWriteAccessPerDevice")]
    pub wpddevices_deny_write_access_per_device: Option<String>,
}

impl MDM_Policy_Config01_Storage02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_disk_health_model_updates: None,
            allow_storage_sense_global: None,
            allow_storage_sense_temporary_files_cleanup: None,
            config_storage_sense_cloud_content_dehydration_threshold: None,
            config_storage_sense_downloads_cleanup_threshold: None,
            config_storage_sense_global_cadence: None,
            config_storage_sense_recycle_bin_cleanup_threshold: None,
            enhanced_storage_devices: None,
            instance_id: None,
            parent_id: None,
            removable_disk_deny_write_access: None,
            wpddevices_deny_read_access_per_device: None,
            wpddevices_deny_write_access_per_device: None,
        }
    }


    /// Sets the value of AllowDiskHealthModelUpdates
    pub fn set_allow_disk_health_model_updates(&mut self, value: i32) {
        self.allow_disk_health_model_updates = Some(value);
    }

    /// Gets the value of AllowDiskHealthModelUpdates
    pub fn get_allow_disk_health_model_updates(&self) -> Option<&i32> {
        self.allow_disk_health_model_updates.as_ref()
    }

    /// Sets the value of AllowStorageSenseGlobal
    pub fn set_allow_storage_sense_global(&mut self, value: i32) {
        self.allow_storage_sense_global = Some(value);
    }

    /// Gets the value of AllowStorageSenseGlobal
    pub fn get_allow_storage_sense_global(&self) -> Option<&i32> {
        self.allow_storage_sense_global.as_ref()
    }

    /// Sets the value of AllowStorageSenseTemporaryFilesCleanup
    pub fn set_allow_storage_sense_temporary_files_cleanup(&mut self, value: i32) {
        self.allow_storage_sense_temporary_files_cleanup = Some(value);
    }

    /// Gets the value of AllowStorageSenseTemporaryFilesCleanup
    pub fn get_allow_storage_sense_temporary_files_cleanup(&self) -> Option<&i32> {
        self.allow_storage_sense_temporary_files_cleanup.as_ref()
    }

    /// Sets the value of ConfigStorageSenseCloudContentDehydrationThreshold
    pub fn set_config_storage_sense_cloud_content_dehydration_threshold(&mut self, value: i32) {
        self.config_storage_sense_cloud_content_dehydration_threshold = Some(value);
    }

    /// Gets the value of ConfigStorageSenseCloudContentDehydrationThreshold
    pub fn get_config_storage_sense_cloud_content_dehydration_threshold(&self) -> Option<&i32> {
        self.config_storage_sense_cloud_content_dehydration_threshold.as_ref()
    }

    /// Sets the value of ConfigStorageSenseDownloadsCleanupThreshold
    pub fn set_config_storage_sense_downloads_cleanup_threshold(&mut self, value: i32) {
        self.config_storage_sense_downloads_cleanup_threshold = Some(value);
    }

    /// Gets the value of ConfigStorageSenseDownloadsCleanupThreshold
    pub fn get_config_storage_sense_downloads_cleanup_threshold(&self) -> Option<&i32> {
        self.config_storage_sense_downloads_cleanup_threshold.as_ref()
    }

    /// Sets the value of ConfigStorageSenseGlobalCadence
    pub fn set_config_storage_sense_global_cadence(&mut self, value: i32) {
        self.config_storage_sense_global_cadence = Some(value);
    }

    /// Gets the value of ConfigStorageSenseGlobalCadence
    pub fn get_config_storage_sense_global_cadence(&self) -> Option<&i32> {
        self.config_storage_sense_global_cadence.as_ref()
    }

    /// Sets the value of ConfigStorageSenseRecycleBinCleanupThreshold
    pub fn set_config_storage_sense_recycle_bin_cleanup_threshold(&mut self, value: i32) {
        self.config_storage_sense_recycle_bin_cleanup_threshold = Some(value);
    }

    /// Gets the value of ConfigStorageSenseRecycleBinCleanupThreshold
    pub fn get_config_storage_sense_recycle_bin_cleanup_threshold(&self) -> Option<&i32> {
        self.config_storage_sense_recycle_bin_cleanup_threshold.as_ref()
    }

    /// Sets the value of EnhancedStorageDevices
    pub fn set_enhanced_storage_devices(&mut self, value: String) {
        self.enhanced_storage_devices = Some(value);
    }

    /// Gets the value of EnhancedStorageDevices
    pub fn get_enhanced_storage_devices(&self) -> Option<&String> {
        self.enhanced_storage_devices.as_ref()
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

    /// Sets the value of RemovableDiskDenyWriteAccess
    pub fn set_removable_disk_deny_write_access(&mut self, value: i32) {
        self.removable_disk_deny_write_access = Some(value);
    }

    /// Gets the value of RemovableDiskDenyWriteAccess
    pub fn get_removable_disk_deny_write_access(&self) -> Option<&i32> {
        self.removable_disk_deny_write_access.as_ref()
    }

    /// Sets the value of WPDDevicesDenyReadAccessPerDevice
    pub fn set_wpddevices_deny_read_access_per_device(&mut self, value: String) {
        self.wpddevices_deny_read_access_per_device = Some(value);
    }

    /// Gets the value of WPDDevicesDenyReadAccessPerDevice
    pub fn get_wpddevices_deny_read_access_per_device(&self) -> Option<&String> {
        self.wpddevices_deny_read_access_per_device.as_ref()
    }

    /// Sets the value of WPDDevicesDenyWriteAccessPerDevice
    pub fn set_wpddevices_deny_write_access_per_device(&mut self, value: String) {
        self.wpddevices_deny_write_access_per_device = Some(value);
    }

    /// Gets the value of WPDDevicesDenyWriteAccessPerDevice
    pub fn get_wpddevices_deny_write_access_per_device(&self) -> Option<&String> {
        self.wpddevices_deny_write_access_per_device.as_ref()
    }
}

