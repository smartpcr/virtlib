// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ConfigurationItem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationItem {

/// Is setting valid
    #[serde(rename = "IsValid")]
    pub is_valid: Option<bool>,

/// Setting name
    #[serde(rename = "SettingName")]
    pub setting_name: Option<String>,

/// Setting source
    #[serde(rename = "SettingSource")]
    pub setting_source: Option<String>,

/// Setting value
    #[serde(rename = "SettingValue")]
    pub setting_value: Option<String>,
}

impl ConfigurationItem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            is_valid: None,
            setting_name: None,
            setting_source: None,
            setting_value: None,
        }
    }


    /// Sets the value of IsValid
    pub fn set_is_valid(&mut self, value: bool) {
        self.is_valid = Some(value);
    }

    /// Gets the value of IsValid
    pub fn get_is_valid(&self) -> Option<&bool> {
        self.is_valid.as_ref()
    }

    /// Sets the value of SettingName
    pub fn set_setting_name(&mut self, value: String) {
        self.setting_name = Some(value);
    }

    /// Gets the value of SettingName
    pub fn get_setting_name(&self) -> Option<&String> {
        self.setting_name.as_ref()
    }

    /// Sets the value of SettingSource
    pub fn set_setting_source(&mut self, value: String) {
        self.setting_source = Some(value);
    }

    /// Gets the value of SettingSource
    pub fn get_setting_source(&self) -> Option<&String> {
        self.setting_source.as_ref()
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

