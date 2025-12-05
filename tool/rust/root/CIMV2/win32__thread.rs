// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Thread struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Thread {
    #[serde(flatten)]
    pub base: CIM_Thread,

/// 
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<u64>,

/// 
    #[serde(rename = "PriorityBase")]
    pub priority_base: Option<u32>,

/// 
    #[serde(rename = "StartAddress")]
    pub start_address: Option<u32>,

/// 
    #[serde(rename = "ThreadState")]
    pub thread_state: Option<u32>,

/// 
    #[serde(rename = "ThreadWaitReason")]
    pub thread_wait_reason: Option<u32>,
}

impl Win32_Thread {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Thread::new(),
            elapsed_time: None,
            priority_base: None,
            start_address: None,
            thread_state: None,
            thread_wait_reason: None,
        }
    }


    /// Sets the value of ElapsedTime
    pub fn set_elapsed_time(&mut self, value: u64) {
        self.elapsed_time = Some(value);
    }

    /// Gets the value of ElapsedTime
    pub fn get_elapsed_time(&self) -> Option<&u64> {
        self.elapsed_time.as_ref()
    }

    /// Sets the value of PriorityBase
    pub fn set_priority_base(&mut self, value: u32) {
        self.priority_base = Some(value);
    }

    /// Gets the value of PriorityBase
    pub fn get_priority_base(&self) -> Option<&u32> {
        self.priority_base.as_ref()
    }

    /// Sets the value of StartAddress
    pub fn set_start_address(&mut self, value: u32) {
        self.start_address = Some(value);
    }

    /// Gets the value of StartAddress
    pub fn get_start_address(&self) -> Option<&u32> {
        self.start_address.as_ref()
    }

    /// Sets the value of ThreadState
    pub fn set_thread_state(&mut self, value: u32) {
        self.thread_state = Some(value);
    }

    /// Gets the value of ThreadState
    pub fn get_thread_state(&self) -> Option<&u32> {
        self.thread_state.as_ref()
    }

    /// Sets the value of ThreadWaitReason
    pub fn set_thread_wait_reason(&mut self, value: u32) {
        self.thread_wait_reason = Some(value);
    }

    /// Gets the value of ThreadWaitReason
    pub fn get_thread_wait_reason(&self) -> Option<&u32> {
        self.thread_wait_reason.as_ref()
    }
}

