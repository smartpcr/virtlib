// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapCreate_V2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapCreate_V2 {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "HeapFlags")]
    pub heap_flags: Option<u32>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,
}

impl HeapCreate_V2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            heap_flags: None,
            heap_handle: None,
        }
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
}

