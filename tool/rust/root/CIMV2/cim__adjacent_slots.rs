// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_AdjacentSlots struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_AdjacentSlots {

/// 
    #[serde(rename = "DistanceBetweenSlots")]
    pub distance_between_slots: Option<f32>,

/// 
    #[serde(rename = "SharedSlots")]
    pub shared_slots: Option<bool>,

/// 
    #[serde(rename = "SlotA")]
    pub slot_a: Option<CIM_Slot>,

/// 
    #[serde(rename = "SlotB")]
    pub slot_b: Option<CIM_Slot>,
}

impl CIM_AdjacentSlots {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            distance_between_slots: None,
            shared_slots: None,
            slot_a: None,
            slot_b: None,
        }
    }


    /// Sets the value of DistanceBetweenSlots
    pub fn set_distance_between_slots(&mut self, value: f32) {
        self.distance_between_slots = Some(value);
    }

    /// Gets the value of DistanceBetweenSlots
    pub fn get_distance_between_slots(&self) -> Option<&f32> {
        self.distance_between_slots.as_ref()
    }

    /// Sets the value of SharedSlots
    pub fn set_shared_slots(&mut self, value: bool) {
        self.shared_slots = Some(value);
    }

    /// Gets the value of SharedSlots
    pub fn get_shared_slots(&self) -> Option<&bool> {
        self.shared_slots.as_ref()
    }

    /// Sets the value of SlotA
    pub fn set_slot_a(&mut self, value: CIM_Slot) {
        self.slot_a = Some(value);
    }

    /// Gets the value of SlotA
    pub fn get_slot_a(&self) -> Option<&CIM_Slot> {
        self.slot_a.as_ref()
    }

    /// Sets the value of SlotB
    pub fn set_slot_b(&mut self, value: CIM_Slot) {
        self.slot_b = Some(value);
    }

    /// Gets the value of SlotB
    pub fn get_slot_b(&self) -> Option<&CIM_Slot> {
        self.slot_b.as_ref()
    }
}

