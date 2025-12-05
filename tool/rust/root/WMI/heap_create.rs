// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapCreate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapCreate {
    #[serde(flatten)]
    pub base: HeapTrace,

/// 
    #[serde(rename = "AllocatedSpace")]
    pub allocated_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "CommittedSpace")]
    pub committed_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapFlags")]
    pub heap_flags: Option<u32>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "ReservedSpace")]
    pub reserved_space: Option<serde_json::Value>,
}

impl HeapCreate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace::new(),
            allocated_space: None,
            committed_space: None,
            heap_flags: None,
            heap_handle: None,
            reserved_space: None,
        }
    }


    /// Sets the value of AllocatedSpace
    pub fn set_allocated_space(&mut self, value: serde_json::Value) {
        self.allocated_space = Some(value);
    }

    /// Gets the value of AllocatedSpace
    pub fn get_allocated_space(&self) -> Option<&serde_json::Value> {
        self.allocated_space.as_ref()
    }

    /// Sets the value of CommittedSpace
    pub fn set_committed_space(&mut self, value: serde_json::Value) {
        self.committed_space = Some(value);
    }

    /// Gets the value of CommittedSpace
    pub fn get_committed_space(&self) -> Option<&serde_json::Value> {
        self.committed_space.as_ref()
    }

    /// Sets the value of HeapFlags
    pub fn set_heap_flags(&mut self, value: u32) {
        self.heap_flags = Some(value);
    }

    /// Gets the value of HeapFlags
    pub fn get_heap_flags(&self) -> Option<&u32> {
        self.heap_flags.as_ref()
    }

    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }

    /// Sets the value of ReservedSpace
    pub fn set_reserved_space(&mut self, value: serde_json::Value) {
        self.reserved_space = Some(value);
    }

    /// Gets the value of ReservedSpace
    pub fn get_reserved_space(&self) -> Option<&serde_json::Value> {
        self.reserved_space.as_ref()
    }
}

