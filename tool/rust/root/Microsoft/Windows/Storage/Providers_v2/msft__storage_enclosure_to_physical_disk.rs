// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageEnclosureToPhysicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageEnclosureToPhysicalDisk {

/// 
    #[serde(rename = "PhysicalDisk")]
    pub physical_disk: Option<MSFT_PhysicalDisk>,

/// 
    #[serde(rename = "StorageEnclosure")]
    pub storage_enclosure: Option<MSFT_StorageEnclosure>,
}

impl MSFT_StorageEnclosureToPhysicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            physical_disk: None,
            storage_enclosure: None,
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

    /// Sets the value of StorageEnclosure
    pub fn set_storage_enclosure(&mut self, value: MSFT_StorageEnclosure) {
        self.storage_enclosure = Some(value);
    }

    /// Gets the value of StorageEnclosure
    pub fn get_storage_enclosure(&self) -> Option<&MSFT_StorageEnclosure> {
        self.storage_enclosure.as_ref()
    }
}

