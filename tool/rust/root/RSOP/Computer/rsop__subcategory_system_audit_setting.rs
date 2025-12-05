// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_SubcategorySystemAuditSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_SubcategorySystemAuditSetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "SettingValue")]
    pub setting_value: Option<u32>,

/// 
    #[serde(rename = "SubcategoryGuid")]
    pub subcategory_guid: Option<String>,

/// 
    #[serde(rename = "SubcategoryName")]
    pub subcategory_name: Option<String>,
}

impl RSOP_SubcategorySystemAuditSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            setting_value: None,
            subcategory_guid: None,
            subcategory_name: None,
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

    /// Sets the value of SubcategoryGuid
    pub fn set_subcategory_guid(&mut self, value: String) {
        self.subcategory_guid = Some(value);
    }

    /// Gets the value of SubcategoryGuid
    pub fn get_subcategory_guid(&self) -> Option<&String> {
        self.subcategory_guid.as_ref()
    }

    /// Sets the value of SubcategoryName
    pub fn set_subcategory_name(&mut self, value: String) {
        self.subcategory_name = Some(value);
    }

    /// Gets the value of SubcategoryName
    pub fn get_subcategory_name(&self) -> Option<&String> {
        self.subcategory_name.as_ref()
    }
}

