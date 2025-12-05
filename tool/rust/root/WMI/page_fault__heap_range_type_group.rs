// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_HeapRangeTypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_HeapRangeTypeGroup {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "HRAddress")]
    pub hraddress: Option<u32>,

/// 
    #[serde(rename = "HRSize")]
    pub hrsize: Option<serde_json::Value>,
}

impl PageFault_HeapRangeTypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            heap_handle: None,
            hraddress: None,
            hrsize: None,
        }
    }


    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }

    /// Sets the value of HRAddress
    pub fn set_hraddress(&mut self, value: u32) {
        self.hraddress = Some(value);
    }

    /// Gets the value of HRAddress
    pub fn get_hraddress(&self) -> Option<&u32> {
        self.hraddress.as_ref()
    }

    /// Sets the value of HRSize
    pub fn set_hrsize(&mut self, value: serde_json::Value) {
        self.hrsize = Some(value);
    }

    /// Gets the value of HRSize
    pub fn get_hrsize(&self) -> Option<&serde_json::Value> {
        self.hrsize.as_ref()
    }
}

