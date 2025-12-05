// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ElementSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ElementSettingData {

/// 
    #[serde(rename = "IsCurrent")]
    pub is_current: Option<u16>,

/// 
    #[serde(rename = "IsDefault")]
    pub is_default: Option<u16>,

/// 
    #[serde(rename = "IsNext")]
    pub is_next: Option<u16>,

/// 
    #[serde(rename = "ManagedElement")]
    pub managed_element: Option<CIM_ManagedElement>,

/// 
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
    pub fn set_is_current(&mut self, value: u16) {
        self.is_current = Some(value);
    }

    /// Gets the value of IsCurrent
    pub fn get_is_current(&self) -> Option<&u16> {
        self.is_current.as_ref()
    }

    /// Sets the value of IsDefault
    pub fn set_is_default(&mut self, value: u16) {
        self.is_default = Some(value);
    }

    /// Gets the value of IsDefault
    pub fn get_is_default(&self) -> Option<&u16> {
        self.is_default.as_ref()
    }

    /// Sets the value of IsNext
    pub fn set_is_next(&mut self, value: u16) {
        self.is_next = Some(value);
    }

    /// Gets the value of IsNext
    pub fn get_is_next(&self) -> Option<&u16> {
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

