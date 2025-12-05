// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MemoryCapacity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MemoryCapacity {
    #[serde(flatten)]
    pub base: CIM_PhysicalCapacity,

/// 
    #[serde(rename = "MaximumMemoryCapacity")]
    pub maximum_memory_capacity: Option<u64>,

/// 
    #[serde(rename = "MemoryType")]
    pub memory_type: Option<u16>,

/// 
    #[serde(rename = "MinimumMemoryCapacity")]
    pub minimum_memory_capacity: Option<u64>,
}

impl CIM_MemoryCapacity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalCapacity::new(),
            maximum_memory_capacity: None,
            memory_type: None,
            minimum_memory_capacity: None,
        }
    }


    /// Sets the value of MaximumMemoryCapacity
    pub fn set_maximum_memory_capacity(&mut self, value: u64) {
        self.maximum_memory_capacity = Some(value);
    }

    /// Gets the value of MaximumMemoryCapacity
    pub fn get_maximum_memory_capacity(&self) -> Option<&u64> {
        self.maximum_memory_capacity.as_ref()
    }

    /// Sets the value of MemoryType
    pub fn set_memory_type(&mut self, value: u16) {
        self.memory_type = Some(value);
    }

    /// Gets the value of MemoryType
    pub fn get_memory_type(&self) -> Option<&u16> {
        self.memory_type.as_ref()
    }

    /// Sets the value of MinimumMemoryCapacity
    pub fn set_minimum_memory_capacity(&mut self, value: u64) {
        self.minimum_memory_capacity = Some(value);
    }

    /// Gets the value of MinimumMemoryCapacity
    pub fn get_minimum_memory_capacity(&self) -> Option<&u64> {
        self.minimum_memory_capacity.as_ref()
    }
}

