// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystemToPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystemToPartition {

/// 
    #[serde(rename = "Partition")]
    pub partition: Option<MSFT_Partition>,

/// 
    #[serde(rename = "StorageSubSystem")]
    pub storage_sub_system: Option<MSFT_StorageSubSystem>,
}

impl MSFT_StorageSubSystemToPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            partition: None,
            storage_sub_system: None,
        }
    }


    /// Sets the value of Partition
    pub fn set_partition(&mut self, value: MSFT_Partition) {
        self.partition = Some(value);
    }

    /// Gets the value of Partition
    pub fn get_partition(&self) -> Option<&MSFT_Partition> {
        self.partition.as_ref()
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

