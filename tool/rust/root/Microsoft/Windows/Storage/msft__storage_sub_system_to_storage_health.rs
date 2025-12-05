// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToStorageHealth struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToStorageHealth {

/// 
    #[serde(rename = "StorageHealth")]
    pub storage_health: Option<MSFT_StorageHealth>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToStorageHealth {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_health: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of StorageHealth
    pub fn set_storage_health(&mut self, value: MSFT_StorageHealth) {
        self.storage_health = Some(value);
    }

    /// Gets the value of StorageHealth
    pub fn get_storage_health(&self) -> Option<&MSFT_StorageHealth> {
        self.storage_health.as_ref()
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

