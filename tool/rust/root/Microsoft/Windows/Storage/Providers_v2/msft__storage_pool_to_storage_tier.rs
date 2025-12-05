// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StoragePoolToStorageTier struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StoragePoolToStorageTier {

/// 
    #[serde(rename = "StoragePool")]
    pub storage_pool: Option<MSFT_StoragePool>,

/// 
    #[serde(rename = "StorageTier")]
    pub storage_tier: Option<MSFT_StorageTier>,
}

impl MSFT_StoragePoolToStorageTier {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_pool: None,
            storage_tier: None,
        }
    }


    /// Sets the value of StoragePool
    pub fn set_storage_pool(&mut self, value: MSFT_StoragePool) {
        self.storage_pool = Some(value);
    }

    /// Gets the value of StoragePool
    pub fn get_storage_pool(&self) -> Option<&MSFT_StoragePool> {
        self.storage_pool.as_ref()
    }

    /// Sets the value of StorageTier
    pub fn set_storage_tier(&mut self, value: MSFT_StorageTier) {
        self.storage_tier = Some(value);
    }

    /// Gets the value of StorageTier
    pub fn get_storage_tier(&self) -> Option<&MSFT_StorageTier> {
        self.storage_tier.as_ref()
    }
}

