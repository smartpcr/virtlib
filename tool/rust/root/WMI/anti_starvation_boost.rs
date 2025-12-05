// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AntiStarvationBoost struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AntiStarvationBoost {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u8>,

/// 
    #[serde(rename = "ProcessorIndex")]
    pub processor_index: Option<u16>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u8>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl AntiStarvationBoost {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            priority: None,
            processor_index: None,
            reserved: None,
            thread_id: None,
        }
    }


    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u8) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u8> {
        self.priority.as_ref()
    }

    /// Sets the value of ProcessorIndex
    pub fn set_processor_index(&mut self, value: u16) {
        self.processor_index = Some(value);
    }

    /// Gets the value of ProcessorIndex
    pub fn get_processor_index(&self) -> Option<&u16> {
        self.processor_index.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u8) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u8> {
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

