// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VssComponentSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VssComponentSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u16>,
}

impl Msvm_VssComponentSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            enabled_state: None,
        }
    }


    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u16) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u16> {
        self.enabled_state.as_ref()
    }
}

impl Msvm_VssComponentSettingData {
    /// Gets the related Msvm_VirtualSystemSettingData object(s)
    pub fn get_related__virtual_system_setting_data(&self) -> Result<Msvm_VirtualSystemSettingData, WmiError> {
        self.get_related("Msvm_VirtualSystemSettingData")
    }

}

