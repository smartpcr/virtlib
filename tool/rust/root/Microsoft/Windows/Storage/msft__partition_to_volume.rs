// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PartitionToVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PartitionToVolume {

/// 
    #[serde(rename = "Partition")]
    pub partition: Option<MSFT_Partition>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<MSFT_Volume>,
}

impl MSFT_PartitionToVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            partition: None,
            volume: None,
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

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: MSFT_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&MSFT_Volume> {
        self.volume.as_ref()
    }
}

