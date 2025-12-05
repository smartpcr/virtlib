// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_GlobalSaclSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_GlobalSaclSetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "SettingType")]
    pub setting_type: Option<String>,

/// 
    #[serde(rename = "SettingValue")]
    pub setting_value: Option<String>,
}

impl RSOP_GlobalSaclSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            setting_type: None,
            setting_value: None,
        }
    }


    /// Sets the value of SettingType
    pub fn set_setting_type(&mut self, value: String) {
        self.setting_type = Some(value);
    }

    /// Gets the value of SettingType
    pub fn get_setting_type(&self) -> Option<&String> {
        self.setting_type.as_ref()
    }

    /// Sets the value of SettingValue
    pub fn set_setting_value(&mut self, value: String) {
        self.setting_value = Some(value);
    }

    /// Gets the value of SettingValue
    pub fn get_setting_value(&self) -> Option<&String> {
        self.setting_value.as_ref()
    }
}

