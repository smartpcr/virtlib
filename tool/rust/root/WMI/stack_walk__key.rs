// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// StackWalk_Key struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StackWalk_Key {
    #[serde(flatten)]
    pub base: StackWalk,

/// 
    #[serde(rename = "EventTimeStamp")]
    pub event_time_stamp: Option<u64>,

/// 
    #[serde(rename = "StackKey")]
    pub stack_key: Option<u32>,

/// 
    #[serde(rename = "StackProcess")]
    pub stack_process: Option<u32>,

/// 
    #[serde(rename = "StackThread")]
    pub stack_thread: Option<u32>,
}

impl StackWalk_Key {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: StackWalk::new(),
            event_time_stamp: None,
            stack_key: None,
            stack_process: None,
            stack_thread: None,
        }
    }


    /// Sets the value of EventTimeStamp
    pub fn set_event_time_stamp(&mut self, value: u64) {
        self.event_time_stamp = Some(value);
    }

    /// Gets the value of EventTimeStamp
    pub fn get_event_time_stamp(&self) -> Option<&u64> {
        self.event_time_stamp.as_ref()
    }

    /// Sets the value of StackKey
    pub fn set_stack_key(&mut self, value: u32) {
        self.stack_key = Some(value);
    }

    /// Gets the value of StackKey
    pub fn get_stack_key(&self) -> Option<&u32> {
        self.stack_key.as_ref()
    }

    /// Sets the value of StackProcess
    pub fn set_stack_process(&mut self, value: u32) {
        self.stack_process = Some(value);
    }

    /// Gets the value of StackProcess
    pub fn get_stack_process(&self) -> Option<&u32> {
        self.stack_process.as_ref()
    }

    /// Sets the value of StackThread
    pub fn set_stack_thread(&mut self, value: u32) {
        self.stack_thread = Some(value);
    }

    /// Gets the value of StackThread
    pub fn get_stack_thread(&self) -> Option<&u32> {
        self.stack_thread.as_ref()
    }
}

