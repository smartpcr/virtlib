// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalDisk {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<u64>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,
}

impl CIM_LogicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            free_space: None,
            size: None,
        }
    }


    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: u64) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&u64> {
        self.free_space.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }
}

