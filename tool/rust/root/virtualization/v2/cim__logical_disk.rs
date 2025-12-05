// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalDisk {
    #[serde(flatten)]
    pub base: CIM_StorageExtent,
}

impl CIM_LogicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageExtent::new(),
        }
    }

}

