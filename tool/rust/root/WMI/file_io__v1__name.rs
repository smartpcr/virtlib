// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FileIo_V1_Name struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileIo_V1_Name {
    #[serde(flatten)]
    pub base: FileIo_V1,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "FileObject")]
    pub file_object: Option<u32>,
}

impl FileIo_V1_Name {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: FileIo_V1::new(),
            file_name: None,
            file_object: None,
        }
    }


    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of FileObject
    pub fn set_file_object(&mut self, value: u32) {
        self.file_object = Some(value);
    }

    /// Gets the value of FileObject
    pub fn get_file_object(&self) -> Option<&u32> {
        self.file_object.as_ref()
    }
}

