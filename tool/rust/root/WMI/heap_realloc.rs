// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapRealloc struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapRealloc {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "NewAllocAddress")]
    pub new_alloc_address: Option<u32>,

/// 
    #[serde(rename = "NewAllocSize")]
    pub new_alloc_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "OldAllocAddress")]
    pub old_alloc_address: Option<u32>,

/// 
    #[serde(rename = "OldAllocSize")]
    pub old_alloc_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "SourceId")]
    pub source_id: Option<u32>,
}

impl HeapRealloc {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            heap_handle: None,
            new_alloc_address: None,
            new_alloc_size: None,
            old_alloc_address: None,
            old_alloc_size: None,
            source_id: None,
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

    /// Sets the value of NewAllocAddress
    pub fn set_new_alloc_address(&mut self, value: u32) {
        self.new_alloc_address = Some(value);
    }

    /// Gets the value of NewAllocAddress
    pub fn get_new_alloc_address(&self) -> Option<&u32> {
        self.new_alloc_address.as_ref()
    }

    /// Sets the value of NewAllocSize
    pub fn set_new_alloc_size(&mut self, value: serde_json::Value) {
        self.new_alloc_size = Some(value);
    }

    /// Gets the value of NewAllocSize
    pub fn get_new_alloc_size(&self) -> Option<&serde_json::Value> {
        self.new_alloc_size.as_ref()
    }

    /// Sets the value of OldAllocAddress
    pub fn set_old_alloc_address(&mut self, value: u32) {
        self.old_alloc_address = Some(value);
    }

    /// Gets the value of OldAllocAddress
    pub fn get_old_alloc_address(&self) -> Option<&u32> {
        self.old_alloc_address.as_ref()
    }

    /// Sets the value of OldAllocSize
    pub fn set_old_alloc_size(&mut self, value: serde_json::Value) {
        self.old_alloc_size = Some(value);
    }

    /// Gets the value of OldAllocSize
    pub fn get_old_alloc_size(&self) -> Option<&serde_json::Value> {
        self.old_alloc_size.as_ref()
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

