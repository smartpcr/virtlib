// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DirectoryAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DirectoryAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "DirectoryName")]
    pub directory_name: Option<String>,
}

impl CIM_DirectoryAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            directory_name: None,
        }
    }


    /// Sets the value of DirectoryName
    pub fn set_directory_name(&mut self, value: String) {
        self.directory_name = Some(value);
    }

    /// Gets the value of DirectoryName
    pub fn get_directory_name(&self) -> Option<&String> {
        self.directory_name.as_ref()
    }
}

