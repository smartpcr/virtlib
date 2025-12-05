// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDiskToPhysicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDiskToPhysicalDisk {

/// 
    #[serde(rename = "PhysicalDisk")]
    pub physical_disk: Option<MSFT_PhysicalDisk>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_VirtualDiskToPhysicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            physical_disk: None,
            virtual_disk: None,
        }
    }


    /// Sets the value of PhysicalDisk
    pub fn set_physical_disk(&mut self, value: MSFT_PhysicalDisk) {
        self.physical_disk = Some(value);
    }

    /// Gets the value of PhysicalDisk
    pub fn get_physical_disk(&self) -> Option<&MSFT_PhysicalDisk> {
        self.physical_disk.as_ref()
    }

    /// Sets the value of VirtualDisk
    pub fn set_virtual_disk(&mut self, value: MSFT_VirtualDisk) {
        self.virtual_disk = Some(value);
    }

    /// Gets the value of VirtualDisk
    pub fn get_virtual_disk(&self) -> Option<&MSFT_VirtualDisk> {
        self.virtual_disk.as_ref()
    }
}

