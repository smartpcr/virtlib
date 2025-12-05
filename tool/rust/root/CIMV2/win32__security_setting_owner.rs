// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SecuritySettingOwner struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SecuritySettingOwner {

/// 
    #[serde(rename = "Owner")]
    pub owner: Option<Win32_SID>,

/// 
    #[serde(rename = "SecuritySetting")]
    pub security_setting: Option<Win32_SecuritySetting>,
}

impl Win32_SecuritySettingOwner {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            owner: None,
            security_setting: None,
        }
    }


    /// Sets the value of Owner
    pub fn set_owner(&mut self, value: Win32_SID) {
        self.owner = Some(value);
    }

    /// Gets the value of Owner
    pub fn get_owner(&self) -> Option<&Win32_SID> {
        self.owner.as_ref()
    }

    /// Sets the value of SecuritySetting
    pub fn set_security_setting(&mut self, value: Win32_SecuritySetting) {
        self.security_setting = Some(value);
    }

    /// Gets the value of SecuritySetting
    pub fn get_security_setting(&self) -> Option<&Win32_SecuritySetting> {
        self.security_setting.as_ref()
    }
}

