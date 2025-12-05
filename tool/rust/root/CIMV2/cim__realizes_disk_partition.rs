// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RealizesDiskPartition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RealizesDiskPartition {
    #[serde(flatten)]
    pub base: CIM_Realizes,

/// 
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,
}

impl CIM_RealizesDiskPartition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Realizes::new(),
            starting_address: None,
        }
    }


    /// Sets the value of StartingAddress
    pub fn set_starting_address(&mut self, value: u64) {
        self.starting_address = Some(value);
    }

    /// Gets the value of StartingAddress
    pub fn get_starting_address(&self) -> Option<&u64> {
        self.starting_address.as_ref()
    }
}

