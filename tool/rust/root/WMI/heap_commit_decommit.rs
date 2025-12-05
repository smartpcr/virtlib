// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapCommitDecommit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapCommitDecommit {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "Block")]
    pub block: Option<u32>,

/// 
    #[serde(rename = "Caller")]
    pub caller: Option<u32>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<serde_json::Value>,
}

impl HeapCommitDecommit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            block: None,
            caller: None,
            heap_handle: None,
            size: None,
        }
    }


    /// Sets the value of Block
    pub fn set_block(&mut self, value: u32) {
        self.block = Some(value);
    }

    /// Gets the value of Block
    pub fn get_block(&self) -> Option<&u32> {
        self.block.as_ref()
    }

    /// Sets the value of Caller
    pub fn set_caller(&mut self, value: u32) {
        self.caller = Some(value);
    }

    /// Gets the value of Caller
    pub fn get_caller(&self) -> Option<&u32> {
        self.caller.as_ref()
    }

    /// Sets the value of HeapHandle
    pub fn set_heap_handle(&mut self, value: u32) {
        self.heap_handle = Some(value);
    }

    /// Gets the value of HeapHandle
    pub fn get_heap_handle(&self) -> Option<&u32> {
        self.heap_handle.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: serde_json::Value) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&serde_json::Value> {
        self.size.as_ref()
    }
}

