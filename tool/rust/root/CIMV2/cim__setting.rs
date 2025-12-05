// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Setting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Setting {

/// 
    #[serde(rename = "Caption")]
    pub caption: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "SettingID")]
    pub setting_id: Option<String>,
}

impl CIM_Setting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            caption: None,
            description: None,
            setting_id: None,
        }
    }


    /// Sets the value of Caption
    pub fn set_caption(&mut self, value: String) {
        self.caption = Some(value);
    }

    /// Gets the value of Caption
    pub fn get_caption(&self) -> Option<&String> {
        self.caption.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of SettingID
    pub fn set_setting_id(&mut self, value: String) {
        self.setting_id = Some(value);
    }

    /// Gets the value of SettingID
    pub fn get_setting_id(&self) -> Option<&String> {
        self.setting_id.as_ref()
    }
}

