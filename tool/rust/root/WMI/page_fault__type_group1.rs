// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PageFault_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageFault_TypeGroup1 {
    #[serde(flatten)]
    pub base: PageFault_V2,

/// 
    #[serde(rename = "ProgramCounter")]
    pub program_counter: Option<u32>,

/// 
    #[serde(rename = "VirtualAddress")]
    pub virtual_address: Option<u32>,
}

impl PageFault_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PageFault_V2::new(),
            program_counter: None,
            virtual_address: None,
        }
    }


    /// Sets the value of ProgramCounter
    pub fn set_program_counter(&mut self, value: u32) {
        self.program_counter = Some(value);
    }

    /// Gets the value of ProgramCounter
    pub fn get_program_counter(&self) -> Option<&u32> {
        self.program_counter.as_ref()
    }

    /// Sets the value of VirtualAddress
    pub fn set_virtual_address(&mut self, value: u32) {
        self.virtual_address = Some(value);
    }

    /// Gets the value of VirtualAddress
    pub fn get_virtual_address(&self) -> Option<&u32> {
        self.virtual_address.as_ref()
    }
}

