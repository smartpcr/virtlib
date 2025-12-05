// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageNodeToStoragePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageNodeToStoragePool {

/// 
    #[serde(rename = "StorageNode")]
    pub storage_node: Option<MSFT_StorageNode>,

/// 
    #[serde(rename = "StoragePool")]
    pub storage_pool: Option<MSFT_StoragePool>,
}

impl MSFT_StorageNodeToStoragePool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            storage_node: None,
            storage_pool: None,
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

    /// Sets the value of StoragePool
    pub fn set_storage_pool(&mut self, value: MSFT_StoragePool) {
        self.storage_pool = Some(value);
    }

    /// Gets the value of StoragePool
    pub fn get_storage_pool(&self) -> Option<&MSFT_StoragePool> {
        self.storage_pool.as_ref()
    }
}

