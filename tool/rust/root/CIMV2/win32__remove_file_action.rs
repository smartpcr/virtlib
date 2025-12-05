// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_RemoveFileAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_RemoveFileAction {
    #[serde(flatten)]
    pub base: CIM_RemoveFileAction,

/// 
    #[serde(rename = "DirProperty")]
    pub dir_property: Option<String>,

/// 
    #[serde(rename = "FileKey")]
    pub file_key: Option<String>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<String>,

/// 
    #[serde(rename = "InstallMode")]
    pub install_mode: Option<u16>,
}

impl Win32_RemoveFileAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RemoveFileAction::new(),
            dir_property: None,
            file_key: None,
            file_name: None,
            install_mode: None,
        }
    }


    /// Sets the value of DirProperty
    pub fn set_dir_property(&mut self, value: String) {
        self.dir_property = Some(value);
    }

    /// Gets the value of DirProperty
    pub fn get_dir_property(&self) -> Option<&String> {
        self.dir_property.as_ref()
    }

    /// Sets the value of FileKey
    pub fn set_file_key(&mut self, value: String) {
        self.file_key = Some(value);
    }

    /// Gets the value of FileKey
    pub fn get_file_key(&self) -> Option<&String> {
        self.file_key.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: String) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&String> {
        self.file_name.as_ref()
    }

    /// Sets the value of InstallMode
    pub fn set_install_mode(&mut self, value: u16) {
        self.install_mode = Some(value);
    }

    /// Gets the value of InstallMode
    pub fn get_install_mode(&self) -> Option<&u16> {
        self.install_mode.as_ref()
    }
}

