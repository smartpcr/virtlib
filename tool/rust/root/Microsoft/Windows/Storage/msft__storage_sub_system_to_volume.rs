// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToVolume {

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<MSFT_Volume>,
}

impl MSFT_StorageSubSystemToVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_sub_system: None,
            volume: None,
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

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: MSFT_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&MSFT_Volume> {
        self.volume.as_ref()
    }
}

