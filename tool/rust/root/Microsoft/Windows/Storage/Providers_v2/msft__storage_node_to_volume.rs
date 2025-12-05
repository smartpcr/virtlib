// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToVolume {

/// 
    #[serde(rename = "StorageNode")]
    pub storage_node: Option<MSFT_StorageNode>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<MSFT_Volume>,
}

impl MSFT_StorageNodeToVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_node: None,
            volume: None,
        }
    }


    /// Sets the value of StorageNode
    pub fn set_storage_node(&mut self, value: MSFT_StorageNode) {
        self.storage_node = Some(value);
    }

    /// Gets the value of StorageNode
    pub fn get_storage_node(&self) -> Option<&MSFT_StorageNode> {
        self.storage_node.as_ref()
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

