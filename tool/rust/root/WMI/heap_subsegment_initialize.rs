// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapSubsegmentInitialize struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapSubsegmentInitialize {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "AffinityIndex")]
    pub affinity_index: Option<u32>,

/// 
    #[serde(rename = "BlockCount")]
    pub block_count: Option<serde_json::Value>,

/// 
    #[serde(rename = "BlockSize")]
    pub block_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "SubSegment")]
    pub sub_segment: Option<u32>,
}

impl HeapSubsegmentInitialize {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            affinity_index: None,
            block_count: None,
            block_size: None,
            heap_handle: None,
            sub_segment: None,
        }
    }


    /// Sets the value of AffinityIndex
    pub fn set_affinity_index(&mut self, value: u32) {
        self.affinity_index = Some(value);
    }

    /// Gets the value of AffinityIndex
    pub fn get_affinity_index(&self) -> Option<&u32> {
        self.affinity_index.as_ref()
    }

    /// Sets the value of BlockCount
    pub fn set_block_count(&mut self, value: serde_json::Value) {
        self.block_count = Some(value);
    }

    /// Gets the value of BlockCount
    pub fn get_block_count(&self) -> Option<&serde_json::Value> {
        self.block_count.as_ref()
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
}

