// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapExpand struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapExpand {
    #[serde(flatten)]
    pub base: HeapTrace,

/// 
    #[serde(rename = "AllocatedSpace")]
    pub allocated_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "CommitAddress")]
    pub commit_address: Option<u32>,

/// 
    #[serde(rename = "CommittedSize")]
    pub committed_size: Option<serde_json::Value>,

/// 
    #[serde(rename = "CommittedSpace")]
    pub committed_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "NoOfUCRs")]
    pub no_of_ucrs: Option<u32>,

/// 
    #[serde(rename = "ReservedSpace")]
    pub reserved_space: Option<serde_json::Value>,
}

impl HeapExpand {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace::new(),
            allocated_space: None,
            commit_address: None,
            committed_size: None,
            committed_space: None,
            free_space: None,
            heap_handle: None,
            no_of_ucrs: None,
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

    /// Sets the value of CommitAddress
    pub fn set_commit_address(&mut self, value: u32) {
        self.commit_address = Some(value);
    }

    /// Gets the value of CommitAddress
    pub fn get_commit_address(&self) -> Option<&u32> {
        self.commit_address.as_ref()
    }

    /// Sets the value of CommittedSize
    pub fn set_committed_size(&mut self, value: serde_json::Value) {
        self.committed_size = Some(value);
    }

    /// Gets the value of CommittedSize
    pub fn get_committed_size(&self) -> Option<&serde_json::Value> {
        self.committed_size.as_ref()
    }

    /// Sets the value of CommittedSpace
    pub fn set_committed_space(&mut self, value: serde_json::Value) {
        self.committed_space = Some(value);
    }

    /// Gets the value of CommittedSpace
    pub fn get_committed_space(&self) -> Option<&serde_json::Value> {
        self.committed_space.as_ref()
    }

    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: serde_json::Value) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&serde_json::Value> {
        self.free_space.as_ref()
    }

    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }

    /// Sets the value of NoOfUCRs
    pub fn set_no_of_ucrs(&mut self, value: u32) {
        self.no_of_ucrs = Some(value);
    }

    /// Gets the value of NoOfUCRs
    pub fn get_no_of_ucrs(&self) -> Option<&u32> {
        self.no_of_ucrs.as_ref()
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

