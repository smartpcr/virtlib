// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_DiskMergeSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_DiskMergeSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "EnabledState")]
    pub enabled_state: Option<u32>,
}

impl Msvm_DiskMergeSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            enabled_state: None,
        }
    }


    /// Sets the value of EnabledState
    pub fn set_enabled_state(&mut self, value: u32) {
        self.enabled_state = Some(value);
    }

    /// Gets the value of EnabledState
    pub fn get_enabled_state(&self) -> Option<&u32> {
        self.enabled_state.as_ref()
    }
}

impl Msvm_DiskMergeSettingData {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

