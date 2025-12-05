// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DirectorySpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DirectorySpecification {
    #[serde(flatten)]
    pub base: CIM_DirectorySpecification,

/// 
    #[serde(rename = "DefaultDir")]
    pub default_dir: Option<String>,

/// 
    #[serde(rename = "Directory")]
    pub directory: Option<String>,
}

impl Win32_DirectorySpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DirectorySpecification::new(),
            default_dir: None,
            directory: None,
        }
    }


    /// Sets the value of DefaultDir
    pub fn set_default_dir(&mut self, value: String) {
        self.default_dir = Some(value);
    }

    /// Gets the value of DefaultDir
    pub fn get_default_dir(&self) -> Option<&String> {
        self.default_dir.as_ref()
    }

    /// Sets the value of Directory
    pub fn set_directory(&mut self, value: String) {
        self.directory = Some(value);
    }

    /// Gets the value of Directory
    pub fn get_directory(&self) -> Option<&String> {
        self.directory.as_ref()
    }
}

