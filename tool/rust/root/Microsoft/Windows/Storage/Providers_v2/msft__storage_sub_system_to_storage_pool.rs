// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToStoragePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToStoragePool {

/// 
    #[serde(rename = "StoragePool")]
    pub storage_pool: Option<MSFT_StoragePool>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToStoragePool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_pool: None,
            storage_sub_system: None,
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

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

