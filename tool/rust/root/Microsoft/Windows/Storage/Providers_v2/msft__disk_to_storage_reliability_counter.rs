// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DiskToStorageReliabilityCounter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DiskToStorageReliabilityCounter {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// 
    #[serde(rename = "StorageReliabilityCounter")]
    pub storage_reliability_counter: Option<MSFT_StorageReliabilityCounter>,
}

impl MSFT_DiskToStorageReliabilityCounter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk: None,
            storage_reliability_counter: None,
        }
    }


    /// Sets the value of Disk
    pub fn set_disk(&mut self, value: MSFT_Disk) {
        self.disk = Some(value);
    }

    /// Gets the value of Disk
    pub fn get_disk(&self) -> Option<&MSFT_Disk> {
        self.disk.as_ref()
    }

    /// Sets the value of StorageReliabilityCounter
    pub fn set_storage_reliability_counter(&mut self, value: MSFT_StorageReliabilityCounter) {
        self.storage_reliability_counter = Some(value);
    }

    /// Gets the value of StorageReliabilityCounter
    pub fn get_storage_reliability_counter(&self) -> Option<&MSFT_StorageReliabilityCounter> {
        self.storage_reliability_counter.as_ref()
    }
}

