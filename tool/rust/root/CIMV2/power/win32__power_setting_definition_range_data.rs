// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerSettingDefinitionRangeData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerSettingDefinitionRangeData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "SettingValue")]
    pub setting_value: Option<u32>,
}

impl Win32_PowerSettingDefinitionRangeData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            setting_value: None,
        }
    }


    /// Sets the value of SettingValue
    pub fn set_setting_value(&mut self, value: u32) {
        self.setting_value = Some(value);
    }

    /// Gets the value of SettingValue
    pub fn get_setting_value(&self) -> Option<&u32> {
        self.setting_value.as_ref()
    }
}

