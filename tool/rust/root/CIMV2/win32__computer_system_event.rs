// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ComputerSystemEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ComputerSystemEvent {
    #[serde(flatten)]
    pub base: __ExtrinsicEvent,

/// 
    #[serde(rename = "MachineName")]
    pub machine_name: Option<String>,
}

impl Win32_ComputerSystemEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __ExtrinsicEvent::new(),
            machine_name: None,
        }
    }


    /// Sets the value of MachineName
    pub fn set_machine_name(&mut self, value: String) {
        self.machine_name = Some(value);
    }

    /// Gets the value of MachineName
    pub fn get_machine_name(&self) -> Option<&String> {
        self.machine_name.as_ref()
    }
}

