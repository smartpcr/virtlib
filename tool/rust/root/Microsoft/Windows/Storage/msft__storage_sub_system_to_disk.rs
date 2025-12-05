// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToDisk {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk: None,
            storage_sub_system: None,
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

    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
    }
}

