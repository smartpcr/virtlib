// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RemoveFileAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RemoveFileAction {
    #[serde(flatten)]
    pub base: CIM_FileAction,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,
}

impl CIM_RemoveFileAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FileAction::new(),
            file: None,
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
}

