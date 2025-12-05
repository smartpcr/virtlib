// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_SecuritySettingStringBlocked struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_SecuritySettingStringBlocked {
    #[serde(flatten)]
    pub base: RSOP_SecuritySettingsBlocked,

/// 
    #[serde(rename = "KeyName")]
    pub key_name: Option<String>,

/// 
    #[serde(rename = "Setting")]
    pub setting: Option<String>,
}

impl RSOP_SecuritySettingStringBlocked {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_SecuritySettingsBlocked::new(),
            key_name: None,
            setting: None,
        }
    }


    /// Sets the value of KeyName
    pub fn set_key_name(&mut self, value: String) {
        self.key_name = Some(value);
    }

    /// Gets the value of KeyName
    pub fn get_key_name(&self) -> Option<&String> {
        self.key_name.as_ref()
    }

    /// Sets the value of Setting
    pub fn set_setting(&mut self, value: String) {
        self.setting = Some(value);
    }

    /// Gets the value of Setting
    pub fn get_setting(&self) -> Option<&String> {
        self.setting.as_ref()
    }
}

