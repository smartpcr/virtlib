// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapAffinityManagerEnable struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapAffinityManagerEnable {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "BucketIndex")]
    pub bucket_index: Option<u32>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,
}

impl HeapAffinityManagerEnable {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            bucket_index: None,
            heap_handle: None,
        }
    }


    /// Sets the value of BucketIndex
    pub fn set_bucket_index(&mut self, value: u32) {
        self.bucket_index = Some(value);
    }

    /// Gets the value of BucketIndex
    pub fn get_bucket_index(&self) -> Option<&u32> {
        self.bucket_index.as_ref()
    }

    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }
}

