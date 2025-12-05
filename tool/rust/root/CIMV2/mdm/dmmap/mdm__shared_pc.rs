// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_SharedPC struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_SharedPC {

/// 
    #[serde(rename = "AccountModel")]
    pub account_model: Option<i32>,

/// 
    #[serde(rename = "DeletionPolicy")]
    pub deletion_policy: Option<i32>,

/// 
    #[serde(rename = "DiskLevelCaching")]
    pub disk_level_caching: Option<i32>,

/// 
    #[serde(rename = "DiskLevelDeletion")]
    pub disk_level_deletion: Option<i32>,

/// 
    #[serde(rename = "EnableAccountManager")]
    pub enable_account_manager: Option<bool>,

/// 
    #[serde(rename = "EnableSharedPCMode")]
    pub enable_shared_pcmode: Option<bool>,

/// 
    #[serde(rename = "InactiveThreshold")]
    pub inactive_threshold: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "KioskModeAUMID")]
    pub kiosk_mode_aumid: Option<String>,

/// 
    #[serde(rename = "KioskModeUserTileDisplayText")]
    pub kiosk_mode_user_tile_display_text: Option<String>,

/// 
    #[serde(rename = "MaintenanceStartTime")]
    pub maintenance_start_time: Option<i32>,

/// 
    #[serde(rename = "MaxPageFileSizeMB")]
    pub max_page_file_size_mb: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RestrictLocalStorage")]
    pub restrict_local_storage: Option<bool>,

/// 
    #[serde(rename = "SetEduPolicies")]
    pub set_edu_policies: Option<bool>,

/// 
    #[serde(rename = "SetPowerPolicies")]
    pub set_power_policies: Option<bool>,

/// 
    #[serde(rename = "SignInOnResume")]
    pub sign_in_on_resume: Option<bool>,

/// 
    #[serde(rename = "SleepTimeout")]
    pub sleep_timeout: Option<i32>,
}

impl MDM_SharedPC {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            account_model: None,
            deletion_policy: None,
            disk_level_caching: None,
            disk_level_deletion: None,
            enable_account_manager: None,
            enable_shared_pcmode: None,
            inactive_threshold: None,
            instance_id: None,
            kiosk_mode_aumid: None,
            kiosk_mode_user_tile_display_text: None,
            maintenance_start_time: None,
            max_page_file_size_mb: None,
            parent_id: None,
            restrict_local_storage: None,
            set_edu_policies: None,
            set_power_policies: None,
            sign_in_on_resume: None,
            sleep_timeout: None,
        }
    }


    /// Sets the value of AccountModel
    pub fn set_account_model(&mut self, value: i32) {
        self.account_model = Some(value);
    }

    /// Gets the value of AccountModel
    pub fn get_account_model(&self) -> Option<&i32> {
        self.account_model.as_ref()
    }

    /// Sets the value of DeletionPolicy
    pub fn set_deletion_policy(&mut self, value: i32) {
        self.deletion_policy = Some(value);
    }

    /// Gets the value of DeletionPolicy
    pub fn get_deletion_policy(&self) -> Option<&i32> {
        self.deletion_policy.as_ref()
    }

    /// Sets the value of DiskLevelCaching
    pub fn set_disk_level_caching(&mut self, value: i32) {
        self.disk_level_caching = Some(value);
    }

    /// Gets the value of DiskLevelCaching
    pub fn get_disk_level_caching(&self) -> Option<&i32> {
        self.disk_level_caching.as_ref()
    }

    /// Sets the value of DiskLevelDeletion
    pub fn set_disk_level_deletion(&mut self, value: i32) {
        self.disk_level_deletion = Some(value);
    }

    /// Gets the value of DiskLevelDeletion
    pub fn get_disk_level_deletion(&self) -> Option<&i32> {
        self.disk_level_deletion.as_ref()
    }

    /// Sets the value of EnableAccountManager
    pub fn set_enable_account_manager(&mut self, value: bool) {
        self.enable_account_manager = Some(value);
    }

    /// Gets the value of EnableAccountManager
    pub fn get_enable_account_manager(&self) -> Option<&bool> {
        self.enable_account_manager.as_ref()
    }

    /// Sets the value of EnableSharedPCMode
    pub fn set_enable_shared_pcmode(&mut self, value: bool) {
        self.enable_shared_pcmode = Some(value);
    }

    /// Gets the value of EnableSharedPCMode
    pub fn get_enable_shared_pcmode(&self) -> Option<&bool> {
        self.enable_shared_pcmode.as_ref()
    }

    /// Sets the value of InactiveThreshold
    pub fn set_inactive_threshold(&mut self, value: i32) {
        self.inactive_threshold = Some(value);
    }

    /// Gets the value of InactiveThreshold
    pub fn get_inactive_threshold(&self) -> Option<&i32> {
        self.inactive_threshold.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of KioskModeAUMID
    pub fn set_kiosk_mode_aumid(&mut self, value: String) {
        self.kiosk_mode_aumid = Some(value);
    }

    /// Gets the value of KioskModeAUMID
    pub fn get_kiosk_mode_aumid(&self) -> Option<&String> {
        self.kiosk_mode_aumid.as_ref()
    }

    /// Sets the value of KioskModeUserTileDisplayText
    pub fn set_kiosk_mode_user_tile_display_text(&mut self, value: String) {
        self.kiosk_mode_user_tile_display_text = Some(value);
    }

    /// Gets the value of KioskModeUserTileDisplayText
    pub fn get_kiosk_mode_user_tile_display_text(&self) -> Option<&String> {
        self.kiosk_mode_user_tile_display_text.as_ref()
    }

    /// Sets the value of MaintenanceStartTime
    pub fn set_maintenance_start_time(&mut self, value: i32) {
        self.maintenance_start_time = Some(value);
    }

    /// Gets the value of MaintenanceStartTime
    pub fn get_maintenance_start_time(&self) -> Option<&i32> {
        self.maintenance_start_time.as_ref()
    }

    /// Sets the value of MaxPageFileSizeMB
    pub fn set_max_page_file_size_mb(&mut self, value: i32) {
        self.max_page_file_size_mb = Some(value);
    }

    /// Gets the value of MaxPageFileSizeMB
    pub fn get_max_page_file_size_mb(&self) -> Option<&i32> {
        self.max_page_file_size_mb.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RestrictLocalStorage
    pub fn set_restrict_local_storage(&mut self, value: bool) {
        self.restrict_local_storage = Some(value);
    }

    /// Gets the value of RestrictLocalStorage
    pub fn get_restrict_local_storage(&self) -> Option<&bool> {
        self.restrict_local_storage.as_ref()
    }

    /// Sets the value of SetEduPolicies
    pub fn set_set_edu_policies(&mut self, value: bool) {
        self.set_edu_policies = Some(value);
    }

    /// Gets the value of SetEduPolicies
    pub fn get_set_edu_policies(&self) -> Option<&bool> {
        self.set_edu_policies.as_ref()
    }

    /// Sets the value of SetPowerPolicies
    pub fn set_set_power_policies(&mut self, value: bool) {
        self.set_power_policies = Some(value);
    }

    /// Gets the value of SetPowerPolicies
    pub fn get_set_power_policies(&self) -> Option<&bool> {
        self.set_power_policies.as_ref()
    }

    /// Sets the value of SignInOnResume
    pub fn set_sign_in_on_resume(&mut self, value: bool) {
        self.sign_in_on_resume = Some(value);
    }

    /// Gets the value of SignInOnResume
    pub fn get_sign_in_on_resume(&self) -> Option<&bool> {
        self.sign_in_on_resume.as_ref()
    }

    /// Sets the value of SleepTimeout
    pub fn set_sleep_timeout(&mut self, value: i32) {
        self.sleep_timeout = Some(value);
    }

    /// Gets the value of SleepTimeout
    pub fn get_sleep_timeout(&self) -> Option<&i32> {
        self.sleep_timeout.as_ref()
    }
}

