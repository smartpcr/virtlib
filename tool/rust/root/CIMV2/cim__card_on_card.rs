// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CardOnCard struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CardOnCard {
    #[serde(flatten)]
    pub base: CIM_Container,

/// 
    #[serde(rename = "MountOrSlotDescription")]
    pub mount_or_slot_description: Option<String>,
}

impl CIM_CardOnCard {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Container::new(),
            mount_or_slot_description: None,
        }
    }


    /// Sets the value of MountOrSlotDescription
    pub fn set_mount_or_slot_description(&mut self, value: String) {
        self.mount_or_slot_description = Some(value);
    }

    /// Gets the value of MountOrSlotDescription
    pub fn get_mount_or_slot_description(&self) -> Option<&String> {
        self.mount_or_slot_description.as_ref()
    }
}

