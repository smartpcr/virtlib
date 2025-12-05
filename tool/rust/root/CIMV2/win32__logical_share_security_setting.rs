// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LogicalShareSecuritySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LogicalShareSecuritySetting {
    #[serde(flatten)]
    pub base: Win32_SecuritySetting,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl Win32_LogicalShareSecuritySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_SecuritySetting::new(),
            name: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

