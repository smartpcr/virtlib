// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_MountPoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_MountPoint {

/// 
    #[serde(rename = "Directory")]
    pub directory: Option<Win32_Directory>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<Win32_Volume>,
}

impl Win32_MountPoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            directory: None,
            volume: None,
        }
    }


    /// Sets the value of Directory
    pub fn set_directory(&mut self, value: Win32_Directory) {
        self.directory = Some(value);
    }

    /// Gets the value of Directory
    pub fn get_directory(&self) -> Option<&Win32_Directory> {
        self.directory.as_ref()
    }

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: Win32_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&Win32_Volume> {
        self.volume.as_ref()
    }
}

