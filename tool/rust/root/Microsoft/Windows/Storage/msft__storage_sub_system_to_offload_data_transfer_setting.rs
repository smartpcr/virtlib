// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToOffloadDataTransferSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToOffloadDataTransferSetting {

/// 
    #[serde(rename = "OffloadDataTransferSetting")]
    pub offload_data_transfer_setting: Option<MSFT_OffloadDataTransferSetting>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToOffloadDataTransferSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            offload_data_transfer_setting: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of OffloadDataTransferSetting
    pub fn set_offload_data_transfer_setting(&mut self, value: MSFT_OffloadDataTransferSetting) {
        self.offload_data_transfer_setting = Some(value);
    }

    /// Gets the value of OffloadDataTransferSetting
    pub fn get_offload_data_transfer_setting(&self) -> Option<&MSFT_OffloadDataTransferSetting> {
        self.offload_data_transfer_setting.as_ref()
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

