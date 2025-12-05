// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerSettingDataIndex struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerSettingDataIndex {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "SettingIndexValue")]
    pub setting_index_value: Option<u32>,
}

impl Win32_PowerSettingDataIndex {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            setting_index_value: None,
        }
    }


    /// Sets the value of SettingIndexValue
    pub fn set_setting_index_value(&mut self, value: u32) {
        self.setting_index_value = Some(value);
    }

    /// Gets the value of SettingIndexValue
    pub fn get_setting_index_value(&self) -> Option<&u32> {
        self.setting_index_value.as_ref()
    }
}

