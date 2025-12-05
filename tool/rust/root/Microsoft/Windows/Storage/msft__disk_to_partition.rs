// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DiskToPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DiskToPartition {

/// 
    #[serde(rename = "Disk")]
    pub disk: Option<MSFT_Disk>,

/// 
    #[serde(rename = "Partition")]
    pub partition: Option<MSFT_Partition>,
}

impl MSFT_DiskToPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disk: None,
            partition: None,
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

    /// Sets the value of Partition
    pub fn set_partition(&mut self, value: MSFT_Partition) {
        self.partition = Some(value);
    }

    /// Gets the value of Partition
    pub fn get_partition(&self) -> Option<&MSFT_Partition> {
        self.partition.as_ref()
    }
}

