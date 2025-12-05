// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ApplicationSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ApplicationSetting {

/// 
    #[serde(rename = "PackageFamilyName")]
    pub package_family_name: Option<String>,

/// 
    #[serde(rename = "SettingName")]
    pub setting_name: Option<String>,

/// 
    #[serde(rename = "SettingType")]
    pub setting_type: Option<u32>,

/// 
    #[serde(rename = "SettingValue")]
    pub setting_value: Option<String>,
}

impl MDM_ApplicationSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            package_family_name: None,
            setting_name: None,
            setting_type: None,
            setting_value: None,
        }
    }


    /// Sets the value of PackageFamilyName
    pub fn set_package_family_name(&mut self, value: String) {
        self.package_family_name = Some(value);
    }

    /// Gets the value of PackageFamilyName
    pub fn get_package_family_name(&self) -> Option<&String> {
        self.package_family_name.as_ref()
    }

    /// Sets the value of SettingName
    pub fn set_setting_name(&mut self, value: String) {
        self.setting_name = Some(value);
    }

    /// Gets the value of SettingName
    pub fn get_setting_name(&self) -> Option<&String> {
        self.setting_name.as_ref()
    }

    /// Sets the value of SettingType
    pub fn set_setting_type(&mut self, value: u32) {
        self.setting_type = Some(value);
    }

    /// Gets the value of SettingType
    pub fn get_setting_type(&self) -> Option<&u32> {
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

