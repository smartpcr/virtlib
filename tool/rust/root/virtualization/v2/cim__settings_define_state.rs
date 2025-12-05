// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SettingsDefineState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SettingsDefineState {

/// The managed element.
    #[serde(rename = "ManagedElement")]
    pub managed_element: Option<CIM_ManagedElement>,

/// The SettingData object that provides additional information about the current state and configuration of the ManagedElement.
    #[serde(rename = "SettingData")]
    pub setting_data: Option<CIM_SettingData>,
}

impl CIM_SettingsDefineState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            managed_element: None,
            setting_data: None,
        }
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

