// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_ErrorReporting02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_ErrorReporting02 {

/// 
    #[serde(rename = "CustomizeConsentSettings")]
    pub customize_consent_settings: Option<String>,

/// 
    #[serde(rename = "DisableWindowsErrorReporting")]
    pub disable_windows_error_reporting: Option<String>,

/// 
    #[serde(rename = "DisplayErrorNotification")]
    pub display_error_notification: Option<String>,

/// 
    #[serde(rename = "DoNotSendAdditionalData")]
    pub do_not_send_additional_data: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PreventCriticalErrorDisplay")]
    pub prevent_critical_error_display: Option<String>,
}

impl MDM_Policy_Config01_ErrorReporting02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            customize_consent_settings: None,
            disable_windows_error_reporting: None,
            display_error_notification: None,
            do_not_send_additional_data: None,
            instance_id: None,
            parent_id: None,
            prevent_critical_error_display: None,
        }
    }


    /// Sets the value of CustomizeConsentSettings
    pub fn set_customize_consent_settings(&mut self, value: String) {
        self.customize_consent_settings = Some(value);
    }

    /// Gets the value of CustomizeConsentSettings
    pub fn get_customize_consent_settings(&self) -> Option<&String> {
        self.customize_consent_settings.as_ref()
    }

    /// Sets the value of DisableWindowsErrorReporting
    pub fn set_disable_windows_error_reporting(&mut self, value: String) {
        self.disable_windows_error_reporting = Some(value);
    }

    /// Gets the value of DisableWindowsErrorReporting
    pub fn get_disable_windows_error_reporting(&self) -> Option<&String> {
        self.disable_windows_error_reporting.as_ref()
    }

    /// Sets the value of DisplayErrorNotification
    pub fn set_display_error_notification(&mut self, value: String) {
        self.display_error_notification = Some(value);
    }

    /// Gets the value of DisplayErrorNotification
    pub fn get_display_error_notification(&self) -> Option<&String> {
        self.display_error_notification.as_ref()
    }

    /// Sets the value of DoNotSendAdditionalData
    pub fn set_do_not_send_additional_data(&mut self, value: String) {
        self.do_not_send_additional_data = Some(value);
    }

    /// Gets the value of DoNotSendAdditionalData
    pub fn get_do_not_send_additional_data(&self) -> Option<&String> {
        self.do_not_send_additional_data.as_ref()
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

    /// Sets the value of PreventCriticalErrorDisplay
    pub fn set_prevent_critical_error_display(&mut self, value: String) {
        self.prevent_critical_error_display = Some(value);
    }

    /// Gets the value of PreventCriticalErrorDisplay
    pub fn get_prevent_critical_error_display(&self) -> Option<&String> {
        self.prevent_critical_error_display.as_ref()
    }
}

