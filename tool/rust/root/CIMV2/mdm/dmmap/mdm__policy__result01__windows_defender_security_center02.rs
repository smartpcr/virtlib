// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_WindowsDefenderSecurityCenter02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_WindowsDefenderSecurityCenter02 {

/// 
    #[serde(rename = "CompanyName")]
    pub company_name: Option<String>,

/// 
    #[serde(rename = "DisableAccountProtectionUI")]
    pub disable_account_protection_ui: Option<i32>,

/// 
    #[serde(rename = "DisableAppBrowserUI")]
    pub disable_app_browser_ui: Option<i32>,

/// 
    #[serde(rename = "DisableClearTpmButton")]
    pub disable_clear_tpm_button: Option<i32>,

/// 
    #[serde(rename = "DisableDeviceSecurityUI")]
    pub disable_device_security_ui: Option<i32>,

/// 
    #[serde(rename = "DisableEnhancedNotifications")]
    pub disable_enhanced_notifications: Option<i32>,

/// 
    #[serde(rename = "DisableFamilyUI")]
    pub disable_family_ui: Option<i32>,

/// 
    #[serde(rename = "DisableHealthUI")]
    pub disable_health_ui: Option<i32>,

/// 
    #[serde(rename = "DisableNetworkUI")]
    pub disable_network_ui: Option<i32>,

/// 
    #[serde(rename = "DisableNotifications")]
    pub disable_notifications: Option<i32>,

/// 
    #[serde(rename = "DisableTpmFirmwareUpdateWarning")]
    pub disable_tpm_firmware_update_warning: Option<i32>,

/// 
    #[serde(rename = "DisableVirusUI")]
    pub disable_virus_ui: Option<i32>,

/// 
    #[serde(rename = "DisallowExploitProtectionOverride")]
    pub disallow_exploit_protection_override: Option<i32>,

/// 
    #[serde(rename = "Email")]
    pub email: Option<String>,

/// 
    #[serde(rename = "EnableCustomizedToasts")]
    pub enable_customized_toasts: Option<i32>,

/// 
    #[serde(rename = "EnableInAppCustomization")]
    pub enable_in_app_customization: Option<i32>,

/// 
    #[serde(rename = "HideRansomwareDataRecovery")]
    pub hide_ransomware_data_recovery: Option<i32>,

/// 
    #[serde(rename = "HideSecureBoot")]
    pub hide_secure_boot: Option<i32>,

/// 
    #[serde(rename = "HideTPMTroubleshooting")]
    pub hide_tpmtroubleshooting: Option<i32>,

/// 
    #[serde(rename = "HideWindowsSecurityNotificationAreaControl")]
    pub hide_windows_security_notification_area_control: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Phone")]
    pub phone: Option<String>,

/// 
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

impl MDM_Policy_Result01_WindowsDefenderSecurityCenter02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            company_name: None,
            disable_account_protection_ui: None,
            disable_app_browser_ui: None,
            disable_clear_tpm_button: None,
            disable_device_security_ui: None,
            disable_enhanced_notifications: None,
            disable_family_ui: None,
            disable_health_ui: None,
            disable_network_ui: None,
            disable_notifications: None,
            disable_tpm_firmware_update_warning: None,
            disable_virus_ui: None,
            disallow_exploit_protection_override: None,
            email: None,
            enable_customized_toasts: None,
            enable_in_app_customization: None,
            hide_ransomware_data_recovery: None,
            hide_secure_boot: None,
            hide_tpmtroubleshooting: None,
            hide_windows_security_notification_area_control: None,
            instance_id: None,
            parent_id: None,
            phone: None,
            url: None,
        }
    }


    /// Sets the value of CompanyName
    pub fn set_company_name(&mut self, value: String) {
        self.company_name = Some(value);
    }

    /// Gets the value of CompanyName
    pub fn get_company_name(&self) -> Option<&String> {
        self.company_name.as_ref()
    }

    /// Sets the value of DisableAccountProtectionUI
    pub fn set_disable_account_protection_ui(&mut self, value: i32) {
        self.disable_account_protection_ui = Some(value);
    }

    /// Gets the value of DisableAccountProtectionUI
    pub fn get_disable_account_protection_ui(&self) -> Option<&i32> {
        self.disable_account_protection_ui.as_ref()
    }

    /// Sets the value of DisableAppBrowserUI
    pub fn set_disable_app_browser_ui(&mut self, value: i32) {
        self.disable_app_browser_ui = Some(value);
    }

    /// Gets the value of DisableAppBrowserUI
    pub fn get_disable_app_browser_ui(&self) -> Option<&i32> {
        self.disable_app_browser_ui.as_ref()
    }

    /// Sets the value of DisableClearTpmButton
    pub fn set_disable_clear_tpm_button(&mut self, value: i32) {
        self.disable_clear_tpm_button = Some(value);
    }

    /// Gets the value of DisableClearTpmButton
    pub fn get_disable_clear_tpm_button(&self) -> Option<&i32> {
        self.disable_clear_tpm_button.as_ref()
    }

    /// Sets the value of DisableDeviceSecurityUI
    pub fn set_disable_device_security_ui(&mut self, value: i32) {
        self.disable_device_security_ui = Some(value);
    }

    /// Gets the value of DisableDeviceSecurityUI
    pub fn get_disable_device_security_ui(&self) -> Option<&i32> {
        self.disable_device_security_ui.as_ref()
    }

    /// Sets the value of DisableEnhancedNotifications
    pub fn set_disable_enhanced_notifications(&mut self, value: i32) {
        self.disable_enhanced_notifications = Some(value);
    }

    /// Gets the value of DisableEnhancedNotifications
    pub fn get_disable_enhanced_notifications(&self) -> Option<&i32> {
        self.disable_enhanced_notifications.as_ref()
    }

    /// Sets the value of DisableFamilyUI
    pub fn set_disable_family_ui(&mut self, value: i32) {
        self.disable_family_ui = Some(value);
    }

    /// Gets the value of DisableFamilyUI
    pub fn get_disable_family_ui(&self) -> Option<&i32> {
        self.disable_family_ui.as_ref()
    }

    /// Sets the value of DisableHealthUI
    pub fn set_disable_health_ui(&mut self, value: i32) {
        self.disable_health_ui = Some(value);
    }

    /// Gets the value of DisableHealthUI
    pub fn get_disable_health_ui(&self) -> Option<&i32> {
        self.disable_health_ui.as_ref()
    }

    /// Sets the value of DisableNetworkUI
    pub fn set_disable_network_ui(&mut self, value: i32) {
        self.disable_network_ui = Some(value);
    }

    /// Gets the value of DisableNetworkUI
    pub fn get_disable_network_ui(&self) -> Option<&i32> {
        self.disable_network_ui.as_ref()
    }

    /// Sets the value of DisableNotifications
    pub fn set_disable_notifications(&mut self, value: i32) {
        self.disable_notifications = Some(value);
    }

    /// Gets the value of DisableNotifications
    pub fn get_disable_notifications(&self) -> Option<&i32> {
        self.disable_notifications.as_ref()
    }

    /// Sets the value of DisableTpmFirmwareUpdateWarning
    pub fn set_disable_tpm_firmware_update_warning(&mut self, value: i32) {
        self.disable_tpm_firmware_update_warning = Some(value);
    }

    /// Gets the value of DisableTpmFirmwareUpdateWarning
    pub fn get_disable_tpm_firmware_update_warning(&self) -> Option<&i32> {
        self.disable_tpm_firmware_update_warning.as_ref()
    }

    /// Sets the value of DisableVirusUI
    pub fn set_disable_virus_ui(&mut self, value: i32) {
        self.disable_virus_ui = Some(value);
    }

    /// Gets the value of DisableVirusUI
    pub fn get_disable_virus_ui(&self) -> Option<&i32> {
        self.disable_virus_ui.as_ref()
    }

    /// Sets the value of DisallowExploitProtectionOverride
    pub fn set_disallow_exploit_protection_override(&mut self, value: i32) {
        self.disallow_exploit_protection_override = Some(value);
    }

    /// Gets the value of DisallowExploitProtectionOverride
    pub fn get_disallow_exploit_protection_override(&self) -> Option<&i32> {
        self.disallow_exploit_protection_override.as_ref()
    }

    /// Sets the value of Email
    pub fn set_email(&mut self, value: String) {
        self.email = Some(value);
    }

    /// Gets the value of Email
    pub fn get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }

    /// Sets the value of EnableCustomizedToasts
    pub fn set_enable_customized_toasts(&mut self, value: i32) {
        self.enable_customized_toasts = Some(value);
    }

    /// Gets the value of EnableCustomizedToasts
    pub fn get_enable_customized_toasts(&self) -> Option<&i32> {
        self.enable_customized_toasts.as_ref()
    }

    /// Sets the value of EnableInAppCustomization
    pub fn set_enable_in_app_customization(&mut self, value: i32) {
        self.enable_in_app_customization = Some(value);
    }

    /// Gets the value of EnableInAppCustomization
    pub fn get_enable_in_app_customization(&self) -> Option<&i32> {
        self.enable_in_app_customization.as_ref()
    }

    /// Sets the value of HideRansomwareDataRecovery
    pub fn set_hide_ransomware_data_recovery(&mut self, value: i32) {
        self.hide_ransomware_data_recovery = Some(value);
    }

    /// Gets the value of HideRansomwareDataRecovery
    pub fn get_hide_ransomware_data_recovery(&self) -> Option<&i32> {
        self.hide_ransomware_data_recovery.as_ref()
    }

    /// Sets the value of HideSecureBoot
    pub fn set_hide_secure_boot(&mut self, value: i32) {
        self.hide_secure_boot = Some(value);
    }

    /// Gets the value of HideSecureBoot
    pub fn get_hide_secure_boot(&self) -> Option<&i32> {
        self.hide_secure_boot.as_ref()
    }

    /// Sets the value of HideTPMTroubleshooting
    pub fn set_hide_tpmtroubleshooting(&mut self, value: i32) {
        self.hide_tpmtroubleshooting = Some(value);
    }

    /// Gets the value of HideTPMTroubleshooting
    pub fn get_hide_tpmtroubleshooting(&self) -> Option<&i32> {
        self.hide_tpmtroubleshooting.as_ref()
    }

    /// Sets the value of HideWindowsSecurityNotificationAreaControl
    pub fn set_hide_windows_security_notification_area_control(&mut self, value: i32) {
        self.hide_windows_security_notification_area_control = Some(value);
    }

    /// Gets the value of HideWindowsSecurityNotificationAreaControl
    pub fn get_hide_windows_security_notification_area_control(&self) -> Option<&i32> {
        self.hide_windows_security_notification_area_control.as_ref()
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

    /// Sets the value of Phone
    pub fn set_phone(&mut self, value: String) {
        self.phone = Some(value);
    }

    /// Gets the value of Phone
    pub fn get_phone(&self) -> Option<&String> {
        self.phone.as_ref()
    }

    /// Sets the value of URL
    pub fn set_url(&mut self, value: String) {
        self.url = Some(value);
    }

    /// Gets the value of URL
    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}

