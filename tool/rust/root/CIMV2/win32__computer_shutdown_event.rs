// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ComputerShutdownEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ComputerShutdownEvent {
    #[serde(flatten)]
    pub base: Win32_ComputerSystemEvent,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl Win32_ComputerShutdownEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_ComputerSystemEvent::new(),
            type: None,
        }
    }


    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }
}

