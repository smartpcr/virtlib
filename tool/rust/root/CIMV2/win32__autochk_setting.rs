// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_AutochkSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_AutochkSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "UserInputDelay")]
    pub user_input_delay: Option<u32>,
}

impl Win32_AutochkSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            user_input_delay: None,
        }
    }


    /// Sets the value of UserInputDelay
    pub fn set_user_input_delay(&mut self, value: u32) {
        self.user_input_delay = Some(value);
    }

    /// Gets the value of UserInputDelay
    pub fn get_user_input_delay(&self) -> Option<&u32> {
        self.user_input_delay.as_ref()
    }
}

