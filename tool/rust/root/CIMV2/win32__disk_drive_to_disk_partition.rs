// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_DiskDriveToDiskPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_DiskDriveToDiskPartition {
    #[serde(flatten)]
    pub base: CIM_MediaPresent,
}

impl Win32_DiskDriveToDiskPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MediaPresent::new(),
        }
    }

}

