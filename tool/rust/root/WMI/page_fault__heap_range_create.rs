// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_HeapRangeCreate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_HeapRangeCreate {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "FirstRangeSize")]
    pub first_range_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "HRCreateFlags")]
    pub hrcreate_flags: Option<u32>,
}

impl PageFault_HeapRangeCreate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            first_range_size: None,
            heap_handle: None,
            hrcreate_flags: None,
        }
    }


    /// Sets the value of FirstRangeSize
    pub fn set_first_range_size(&mut self, value: serde_json::Value) {
        self.first_range_size = Some(value);
    }

    /// Gets the value of FirstRangeSize
    pub fn get_first_range_size(&self) -> Option<&serde_json::Value> {
        self.first_range_size.as_ref()
    }

    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }

    /// Sets the value of HRCreateFlags
    pub fn set_hrcreate_flags(&mut self, value: u32) {
        self.hrcreate_flags = Some(value);
    }

    /// Gets the value of HRCreateFlags
    pub fn get_hrcreate_flags(&self) -> Option<&u32> {
        self.hrcreate_flags.as_ref()
    }
}

