// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_BindImageAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_BindImageAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,
}

impl Win32_BindImageAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            file: None,
            path: None,
        }
    }


    /// Sets the value of File
    pub fn set_file(&mut self, value: String) {
        self.file = Some(value);
    }

    /// Gets the value of File
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }
}

