// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_BootConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_BootConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "BootDirectory")]
    pub boot_directory: Option<String>,

/// 
    #[serde(rename = "ConfigurationPath")]
    pub configuration_path: Option<String>,

/// 
    #[serde(rename = "LastDrive")]
    pub last_drive: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ScratchDirectory")]
    pub scratch_directory: Option<String>,

/// 
    #[serde(rename = "TempDirectory")]
    pub temp_directory: Option<String>,
}

impl Win32_BootConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            boot_directory: None,
            configuration_path: None,
            last_drive: None,
            name: None,
            scratch_directory: None,
            temp_directory: None,
        }
    }


    /// Sets the value of BootDirectory
    pub fn set_boot_directory(&mut self, value: String) {
        self.boot_directory = Some(value);
    }

    /// Gets the value of BootDirectory
    pub fn get_boot_directory(&self) -> Option<&String> {
        self.boot_directory.as_ref()
    }

    /// Sets the value of ConfigurationPath
    pub fn set_configuration_path(&mut self, value: String) {
        self.configuration_path = Some(value);
    }

    /// Gets the value of ConfigurationPath
    pub fn get_configuration_path(&self) -> Option<&String> {
        self.configuration_path.as_ref()
    }

    /// Sets the value of LastDrive
    pub fn set_last_drive(&mut self, value: String) {
        self.last_drive = Some(value);
    }

    /// Gets the value of LastDrive
    pub fn get_last_drive(&self) -> Option<&String> {
        self.last_drive.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ScratchDirectory
    pub fn set_scratch_directory(&mut self, value: String) {
        self.scratch_directory = Some(value);
    }

    /// Gets the value of ScratchDirectory
    pub fn get_scratch_directory(&self) -> Option<&String> {
        self.scratch_directory.as_ref()
    }

    /// Sets the value of TempDirectory
    pub fn set_temp_directory(&mut self, value: String) {
        self.temp_directory = Some(value);
    }

    /// Gets the value of TempDirectory
    pub fn get_temp_directory(&self) -> Option<&String> {
        self.temp_directory.as_ref()
    }
}

