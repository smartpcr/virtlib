// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_MoveFileAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_MoveFileAction {
    #[serde(flatten)]
    pub base: CIM_FileAction,

/// 
    #[serde(rename = "DestFolder")]
    pub dest_folder: Option<String>,

/// 
    #[serde(rename = "DestName")]
    pub dest_name: Option<String>,

/// 
    #[serde(rename = "FileKey")]
    pub file_key: Option<String>,

/// 
    #[serde(rename = "Options")]
    pub options: Option<u16>,

/// 
    #[serde(rename = "SourceFolder")]
    pub source_folder: Option<String>,

/// 
    #[serde(rename = "SourceName")]
    pub source_name: Option<String>,
}

impl Win32_MoveFileAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FileAction::new(),
            dest_folder: None,
            dest_name: None,
            file_key: None,
            options: None,
            source_folder: None,
            source_name: None,
        }
    }


    /// Sets the value of DestFolder
    pub fn set_dest_folder(&mut self, value: String) {
        self.dest_folder = Some(value);
    }

    /// Gets the value of DestFolder
    pub fn get_dest_folder(&self) -> Option<&String> {
        self.dest_folder.as_ref()
    }

    /// Sets the value of DestName
    pub fn set_dest_name(&mut self, value: String) {
        self.dest_name = Some(value);
    }

    /// Gets the value of DestName
    pub fn get_dest_name(&self) -> Option<&String> {
        self.dest_name.as_ref()
    }

    /// Sets the value of FileKey
    pub fn set_file_key(&mut self, value: String) {
        self.file_key = Some(value);
    }

    /// Gets the value of FileKey
    pub fn get_file_key(&self) -> Option<&String> {
        self.file_key.as_ref()
    }

    /// Sets the value of Options
    pub fn set_options(&mut self, value: u16) {
        self.options = Some(value);
    }

    /// Gets the value of Options
    pub fn get_options(&self) -> Option<&u16> {
        self.options.as_ref()
    }

    /// Sets the value of SourceFolder
    pub fn set_source_folder(&mut self, value: String) {
        self.source_folder = Some(value);
    }

    /// Gets the value of SourceFolder
    pub fn get_source_folder(&self) -> Option<&String> {
        self.source_folder.as_ref()
    }

    /// Sets the value of SourceName
    pub fn set_source_name(&mut self, value: String) {
        self.source_name = Some(value);
    }

    /// Gets the value of SourceName
    pub fn get_source_name(&self) -> Option<&String> {
        self.source_name.as_ref()
    }
}

