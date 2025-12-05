// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDiskToStorageTier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDiskToStorageTier {

/// 
    #[serde(rename = "StorageTier")]
    pub storage_tier: Option<MSFT_StorageTier>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_VirtualDiskToStorageTier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_tier: None,
            virtual_disk: None,
        }
    }


    /// Sets the value of StorageTier
    pub fn set_storage_tier(&mut self, value: MSFT_StorageTier) {
        self.storage_tier = Some(value);
    }

    /// Gets the value of StorageTier
    pub fn get_storage_tier(&self) -> Option<&MSFT_StorageTier> {
        self.storage_tier.as_ref()
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

