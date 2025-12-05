// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DiskSpaceCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DiskSpaceCheck {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "AvailableDiskSpace")]
    pub available_disk_space: Option<u64>,
}

impl CIM_DiskSpaceCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            available_disk_space: None,
        }
    }


    /// Sets the value of AvailableDiskSpace
    pub fn set_available_disk_space(&mut self, value: u64) {
        self.available_disk_space = Some(value);
    }

    /// Gets the value of AvailableDiskSpace
    pub fn get_available_disk_space(&self) -> Option<&u64> {
        self.available_disk_space.as_ref()
    }
}

