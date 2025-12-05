// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_VirtualAlloc struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_VirtualAlloc {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "RegionSize")]
    pub region_size: Option<serde_json::Value>,
}

impl PageFault_VirtualAlloc {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            base_address: None,
            process_id: None,
            region_size: None,
        }
    }


    /// Sets the value of BaseAddress
    pub fn set_base_address(&mut self, value: u32) {
        self.base_address = Some(value);
    }

    /// Gets the value of BaseAddress
    pub fn get_base_address(&self) -> Option<&u32> {
        self.base_address.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of RegionSize
    pub fn set_region_size(&mut self, value: serde_json::Value) {
        self.region_size = Some(value);
    }

    /// Gets the value of RegionSize
    pub fn get_region_size(&self) -> Option<&serde_json::Value> {
        self.region_size.as_ref()
    }
}

