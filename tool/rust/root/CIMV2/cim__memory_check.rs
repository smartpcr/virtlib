// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MemoryCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MemoryCheck {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "MemorySize")]
    pub memory_size: Option<u64>,
}

impl CIM_MemoryCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            memory_size: None,
        }
    }


    /// Sets the value of MemorySize
    pub fn set_memory_size(&mut self, value: u64) {
        self.memory_size = Some(value);
    }

    /// Gets the value of MemorySize
    pub fn get_memory_size(&self) -> Option<&u64> {
        self.memory_size.as_ref()
    }
}

