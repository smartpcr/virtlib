// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CSwitch_V1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CSwitch_V1 {
    #[serde(flatten)]
    pub base: Thread_V1,

/// 
    #[serde(rename = "NewThreadId")]
    pub new_thread_id: Option<u32>,

/// 
    #[serde(rename = "NewThreadPriority")]
    pub new_thread_priority: Option<u8>,

/// 
    #[serde(rename = "NewThreadQuantum")]
    pub new_thread_quantum: Option<u8>,

/// 
    #[serde(rename = "OldThreadId")]
    pub old_thread_id: Option<u32>,

/// 
    #[serde(rename = "OldThreadPriority")]
    pub old_thread_priority: Option<u8>,

/// 
    #[serde(rename = "OldThreadQuantum")]
    pub old_thread_quantum: Option<u8>,

/// 
    #[serde(rename = "OldThreadState")]
    pub old_thread_state: Option<u8>,

/// 
    #[serde(rename = "OldThreadWaitIdealProcessor")]
    pub old_thread_wait_ideal_processor: Option<u8>,

/// 
    #[serde(rename = "OldThreadWaitMode")]
    pub old_thread_wait_mode: Option<u8>,

/// 
    #[serde(rename = "OldThreadWaitReason")]
    pub old_thread_wait_reason: Option<u8>,
}

impl CSwitch_V1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V1::new(),
            new_thread_id: None,
            new_thread_priority: None,
            new_thread_quantum: None,
            old_thread_id: None,
            old_thread_priority: None,
            old_thread_quantum: None,
            old_thread_state: None,
            old_thread_wait_ideal_processor: None,
            old_thread_wait_mode: None,
            old_thread_wait_reason: None,
        }
    }


    /// Sets the value of NewThreadId
    pub fn set_new_thread_id(&mut self, value: u32) {
        self.new_thread_id = Some(value);
    }

    /// Gets the value of NewThreadId
    pub fn get_new_thread_id(&self) -> Option<&u32> {
        self.new_thread_id.as_ref()
    }

    /// Sets the value of NewThreadPriority
    pub fn set_new_thread_priority(&mut self, value: u8) {
        self.new_thread_priority = Some(value);
    }

    /// Gets the value of NewThreadPriority
    pub fn get_new_thread_priority(&self) -> Option<&u8> {
        self.new_thread_priority.as_ref()
    }

    /// Sets the value of NewThreadQuantum
    pub fn set_new_thread_quantum(&mut self, value: u8) {
        self.new_thread_quantum = Some(value);
    }

    /// Gets the value of NewThreadQuantum
    pub fn get_new_thread_quantum(&self) -> Option<&u8> {
        self.new_thread_quantum.as_ref()
    }

    /// Sets the value of OldThreadId
    pub fn set_old_thread_id(&mut self, value: u32) {
        self.old_thread_id = Some(value);
    }

    /// Gets the value of OldThreadId
    pub fn get_old_thread_id(&self) -> Option<&u32> {
        self.old_thread_id.as_ref()
    }

    /// Sets the value of OldThreadPriority
    pub fn set_old_thread_priority(&mut self, value: u8) {
        self.old_thread_priority = Some(value);
    }

    /// Gets the value of OldThreadPriority
    pub fn get_old_thread_priority(&self) -> Option<&u8> {
        self.old_thread_priority.as_ref()
    }

    /// Sets the value of OldThreadQuantum
    pub fn set_old_thread_quantum(&mut self, value: u8) {
        self.old_thread_quantum = Some(value);
    }

    /// Gets the value of OldThreadQuantum
    pub fn get_old_thread_quantum(&self) -> Option<&u8> {
        self.old_thread_quantum.as_ref()
    }

    /// Sets the value of OldThreadState
    pub fn set_old_thread_state(&mut self, value: u8) {
        self.old_thread_state = Some(value);
    }

    /// Gets the value of OldThreadState
    pub fn get_old_thread_state(&self) -> Option<&u8> {
        self.old_thread_state.as_ref()
    }

    /// Sets the value of OldThreadWaitIdealProcessor
    pub fn set_old_thread_wait_ideal_processor(&mut self, value: u8) {
        self.old_thread_wait_ideal_processor = Some(value);
    }

    /// Gets the value of OldThreadWaitIdealProcessor
    pub fn get_old_thread_wait_ideal_processor(&self) -> Option<&u8> {
        self.old_thread_wait_ideal_processor.as_ref()
    }

    /// Sets the value of OldThreadWaitMode
    pub fn set_old_thread_wait_mode(&mut self, value: u8) {
        self.old_thread_wait_mode = Some(value);
    }

    /// Gets the value of OldThreadWaitMode
    pub fn get_old_thread_wait_mode(&self) -> Option<&u8> {
        self.old_thread_wait_mode.as_ref()
    }

    /// Sets the value of OldThreadWaitReason
    pub fn set_old_thread_wait_reason(&mut self, value: u8) {
        self.old_thread_wait_reason = Some(value);
    }

    /// Gets the value of OldThreadWaitReason
    pub fn get_old_thread_wait_reason(&self) -> Option<&u8> {
        self.old_thread_wait_reason.as_ref()
    }
}

