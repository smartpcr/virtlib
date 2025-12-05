// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToPhysicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToPhysicalDisk {

/// 
    #[serde(rename = "PhysicalDisk")]
    pub physical_disk: Option<MSFT_PhysicalDisk>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToPhysicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            physical_disk: None,
            storage_sub_system: None,
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

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

