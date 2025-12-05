// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapFree struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapFree {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "FreeAddress")]
    pub free_address: Option<u32>,

/// 
    #[serde(rename = "HeapHandle")]
    pub heap_handle: Option<u32>,

/// 
    #[serde(rename = "SourceId")]
    pub source_id: Option<u32>,
}

impl HeapFree {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            free_address: None,
            heap_handle: None,
            source_id: None,
        }
    }


    /// Sets the value of FreeAddress
    pub fn set_free_address(&mut self, value: u32) {
        self.free_address = Some(value);
    }

    /// Gets the value of FreeAddress
    pub fn get_free_address(&self) -> Option<&u32> {
        self.free_address.as_ref()
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

