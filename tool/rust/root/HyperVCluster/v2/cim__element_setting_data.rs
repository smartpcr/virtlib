// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ElementSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementSettingData {

/// An enumerated integer indicating that the referenced setting is currently being used in the operation of the element, or that this information is unknown.
    #[serde(rename = "IsCurrent")]
    pub is_current: Option<ElementSettingData_IsCurrent>,

/// An enumerated integer indicating that the referenced setting is a default setting for the element, or that this information is unknown.
    #[serde(rename = "IsDefault")]
    pub is_default: Option<ElementSettingData_IsDefault>,

/// An enumerated integer indicating whether or not the referenced setting is the next setting to be applied. For example, the application could take place on a re-initialization, reset, reconfiguration request. This could be a permanent setting, or a setting used only one time, as indicated by the flag. If it is a permanent setting then the setting is applied every time the managed element reinitializes, until this flag is manually reset. However, if it is single use, then the flag is automatically cleared after the settings are applied. Also note that if this flag is specified (i.e. set to value other than "Unknown"), then this takes precedence over any SettingData that may have been specified as Default. For example: If the managed element is a computer system, and the value of this flag is "Is Next", then the setting will be effective next time the system resets. And, unless this flag is changed, it will persist for subsequent system resets. However, if this flag is set to "Is Next For Single Use", then this setting will only be used once and the flag would be reset after that to "Is Not Next". So, in the above example, if the system reboots in a quick succession, the setting will not be used at the second reboot.
    #[serde(rename = "IsNext")]
    pub is_next: Option<ElementSettingData_IsNext>,

/// The managed element.
    #[serde(rename = "ManagedElement")]
    pub managed_element: Option<CIM_ManagedElement>,

/// The SettingData object associated with the element.
    #[serde(rename = "SettingData")]
    pub setting_data: Option<CIM_SettingData>,
}

impl CIM_ElementSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            is_current: None,
            is_default: None,
            is_next: None,
            managed_element: None,
            setting_data: None,
        }
    }


    /// Sets the value of IsCurrent
    pub fn set_is_current(&mut self, value: ElementSettingData_IsCurrent) {
        self.is_current = Some(value);
    }

    /// Gets the value of IsCurrent
    pub fn get_is_current(&self) -> Option<&ElementSettingData_IsCurrent> {
        self.is_current.as_ref()
    }

    /// Sets the value of IsDefault
    pub fn set_is_default(&mut self, value: ElementSettingData_IsDefault) {
        self.is_default = Some(value);
    }

    /// Gets the value of IsDefault
    pub fn get_is_default(&self) -> Option<&ElementSettingData_IsDefault> {
        self.is_default.as_ref()
    }

    /// Sets the value of IsNext
    pub fn set_is_next(&mut self, value: ElementSettingData_IsNext) {
        self.is_next = Some(value);
    }

    /// Gets the value of IsNext
    pub fn get_is_next(&self) -> Option<&ElementSettingData_IsNext> {
        self.is_next.as_ref()
    }

    /// Sets the value of ManagedElement
    pub fn set_managed_element(&mut self, value: CIM_ManagedElement) {
        self.managed_element = Some(value);
    }

    /// Gets the value of ManagedElement
    pub fn get_managed_element(&self) -> Option<&CIM_ManagedElement> {
        self.managed_element.as_ref()
    }

    /// Sets the value of SettingData
    pub fn set_setting_data(&mut self, value: CIM_SettingData) {
        self.setting_data = Some(value);
    }

    /// Gets the value of SettingData
    pub fn get_setting_data(&self) -> Option<&CIM_SettingData> {
        self.setting_data.as_ref()
    }
}

