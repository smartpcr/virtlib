// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapAlloc struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapAlloc {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "AllocAddress")]
    pub alloc_address: Option<u32>,

/// 
    #[serde(rename = "AllocSize")]
    pub alloc_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "SourceId")]
    pub source_id: Option<u32>,
}

impl HeapAlloc {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            alloc_address: None,
            alloc_size: None,
            heap_handle: None,
            source_id: None,
        }
    }


    /// Sets the value of AllocAddress
    pub fn set_alloc_address(&mut self, value: u32) {
        self.alloc_address = Some(value);
    }

    /// Gets the value of AllocAddress
    pub fn get_alloc_address(&self) -> Option<&u32> {
        self.alloc_address.as_ref()
    }

    /// Sets the value of AllocSize
    pub fn set_alloc_size(&mut self, value: serde_json::Value) {
        self.alloc_size = Some(value);
    }

    /// Gets the value of AllocSize
    pub fn get_alloc_size(&self) -> Option<&serde_json::Value> {
        self.alloc_size.as_ref()
    }

    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }

    /// Sets the value of SourceId
    pub fn set_source_id(&mut self, value: u32) {
        self.source_id = Some(value);
    }

    /// Gets the value of SourceId
    pub fn get_source_id(&self) -> Option<&u32> {
        self.source_id.as_ref()
    }
}

