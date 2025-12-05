// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapSnapShot_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapSnapShot_V2 {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "CommittedSpace")]
    pub committed_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "FreeListLength")]
    pub free_list_length: Option<u32>,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "HeapFlags")]
    pub heap_flags: Option<u32>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "LargeUCRSpace")]
    pub large_ucrspace: Option<serde_json::Value>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ReservedSpace")]
    pub reserved_space: Option<serde_json::Value>,

/// 
    #[serde(rename = "UCRLength")]
    pub ucrlength: Option<u32>,
}

impl HeapSnapShot_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            committed_space: None,
            free_list_length: None,
            free_space: None,
            heap_flags: None,
            heap_handle: None,
            large_ucrspace: None,
            process_id: None,
            reserved_space: None,
            ucrlength: None,
        }
    }


    /// Sets the value of CommittedSpace
    pub fn set_committed_space(&mut self, value: serde_json::Value) {
        self.committed_space = Some(value);
    }

    /// Gets the value of CommittedSpace
    pub fn get_committed_space(&self) -> Option<&serde_json::Value> {
        self.committed_space.as_ref()
    }

    /// Sets the value of FreeListLength
    pub fn set_free_list_length(&mut self, value: u32) {
        self.free_list_length = Some(value);
    }

    /// Gets the value of FreeListLength
    pub fn get_free_list_length(&self) -> Option<&u32> {
        self.free_list_length.as_ref()
    }

    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: serde_json::Value) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&serde_json::Value> {
        self.free_space.as_ref()
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

    /// Sets the value of LargeUCRSpace
    pub fn set_large_ucrspace(&mut self, value: serde_json::Value) {
        self.large_ucrspace = Some(value);
    }

    /// Gets the value of LargeUCRSpace
    pub fn get_large_ucrspace(&self) -> Option<&serde_json::Value> {
        self.large_ucrspace.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ReservedSpace
    pub fn set_reserved_space(&mut self, value: serde_json::Value) {
        self.reserved_space = Some(value);
    }

    /// Gets the value of ReservedSpace
    pub fn get_reserved_space(&self) -> Option<&serde_json::Value> {
        self.reserved_space.as_ref()
    }

    /// Sets the value of UCRLength
    pub fn set_ucrlength(&mut self, value: u32) {
        self.ucrlength = Some(value);
    }

    /// Gets the value of UCRLength
    pub fn get_ucrlength(&self) -> Option<&u32> {
        self.ucrlength.as_ref()
    }
}

