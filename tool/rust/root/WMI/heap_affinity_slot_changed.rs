// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapAffinitySlotChanged struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapAffinitySlotChanged {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "Heap")]
    pub heap: Option<u32>,

/// 
    #[serde(rename = "SlotIndex")]
    pub slot_index: Option<u32>,

/// 
    #[serde(rename = "SubSegment")]
    pub sub_segment: Option<u32>,
}

impl HeapAffinitySlotChanged {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            heap: None,
            slot_index: None,
            sub_segment: None,
        }
    }


    /// Sets the value of Heap
    pub fn set_heap(&mut self, value: u32) {
        self.heap = Some(value);
    }

    /// Gets the value of Heap
    pub fn get_heap(&self) -> Option<&u32> {
        self.heap.as_ref()
    }

    /// Sets the value of SlotIndex
    pub fn set_slot_index(&mut self, value: u32) {
        self.slot_index = Some(value);
    }

    /// Gets the value of SlotIndex
    pub fn get_slot_index(&self) -> Option<&u32> {
        self.slot_index.as_ref()
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

