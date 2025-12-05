// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShortcutFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShortcutFile {
    #[serde(flatten)]
    pub base: CIM_DataFile,

/// 
    #[serde(rename = "Target")]
    pub target: Option<String>,
}

impl Win32_ShortcutFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DataFile::new(),
            target: None,
        }
    }


    /// Sets the value of Target
    pub fn set_target(&mut self, value: String) {
        self.target = Some(value);
    }

    /// Gets the value of Target
    pub fn get_target(&self) -> Option<&String> {
        self.target.as_ref()
    }
}

