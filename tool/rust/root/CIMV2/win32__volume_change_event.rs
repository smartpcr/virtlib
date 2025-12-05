// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_VolumeChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_VolumeChangeEvent {
    #[serde(flatten)]
    pub base: Win32_DeviceChangeEvent,

/// 
    #[serde(rename = "DriveName")]
    pub drive_name: Option<String>,
}

impl Win32_VolumeChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_DeviceChangeEvent::new(),
            drive_name: None,
        }
    }


    /// Sets the value of DriveName
    pub fn set_drive_name(&mut self, value: String) {
        self.drive_name = Some(value);
    }

    /// Gets the value of DriveName
    pub fn get_drive_name(&self) -> Option<&String> {
        self.drive_name.as_ref()
    }
}

