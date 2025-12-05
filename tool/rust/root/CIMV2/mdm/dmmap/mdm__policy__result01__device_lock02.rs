// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_DeviceLock02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_DeviceLock02 {

/// 
    #[serde(rename = "AllowScreenTimeoutWhileLockedUserConfig")]
    pub allow_screen_timeout_while_locked_user_config: Option<i32>,

/// 
    #[serde(rename = "AllowSimpleDevicePassword")]
    pub allow_simple_device_password: Option<i32>,

/// 
    #[serde(rename = "AlphanumericDevicePasswordRequired")]
    pub alphanumeric_device_password_required: Option<i32>,

/// 
    #[serde(rename = "DevicePasswordEnabled")]
    pub device_password_enabled: Option<i32>,

/// 
    #[serde(rename = "DevicePasswordExpiration")]
    pub device_password_expiration: Option<i32>,

/// 
    #[serde(rename = "DevicePasswordHistory")]
    pub device_password_history: Option<i32>,

/// 
    #[serde(rename = "EnforceLockScreenAndLogonImage")]
    pub enforce_lock_screen_and_logon_image: Option<String>,

/// 
    #[serde(rename = "EnforceLockScreenProvider")]
    pub enforce_lock_screen_provider: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MaxDevicePasswordFailedAttempts")]
    pub max_device_password_failed_attempts: Option<i32>,

/// 
    #[serde(rename = "MaxInactivityTimeDeviceLock")]
    pub max_inactivity_time_device_lock: Option<i32>,

/// 
    #[serde(rename = "MinDevicePasswordComplexCharacters")]
    pub min_device_password_complex_characters: Option<i32>,

/// 
    #[serde(rename = "MinDevicePasswordLength")]
    pub min_device_password_length: Option<i32>,

/// 
    #[serde(rename = "MinimumPasswordAge")]
    pub minimum_password_age: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventEnablingLockScreenCamera")]
    pub prevent_enabling_lock_screen_camera: Option<String>,

/// 
    #[serde(rename = "PreventLockScreenSlideShow")]
    pub prevent_lock_screen_slide_show: Option<String>,

/// 
    #[serde(rename = "ScreenTimeoutWhileLocked")]
    pub screen_timeout_while_locked: Option<i32>,
}

impl MDM_Policy_Result01_DeviceLock02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_screen_timeout_while_locked_user_config: None,
            allow_simple_device_password: None,
            alphanumeric_device_password_required: None,
            device_password_enabled: None,
            device_password_expiration: None,
            device_password_history: None,
            enforce_lock_screen_and_logon_image: None,
            enforce_lock_screen_provider: None,
            instance_id: None,
            max_device_password_failed_attempts: None,
            max_inactivity_time_device_lock: None,
            min_device_password_complex_characters: None,
            min_device_password_length: None,
            minimum_password_age: None,
            parent_id: None,
            prevent_enabling_lock_screen_camera: None,
            prevent_lock_screen_slide_show: None,
            screen_timeout_while_locked: None,
        }
    }


    /// Sets the value of AllowScreenTimeoutWhileLockedUserConfig
    pub fn set_allow_screen_timeout_while_locked_user_config(&mut self, value: i32) {
        self.allow_screen_timeout_while_locked_user_config = Some(value);
    }

    /// Gets the value of AllowScreenTimeoutWhileLockedUserConfig
    pub fn get_allow_screen_timeout_while_locked_user_config(&self) -> Option<&i32> {
        self.allow_screen_timeout_while_locked_user_config.as_ref()
    }

    /// Sets the value of AllowSimpleDevicePassword
    pub fn set_allow_simple_device_password(&mut self, value: i32) {
        self.allow_simple_device_password = Some(value);
    }

    /// Gets the value of AllowSimpleDevicePassword
    pub fn get_allow_simple_device_password(&self) -> Option<&i32> {
        self.allow_simple_device_password.as_ref()
    }

    /// Sets the value of AlphanumericDevicePasswordRequired
    pub fn set_alphanumeric_device_password_required(&mut self, value: i32) {
        self.alphanumeric_device_password_required = Some(value);
    }

    /// Gets the value of AlphanumericDevicePasswordRequired
    pub fn get_alphanumeric_device_password_required(&self) -> Option<&i32> {
        self.alphanumeric_device_password_required.as_ref()
    }

    /// Sets the value of DevicePasswordEnabled
    pub fn set_device_password_enabled(&mut self, value: i32) {
        self.device_password_enabled = Some(value);
    }

    /// Gets the value of DevicePasswordEnabled
    pub fn get_device_password_enabled(&self) -> Option<&i32> {
        self.device_password_enabled.as_ref()
    }

    /// Sets the value of DevicePasswordExpiration
    pub fn set_device_password_expiration(&mut self, value: i32) {
        self.device_password_expiration = Some(value);
    }

    /// Gets the value of DevicePasswordExpiration
    pub fn get_device_password_expiration(&self) -> Option<&i32> {
        self.device_password_expiration.as_ref()
    }

    /// Sets the value of DevicePasswordHistory
    pub fn set_device_password_history(&mut self, value: i32) {
        self.device_password_history = Some(value);
    }

    /// Gets the value of DevicePasswordHistory
    pub fn get_device_password_history(&self) -> Option<&i32> {
        self.device_password_history.as_ref()
    }

    /// Sets the value of EnforceLockScreenAndLogonImage
    pub fn set_enforce_lock_screen_and_logon_image(&mut self, value: String) {
        self.enforce_lock_screen_and_logon_image = Some(value);
    }

    /// Gets the value of EnforceLockScreenAndLogonImage
    pub fn get_enforce_lock_screen_and_logon_image(&self) -> Option<&String> {
        self.enforce_lock_screen_and_logon_image.as_ref()
    }

    /// Sets the value of EnforceLockScreenProvider
    pub fn set_enforce_lock_screen_provider(&mut self, value: String) {
        self.enforce_lock_screen_provider = Some(value);
    }

    /// Gets the value of EnforceLockScreenProvider
    pub fn get_enforce_lock_screen_provider(&self) -> Option<&String> {
        self.enforce_lock_screen_provider.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MaxDevicePasswordFailedAttempts
    pub fn set_max_device_password_failed_attempts(&mut self, value: i32) {
        self.max_device_password_failed_attempts = Some(value);
    }

    /// Gets the value of MaxDevicePasswordFailedAttempts
    pub fn get_max_device_password_failed_attempts(&self) -> Option<&i32> {
        self.max_device_password_failed_attempts.as_ref()
    }

    /// Sets the value of MaxInactivityTimeDeviceLock
    pub fn set_max_inactivity_time_device_lock(&mut self, value: i32) {
        self.max_inactivity_time_device_lock = Some(value);
    }

    /// Gets the value of MaxInactivityTimeDeviceLock
    pub fn get_max_inactivity_time_device_lock(&self) -> Option<&i32> {
        self.max_inactivity_time_device_lock.as_ref()
    }

    /// Sets the value of MinDevicePasswordComplexCharacters
    pub fn set_min_device_password_complex_characters(&mut self, value: i32) {
        self.min_device_password_complex_characters = Some(value);
    }

    /// Gets the value of MinDevicePasswordComplexCharacters
    pub fn get_min_device_password_complex_characters(&self) -> Option<&i32> {
        self.min_device_password_complex_characters.as_ref()
    }

    /// Sets the value of MinDevicePasswordLength
    pub fn set_min_device_password_length(&mut self, value: i32) {
        self.min_device_password_length = Some(value);
    }

    /// Gets the value of MinDevicePasswordLength
    pub fn get_min_device_password_length(&self) -> Option<&i32> {
        self.min_device_password_length.as_ref()
    }

    /// Sets the value of MinimumPasswordAge
    pub fn set_minimum_password_age(&mut self, value: i32) {
        self.minimum_password_age = Some(value);
    }

    /// Gets the value of MinimumPasswordAge
    pub fn get_minimum_password_age(&self) -> Option<&i32> {
        self.minimum_password_age.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PreventEnablingLockScreenCamera
    pub fn set_prevent_enabling_lock_screen_camera(&mut self, value: String) {
        self.prevent_enabling_lock_screen_camera = Some(value);
    }

    /// Gets the value of PreventEnablingLockScreenCamera
    pub fn get_prevent_enabling_lock_screen_camera(&self) -> Option<&String> {
        self.prevent_enabling_lock_screen_camera.as_ref()
    }

    /// Sets the value of PreventLockScreenSlideShow
    pub fn set_prevent_lock_screen_slide_show(&mut self, value: String) {
        self.prevent_lock_screen_slide_show = Some(value);
    }

    /// Gets the value of PreventLockScreenSlideShow
    pub fn get_prevent_lock_screen_slide_show(&self) -> Option<&String> {
        self.prevent_lock_screen_slide_show.as_ref()
    }

    /// Sets the value of ScreenTimeoutWhileLocked
    pub fn set_screen_timeout_while_locked(&mut self, value: i32) {
        self.screen_timeout_while_locked = Some(value);
    }

    /// Gets the value of ScreenTimeoutWhileLocked
    pub fn get_screen_timeout_while_locked(&self) -> Option<&i32> {
        self.screen_timeout_while_locked.as_ref()
    }
}

