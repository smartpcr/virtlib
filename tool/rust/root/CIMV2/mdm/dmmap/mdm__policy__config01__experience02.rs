// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Experience02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Experience02 {

/// 
    #[serde(rename = "AllowClipboardHistory")]
    pub allow_clipboard_history: Option<i32>,

/// 
    #[serde(rename = "AllowCortana")]
    pub allow_cortana: Option<i32>,

/// 
    #[serde(rename = "AllowDeviceDiscovery")]
    pub allow_device_discovery: Option<i32>,

/// 
    #[serde(rename = "AllowFindMyDevice")]
    pub allow_find_my_device: Option<i32>,

/// 
    #[serde(rename = "AllowManualMDMUnenrollment")]
    pub allow_manual_mdmunenrollment: Option<i32>,

/// 
    #[serde(rename = "AllowSaveAsOfOfficeFiles")]
    pub allow_save_as_of_office_files: Option<i32>,

/// 
    #[serde(rename = "AllowScreenCapture")]
    pub allow_screen_capture: Option<i32>,

/// 
    #[serde(rename = "AllowSharingOfOfficeFiles")]
    pub allow_sharing_of_office_files: Option<i32>,

/// 
    #[serde(rename = "AllowSIMErrorDialogPromptWhenNoSIM")]
    pub allow_simerror_dialog_prompt_when_no_sim: Option<i32>,

/// 
    #[serde(rename = "AllowSyncMySettings")]
    pub allow_sync_my_settings: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsConsumerFeatures")]
    pub allow_windows_consumer_features: Option<i32>,

/// 
    #[serde(rename = "AllowWindowsTips")]
    pub allow_windows_tips: Option<i32>,

/// 
    #[serde(rename = "DisableCloudOptimizedContent")]
    pub disable_cloud_optimized_content: Option<i32>,

/// 
    #[serde(rename = "DisableConsumerAccountStateContent")]
    pub disable_consumer_account_state_content: Option<i32>,

/// 
    #[serde(rename = "DoNotShowFeedbackNotifications")]
    pub do_not_show_feedback_notifications: Option<i32>,

/// 
    #[serde(rename = "DoNotSyncBrowserSettings")]
    pub do_not_sync_browser_settings: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventUsersFromTurningOnBrowserSyncing")]
    pub prevent_users_from_turning_on_browser_syncing: Option<i32>,

/// 
    #[serde(rename = "ShowLockOnUserTile")]
    pub show_lock_on_user_tile: Option<i32>,
}

impl MDM_Policy_Config01_Experience02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_clipboard_history: None,
            allow_cortana: None,
            allow_device_discovery: None,
            allow_find_my_device: None,
            allow_manual_mdmunenrollment: None,
            allow_save_as_of_office_files: None,
            allow_screen_capture: None,
            allow_sharing_of_office_files: None,
            allow_simerror_dialog_prompt_when_no_sim: None,
            allow_sync_my_settings: None,
            allow_windows_consumer_features: None,
            allow_windows_tips: None,
            disable_cloud_optimized_content: None,
            disable_consumer_account_state_content: None,
            do_not_show_feedback_notifications: None,
            do_not_sync_browser_settings: None,
            instance_id: None,
            parent_id: None,
            prevent_users_from_turning_on_browser_syncing: None,
            show_lock_on_user_tile: None,
        }
    }


    /// Sets the value of AllowClipboardHistory
    pub fn set_allow_clipboard_history(&mut self, value: i32) {
        self.allow_clipboard_history = Some(value);
    }

    /// Gets the value of AllowClipboardHistory
    pub fn get_allow_clipboard_history(&self) -> Option<&i32> {
        self.allow_clipboard_history.as_ref()
    }

    /// Sets the value of AllowCortana
    pub fn set_allow_cortana(&mut self, value: i32) {
        self.allow_cortana = Some(value);
    }

    /// Gets the value of AllowCortana
    pub fn get_allow_cortana(&self) -> Option<&i32> {
        self.allow_cortana.as_ref()
    }

    /// Sets the value of AllowDeviceDiscovery
    pub fn set_allow_device_discovery(&mut self, value: i32) {
        self.allow_device_discovery = Some(value);
    }

    /// Gets the value of AllowDeviceDiscovery
    pub fn get_allow_device_discovery(&self) -> Option<&i32> {
        self.allow_device_discovery.as_ref()
    }

    /// Sets the value of AllowFindMyDevice
    pub fn set_allow_find_my_device(&mut self, value: i32) {
        self.allow_find_my_device = Some(value);
    }

    /// Gets the value of AllowFindMyDevice
    pub fn get_allow_find_my_device(&self) -> Option<&i32> {
        self.allow_find_my_device.as_ref()
    }

    /// Sets the value of AllowManualMDMUnenrollment
    pub fn set_allow_manual_mdmunenrollment(&mut self, value: i32) {
        self.allow_manual_mdmunenrollment = Some(value);
    }

    /// Gets the value of AllowManualMDMUnenrollment
    pub fn get_allow_manual_mdmunenrollment(&self) -> Option<&i32> {
        self.allow_manual_mdmunenrollment.as_ref()
    }

    /// Sets the value of AllowSaveAsOfOfficeFiles
    pub fn set_allow_save_as_of_office_files(&mut self, value: i32) {
        self.allow_save_as_of_office_files = Some(value);
    }

    /// Gets the value of AllowSaveAsOfOfficeFiles
    pub fn get_allow_save_as_of_office_files(&self) -> Option<&i32> {
        self.allow_save_as_of_office_files.as_ref()
    }

    /// Sets the value of AllowScreenCapture
    pub fn set_allow_screen_capture(&mut self, value: i32) {
        self.allow_screen_capture = Some(value);
    }

    /// Gets the value of AllowScreenCapture
    pub fn get_allow_screen_capture(&self) -> Option<&i32> {
        self.allow_screen_capture.as_ref()
    }

    /// Sets the value of AllowSharingOfOfficeFiles
    pub fn set_allow_sharing_of_office_files(&mut self, value: i32) {
        self.allow_sharing_of_office_files = Some(value);
    }

    /// Gets the value of AllowSharingOfOfficeFiles
    pub fn get_allow_sharing_of_office_files(&self) -> Option<&i32> {
        self.allow_sharing_of_office_files.as_ref()
    }

    /// Sets the value of AllowSIMErrorDialogPromptWhenNoSIM
    pub fn set_allow_simerror_dialog_prompt_when_no_sim(&mut self, value: i32) {
        self.allow_simerror_dialog_prompt_when_no_sim = Some(value);
    }

    /// Gets the value of AllowSIMErrorDialogPromptWhenNoSIM
    pub fn get_allow_simerror_dialog_prompt_when_no_sim(&self) -> Option<&i32> {
        self.allow_simerror_dialog_prompt_when_no_sim.as_ref()
    }

    /// Sets the value of AllowSyncMySettings
    pub fn set_allow_sync_my_settings(&mut self, value: i32) {
        self.allow_sync_my_settings = Some(value);
    }

    /// Gets the value of AllowSyncMySettings
    pub fn get_allow_sync_my_settings(&self) -> Option<&i32> {
        self.allow_sync_my_settings.as_ref()
    }

    /// Sets the value of AllowWindowsConsumerFeatures
    pub fn set_allow_windows_consumer_features(&mut self, value: i32) {
        self.allow_windows_consumer_features = Some(value);
    }

    /// Gets the value of AllowWindowsConsumerFeatures
    pub fn get_allow_windows_consumer_features(&self) -> Option<&i32> {
        self.allow_windows_consumer_features.as_ref()
    }

    /// Sets the value of AllowWindowsTips
    pub fn set_allow_windows_tips(&mut self, value: i32) {
        self.allow_windows_tips = Some(value);
    }

    /// Gets the value of AllowWindowsTips
    pub fn get_allow_windows_tips(&self) -> Option<&i32> {
        self.allow_windows_tips.as_ref()
    }

    /// Sets the value of DisableCloudOptimizedContent
    pub fn set_disable_cloud_optimized_content(&mut self, value: i32) {
        self.disable_cloud_optimized_content = Some(value);
    }

    /// Gets the value of DisableCloudOptimizedContent
    pub fn get_disable_cloud_optimized_content(&self) -> Option<&i32> {
        self.disable_cloud_optimized_content.as_ref()
    }

    /// Sets the value of DisableConsumerAccountStateContent
    pub fn set_disable_consumer_account_state_content(&mut self, value: i32) {
        self.disable_consumer_account_state_content = Some(value);
    }

    /// Gets the value of DisableConsumerAccountStateContent
    pub fn get_disable_consumer_account_state_content(&self) -> Option<&i32> {
        self.disable_consumer_account_state_content.as_ref()
    }

    /// Sets the value of DoNotShowFeedbackNotifications
    pub fn set_do_not_show_feedback_notifications(&mut self, value: i32) {
        self.do_not_show_feedback_notifications = Some(value);
    }

    /// Gets the value of DoNotShowFeedbackNotifications
    pub fn get_do_not_show_feedback_notifications(&self) -> Option<&i32> {
        self.do_not_show_feedback_notifications.as_ref()
    }

    /// Sets the value of DoNotSyncBrowserSettings
    pub fn set_do_not_sync_browser_settings(&mut self, value: i32) {
        self.do_not_sync_browser_settings = Some(value);
    }

    /// Gets the value of DoNotSyncBrowserSettings
    pub fn get_do_not_sync_browser_settings(&self) -> Option<&i32> {
        self.do_not_sync_browser_settings.as_ref()
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

    /// Sets the value of PreventUsersFromTurningOnBrowserSyncing
    pub fn set_prevent_users_from_turning_on_browser_syncing(&mut self, value: i32) {
        self.prevent_users_from_turning_on_browser_syncing = Some(value);
    }

    /// Gets the value of PreventUsersFromTurningOnBrowserSyncing
    pub fn get_prevent_users_from_turning_on_browser_syncing(&self) -> Option<&i32> {
        self.prevent_users_from_turning_on_browser_syncing.as_ref()
    }

    /// Sets the value of ShowLockOnUserTile
    pub fn set_show_lock_on_user_tile(&mut self, value: i32) {
        self.show_lock_on_user_tile = Some(value);
    }

    /// Gets the value of ShowLockOnUserTile
    pub fn get_show_lock_on_user_tile(&self) -> Option<&i32> {
        self.show_lock_on_user_tile.as_ref()
    }
}

