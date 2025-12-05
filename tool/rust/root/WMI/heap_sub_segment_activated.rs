// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapSubSegmentActivated struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapSubSegmentActivated {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "SubSegment")]
    pub sub_segment: Option<u32>,
}

impl HeapSubSegmentActivated {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            heap_handle: None,
            sub_segment: None,
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

    /// Sets the value of SubSegment
    pub fn set_sub_segment(&mut self, value: u32) {
        self.sub_segment = Some(value);
    }

    /// Gets the value of SubSegment
    pub fn get_sub_segment(&self) -> Option<&u32> {
        self.sub_segment.as_ref()
    }
}

