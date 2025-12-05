// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageProviderToStorageSubSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageProviderToStorageSubSystem {

/// 
    #[serde(rename = "StorageProvider")]
    pub storage_provider: Option<MSFT_StorageProvider>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageProviderToStorageSubSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_provider: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of StorageProvider
    pub fn set_storage_provider(&mut self, value: MSFT_StorageProvider) {
        self.storage_provider = Some(value);
    }

    /// Gets the value of StorageProvider
    pub fn get_storage_provider(&self) -> Option<&MSFT_StorageProvider> {
        self.storage_provider.as_ref()
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

