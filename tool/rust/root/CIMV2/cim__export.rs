// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Export struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Export {

/// 
    #[serde(rename = "Directory")]
    pub directory: Option<CIM_Directory>,

/// 
    #[serde(rename = "ExportedDirectoryName")]
    pub exported_directory_name: Option<String>,

/// 
    #[serde(rename = "LocalFS")]
    pub local_fs: Option<CIM_LocalFileSystem>,
}

impl CIM_Export {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            directory: None,
            exported_directory_name: None,
            local_fs: None,
        }
    }


    /// Sets the value of Directory
    pub fn set_directory(&mut self, value: CIM_Directory) {
        self.directory = Some(value);
    }

    /// Gets the value of Directory
    pub fn get_directory(&self) -> Option<&CIM_Directory> {
        self.directory.as_ref()
    }

    /// Sets the value of ExportedDirectoryName
    pub fn set_exported_directory_name(&mut self, value: String) {
        self.exported_directory_name = Some(value);
    }

    /// Gets the value of ExportedDirectoryName
    pub fn get_exported_directory_name(&self) -> Option<&String> {
        self.exported_directory_name.as_ref()
    }

    /// Sets the value of LocalFS
    pub fn set_local_fs(&mut self, value: CIM_LocalFileSystem) {
        self.local_fs = Some(value);
    }

    /// Gets the value of LocalFS
    pub fn get_local_fs(&self) -> Option<&CIM_LocalFileSystem> {
        self.local_fs.as_ref()
    }
}

