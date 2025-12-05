// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HeapAffinitySlotAssigned struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapAffinitySlotAssigned {
    #[serde(flatten)]
    pub base: HeapTrace_V2,

/// 
    #[serde(rename = "SlotIndex")]
    pub slot_index: Option<u32>,
}

impl HeapAffinitySlotAssigned {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: HeapTrace_V2::new(),
            slot_index: None,
        }
    }


    /// Sets the value of SlotIndex
    pub fn set_slot_index(&mut self, value: u32) {
        self.slot_index = Some(value);
    }

    /// Gets the value of SlotIndex
    pub fn get_slot_index(&self) -> Option<&u32> {
        self.slot_index.as_ref()
    }
}

