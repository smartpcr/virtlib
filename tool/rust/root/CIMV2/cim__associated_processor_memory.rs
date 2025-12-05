// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AssociatedProcessorMemory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AssociatedProcessorMemory {
    #[serde(flatten)]
    pub base: CIM_AssociatedMemory,

/// 
    #[serde(rename = "BusSpeed")]
    pub bus_speed: Option<u32>,
}

impl CIM_AssociatedProcessorMemory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_AssociatedMemory::new(),
            bus_speed: None,
        }
    }


    /// Sets the value of BusSpeed
    pub fn set_bus_speed(&mut self, value: u32) {
        self.bus_speed = Some(value);
    }

    /// Gets the value of BusSpeed
    pub fn get_bus_speed(&self) -> Option<&u32> {
        self.bus_speed.as_ref()
    }
}

