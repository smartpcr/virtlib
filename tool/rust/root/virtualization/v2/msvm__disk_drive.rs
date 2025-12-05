// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_DiskDrive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_DiskDrive {
    #[serde(flatten)]
    pub base: CIM_DiskDrive,

/// 
    #[serde(rename = "DriveNumber")]
    pub drive_number: Option<u32>,
}

impl Msvm_DiskDrive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DiskDrive::new(),
            drive_number: None,
        }
    }


    /// Sets the value of DriveNumber
    pub fn set_drive_number(&mut self, value: u32) {
        self.drive_number = Some(value);
    }

    /// Gets the value of DriveNumber
    pub fn get_drive_number(&self) -> Option<&u32> {
        self.drive_number.as_ref()
    }
}

