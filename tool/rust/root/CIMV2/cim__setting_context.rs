// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SettingContext struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SettingContext {

/// 
    #[serde(rename = "Context")]
    pub context: Option<CIM_Configuration>,

/// 
    #[serde(rename = "Setting")]
    pub setting: Option<CIM_Setting>,
}

impl CIM_SettingContext {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            context: None,
            setting: None,
        }
    }


    /// Sets the value of Context
    pub fn set_context(&mut self, value: CIM_Configuration) {
        self.context = Some(value);
    }

    /// Gets the value of Context
    pub fn get_context(&self) -> Option<&CIM_Configuration> {
        self.context.as_ref()
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

