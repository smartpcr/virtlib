// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SettingCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SettingCheck {

/// 
    #[serde(rename = "Check")]
    pub check: Option<CIM_Check>,

/// 
    #[serde(rename = "Setting")]
    pub setting: Option<CIM_Setting>,
}

impl Win32_SettingCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            check: None,
            setting: None,
        }
    }


    /// Sets the value of Check
    pub fn set_check(&mut self, value: CIM_Check) {
        self.check = Some(value);
    }

    /// Gets the value of Check
    pub fn get_check(&self) -> Option<&CIM_Check> {
        self.check.as_ref()
    }

    /// Sets the value of Setting
    pub fn set_setting(&mut self, value: CIM_Setting) {
        self.setting = Some(value);
    }

    /// Gets the value of Setting
    pub fn get_setting(&self) -> Option<&CIM_Setting> {
        self.setting.as_ref()
    }
}

