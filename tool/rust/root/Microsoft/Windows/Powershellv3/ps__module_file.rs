// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Powershellv3
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_ModuleFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_ModuleFile {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "FileData")]
    pub file_data: Vec<u8>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,
}

impl PS_ModuleFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            file_data: Vec::new(),
            file_name: None,
        }
    }


    /// Sets the value of FileData
    pub fn set_file_data(&mut self, value: Vec<u8>) {
        self.file_data = value;
    }

    /// Gets the value of FileData
    pub fn get_file_data(&self) -> &Vec<u8> {
        &self.file_data
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }
}

