// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToStorageEnclosure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToStorageEnclosure {

/// 
    #[serde(rename = "StorageEnclosure")]
    pub storage_enclosure: Option<MSFT_StorageEnclosure>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToStorageEnclosure {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_enclosure: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of StorageEnclosure
    pub fn set_storage_enclosure(&mut self, value: MSFT_StorageEnclosure) {
        self.storage_enclosure = Some(value);
    }

    /// Gets the value of StorageEnclosure
    pub fn get_storage_enclosure(&self) -> Option<&MSFT_StorageEnclosure> {
        self.storage_enclosure.as_ref()
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

