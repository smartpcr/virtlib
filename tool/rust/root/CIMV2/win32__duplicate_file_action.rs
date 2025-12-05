// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DuplicateFileAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DuplicateFileAction {
    #[serde(flatten)]
    pub base: CIM_CopyFileAction,

/// 
    #[serde(rename = "FileKey")]
    pub file_key: Option<String>,
}

impl Win32_DuplicateFileAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_CopyFileAction::new(),
            file_key: None,
        }
    }


    /// Sets the value of FileKey
    pub fn set_file_key(&mut self, value: String) {
        self.file_key = Some(value);
    }

    /// Gets the value of FileKey
    pub fn get_file_key(&self) -> Option<&String> {
        self.file_key.as_ref()
    }
}

