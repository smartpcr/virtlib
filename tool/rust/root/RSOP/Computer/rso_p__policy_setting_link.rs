// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSoP_PolicySettingLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSoP_PolicySettingLink {

/// 
    #[serde(rename = "setting")]
    pub setting: Option<RSOP_PolicySetting>,

/// 
    #[serde(rename = "status")]
    pub status: Option<RSoP_PolicySettingStatus>,
}

impl RSoP_PolicySettingLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            setting: None,
            status: None,
        }
    }


    /// Sets the value of setting
    pub fn set_setting(&mut self, value: RSOP_PolicySetting) {
        self.setting = Some(value);
    }

    /// Gets the value of setting
    pub fn get_setting(&self) -> Option<&RSOP_PolicySetting> {
        self.setting.as_ref()
    }

    /// Sets the value of status
    pub fn set_status(&mut self, value: RSoP_PolicySettingStatus) {
        self.status = Some(value);
    }

    /// Gets the value of status
    pub fn get_status(&self) -> Option<&RSoP_PolicySettingStatus> {
        self.status.as_ref()
    }
}

