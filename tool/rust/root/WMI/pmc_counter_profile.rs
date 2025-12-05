// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PmcCounterProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PmcCounterProfile {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "InstructionPointer")]
    pub instruction_pointer: Option<u32>,

/// 
    #[serde(rename = "ProfileSource")]
    pub profile_source: Option<u16>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u16>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl PmcCounterProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            instruction_pointer: None,
            profile_source: None,
            reserved: None,
            thread_id: None,
        }
    }


    /// Sets the value of InstructionPointer
    pub fn set_instruction_pointer(&mut self, value: u32) {
        self.instruction_pointer = Some(value);
    }

    /// Gets the value of InstructionPointer
    pub fn get_instruction_pointer(&self) -> Option<&u32> {
        self.instruction_pointer.as_ref()
    }

    /// Sets the value of ProfileSource
    pub fn set_profile_source(&mut self, value: u16) {
        self.profile_source = Some(value);
    }

    /// Gets the value of ProfileSource
    pub fn get_profile_source(&self) -> Option<&u16> {
        self.profile_source.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u16) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u16> {
        self.reserved.as_ref()
    }

    /// Sets the value of ThreadId
    pub fn set_thread_id(&mut self, value: u32) {
        self.thread_id = Some(value);
    }

    /// Gets the value of ThreadId
    pub fn get_thread_id(&self) -> Option<&u32> {
        self.thread_id.as_ref()
    }
}

