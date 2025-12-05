// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DirectorySpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DirectorySpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "DirectoryPath")]
    pub directory_path: Option<String>,

/// 
    #[serde(rename = "DirectoryType")]
    pub directory_type: Option<u16>,
}

impl CIM_DirectorySpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            directory_path: None,
            directory_type: None,
        }
    }


    /// Sets the value of DirectoryPath
    pub fn set_directory_path(&mut self, value: String) {
        self.directory_path = Some(value);
    }

    /// Gets the value of DirectoryPath
    pub fn get_directory_path(&self) -> Option<&String> {
        self.directory_path.as_ref()
    }

    /// Sets the value of DirectoryType
    pub fn set_directory_type(&mut self, value: u16) {
        self.directory_type = Some(value);
    }

    /// Gets the value of DirectoryType
    pub fn get_directory_type(&self) -> Option<&u16> {
        self.directory_type.as_ref()
    }
}

