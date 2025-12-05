// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StoragePoolToPhysicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StoragePoolToPhysicalDisk {

/// 
    #[serde(rename = "PhysicalDisk")]
    pub physical_disk: Option<MSFT_PhysicalDisk>,

/// 
    #[serde(rename = "StoragePool")]
    pub storage_pool: Option<MSFT_StoragePool>,
}

impl MSFT_StoragePoolToPhysicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            physical_disk: None,
            storage_pool: None,
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

    /// Sets the value of StoragePool
    pub fn set_storage_pool(&mut self, value: MSFT_StoragePool) {
        self.storage_pool = Some(value);
    }

    /// Gets the value of StoragePool
    pub fn get_storage_pool(&self) -> Option<&MSFT_StoragePool> {
        self.storage_pool.as_ref()
    }
}

