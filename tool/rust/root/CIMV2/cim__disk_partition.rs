// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DiskPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DiskPartition {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,

/// 
    #[serde(rename = "Bootable")]
    pub bootable: Option<bool>,

/// 
    #[serde(rename = "PrimaryPartition")]
    pub primary_partition: Option<bool>,
}

impl CIM_DiskPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
            bootable: None,
            primary_partition: None,
        }
    }


    /// Sets the value of Bootable
    pub fn set_bootable(&mut self, value: bool) {
        self.bootable = Some(value);
    }

    /// Gets the value of Bootable
    pub fn get_bootable(&self) -> Option<&bool> {
        self.bootable.as_ref()
    }

    /// Sets the value of PrimaryPartition
    pub fn set_primary_partition(&mut self, value: bool) {
        self.primary_partition = Some(value);
    }

    /// Gets the value of PrimaryPartition
    pub fn get_primary_partition(&self) -> Option<&bool> {
        self.primary_partition.as_ref()
    }
}

