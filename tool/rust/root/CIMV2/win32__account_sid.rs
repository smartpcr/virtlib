// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_AccountSID struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_AccountSID {

/// 
    #[serde(rename = "Element")]
    pub element: Option<Win32_Account>,

/// 
    #[serde(rename = "Setting")]
    pub setting: Option<Win32_SID>,
}

impl Win32_AccountSID {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            element: None,
            setting: None,
        }
    }


    /// Sets the value of Element
    pub fn set_element(&mut self, value: Win32_Account) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&Win32_Account> {
        self.element.as_ref()
    }

    /// Sets the value of Setting
    pub fn set_setting(&mut self, value: Win32_SID) {
        self.setting = Some(value);
    }

    /// Gets the value of Setting
    pub fn get_setting(&self) -> Option<&Win32_SID> {
        self.setting.as_ref()
    }
}

