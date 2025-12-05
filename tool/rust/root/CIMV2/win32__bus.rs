// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Bus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Bus {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "BusNum")]
    pub bus_num: Option<u32>,

/// 
    #[serde(rename = "BusType")]
    pub bus_type: Option<u32>,
}

impl Win32_Bus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            bus_num: None,
            bus_type: None,
        }
    }


    /// Sets the value of BusNum
    pub fn set_bus_num(&mut self, value: u32) {
        self.bus_num = Some(value);
    }

    /// Gets the value of BusNum
    pub fn get_bus_num(&self) -> Option<&u32> {
        self.bus_num.as_ref()
    }

    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: u32) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&u32> {
        self.bus_type.as_ref()
    }
}

