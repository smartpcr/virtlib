// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToVirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToVirtualDisk {

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,

/// 
    #[serde(rename = "VirtualDisk")]
    pub virtual_disk: Option<MSFT_VirtualDisk>,
}

impl MSFT_StorageSubSystemToVirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_sub_system: None,
            virtual_disk: None,
        }
    }


    /// Sets the value of StorageSubSystem
    pub fn set_storage_sub_system(&mut self, value: MSFT_StorageSubSystem) {
        self.storage_sub_system = Some(value);
    }

    /// Gets the value of StorageSubSystem
    pub fn get_storage_sub_system(&self) -> Option<&MSFT_StorageSubSystem> {
        self.storage_sub_system.as_ref()
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

