// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.power
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PowerSettingElementSettingDataIndex struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PowerSettingElementSettingDataIndex {
    #[serde(flatten)]
    pub base: CIM_ElementSettingData,

/// 
    #[serde(rename = "IsACSetting")]
    pub is_acsetting: Option<u16>,
}

impl Win32_PowerSettingElementSettingDataIndex {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ElementSettingData::new(),
            is_acsetting: None,
        }
    }


    /// Sets the value of IsACSetting
    pub fn set_is_acsetting(&mut self, value: u16) {
        self.is_acsetting = Some(value);
    }

    /// Gets the value of IsACSetting
    pub fn get_is_acsetting(&self) -> Option<&u16> {
        self.is_acsetting.as_ref()
    }
}

