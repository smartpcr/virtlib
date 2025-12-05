// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_HeapRangeRundown_V4 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_HeapRangeRundown_V4 {
    #[serde(flatten)]
    pub base: PageFault,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "HRFlags")]
    pub hrflags: Option<u32>,

/// 
    #[serde(rename = "HRHeapTag")]
    pub hrheap_tag: Option<u64>,

/// 
    #[serde(rename = "HRPid")]
    pub hrpid: Option<u32>,

/// 
    #[serde(rename = "HRRangeCount")]
    pub hrrange_count: Option<u32>,
}

impl PageFault_HeapRangeRundown_V4 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault::new(),
            heap_handle: None,
            hrflags: None,
            hrheap_tag: None,
            hrpid: None,
            hrrange_count: None,
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

    /// Sets the value of HRFlags
    pub fn set_hrflags(&mut self, value: u32) {
        self.hrflags = Some(value);
    }

    /// Gets the value of HRFlags
    pub fn get_hrflags(&self) -> Option<&u32> {
        self.hrflags.as_ref()
    }

    /// Sets the value of HRHeapTag
    pub fn set_hrheap_tag(&mut self, value: u64) {
        self.hrheap_tag = Some(value);
    }

    /// Gets the value of HRHeapTag
    pub fn get_hrheap_tag(&self) -> Option<&u64> {
        self.hrheap_tag.as_ref()
    }

    /// Sets the value of HRPid
    pub fn set_hrpid(&mut self, value: u32) {
        self.hrpid = Some(value);
    }

    /// Gets the value of HRPid
    pub fn get_hrpid(&self) -> Option<&u32> {
        self.hrpid.as_ref()
    }

    /// Sets the value of HRRangeCount
    pub fn set_hrrange_count(&mut self, value: u32) {
        self.hrrange_count = Some(value);
    }

    /// Gets the value of HRRangeCount
    pub fn get_hrrange_count(&self) -> Option<&u32> {
        self.hrrange_count.as_ref()
    }
}

