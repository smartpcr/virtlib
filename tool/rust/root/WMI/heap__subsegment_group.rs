// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Heap_SubsegmentGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Heap_SubsegmentGroup {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "SubSegment")]
    pub sub_segment: Option<u32>,

/// 
    #[serde(rename = "SubSegmentSize")]
    pub sub_segment_size: Option<serde_json::Value>,
}

impl Heap_SubsegmentGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            block_size: None,
            heap_handle: None,
            sub_segment: None,
            sub_segment_size: None,
        }
    }


    /// Sets the value of BlockSize
    pub fn set_block_size(&mut self, value: serde_json::Value) {
        self.block_size = Some(value);
    }

    /// Gets the value of BlockSize
    pub fn get_block_size(&self) -> Option<&serde_json::Value> {
        self.block_size.as_ref()
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

    /// Sets the value of SubSegmentSize
    pub fn set_sub_segment_size(&mut self, value: serde_json::Value) {
        self.sub_segment_size = Some(value);
    }

    /// Gets the value of SubSegmentSize
    pub fn get_sub_segment_size(&self) -> Option<&serde_json::Value> {
        self.sub_segment_size.as_ref()
    }
}

