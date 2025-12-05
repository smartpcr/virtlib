// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISR_MSI struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISR_MSI {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "InitialTime")]
    pub initial_time: Option<serde_json::Value>,

/// 
    #[serde(rename = "MessageNumber")]
    pub message_number: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,

/// 
    #[serde(rename = "ReturnValue")]
    pub return_value: Option<u8>,

/// 
    #[serde(rename = "Routine")]
    pub routine: Option<u32>,

/// 
    #[serde(rename = "Vector")]
    pub vector: Option<u16>,
}

impl ISR_MSI {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            initial_time: None,
            message_number: None,
            reserved: None,
            return_value: None,
            routine: None,
            vector: None,
        }
    }


    /// Sets the value of InitialTime
    pub fn set_initial_time(&mut self, value: serde_json::Value) {
        self.initial_time = Some(value);
    }

    /// Gets the value of InitialTime
    pub fn get_initial_time(&self) -> Option<&serde_json::Value> {
        self.initial_time.as_ref()
    }

    /// Sets the value of MessageNumber
    pub fn set_message_number(&mut self, value: u32) {
        self.message_number = Some(value);
    }

    /// Gets the value of MessageNumber
    pub fn get_message_number(&self) -> Option<&u32> {
        self.message_number.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u8) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u8> {
        self.reserved.as_ref()
    }

    /// Sets the value of ReturnValue
    pub fn set_return_value(&mut self, value: u8) {
        self.return_value = Some(value);
    }

    /// Gets the value of ReturnValue
    pub fn get_return_value(&self) -> Option<&u8> {
        self.return_value.as_ref()
    }

    /// Sets the value of Routine
    pub fn set_routine(&mut self, value: u32) {
        self.routine = Some(value);
    }

    /// Gets the value of Routine
    pub fn get_routine(&self) -> Option<&u32> {
        self.routine.as_ref()
    }

    /// Sets the value of Vector
    pub fn set_vector(&mut self, value: u16) {
        self.vector = Some(value);
    }

    /// Gets the value of Vector
    pub fn get_vector(&self) -> Option<&u16> {
        self.vector.as_ref()
    }
}

