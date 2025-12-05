// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfProc_Thread struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfProc_Thread {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ContextSwitchesPersec")]
    pub context_switches_persec: Option<u32>,

/// 
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<u64>,

/// 
    #[serde(rename = "IDProcess")]
    pub idprocess: Option<u32>,

/// 
    #[serde(rename = "IDThread")]
    pub idthread: Option<u32>,

/// 
    #[serde(rename = "PercentPrivilegedTime")]
    pub percent_privileged_time: Option<u64>,

/// 
    #[serde(rename = "PercentProcessorTime")]
    pub percent_processor_time: Option<u64>,

/// 
    #[serde(rename = "PercentUserTime")]
    pub percent_user_time: Option<u64>,

/// 
    #[serde(rename = "PriorityBase")]
    pub priority_base: Option<u32>,

/// 
    #[serde(rename = "PriorityCurrent")]
    pub priority_current: Option<u32>,

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

impl Win32_PerfFormattedData_PerfProc_Thread {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            context_switches_persec: None,
            elapsed_time: None,
            idprocess: None,
            idthread: None,
            percent_privileged_time: None,
            percent_processor_time: None,
            percent_user_time: None,
            priority_base: None,
            priority_current: None,
            start_address: None,
            thread_state: None,
            thread_wait_reason: None,
        }
    }


    /// Sets the value of ContextSwitchesPersec
    pub fn set_context_switches_persec(&mut self, value: u32) {
        self.context_switches_persec = Some(value);
    }

    /// Gets the value of ContextSwitchesPersec
    pub fn get_context_switches_persec(&self) -> Option<&u32> {
        self.context_switches_persec.as_ref()
    }

    /// Sets the value of ElapsedTime
    pub fn set_elapsed_time(&mut self, value: u64) {
        self.elapsed_time = Some(value);
    }

    /// Gets the value of ElapsedTime
    pub fn get_elapsed_time(&self) -> Option<&u64> {
        self.elapsed_time.as_ref()
    }

    /// Sets the value of IDProcess
    pub fn set_idprocess(&mut self, value: u32) {
        self.idprocess = Some(value);
    }

    /// Gets the value of IDProcess
    pub fn get_idprocess(&self) -> Option<&u32> {
        self.idprocess.as_ref()
    }

    /// Sets the value of IDThread
    pub fn set_idthread(&mut self, value: u32) {
        self.idthread = Some(value);
    }

    /// Gets the value of IDThread
    pub fn get_idthread(&self) -> Option<&u32> {
        self.idthread.as_ref()
    }

    /// Sets the value of PercentPrivilegedTime
    pub fn set_percent_privileged_time(&mut self, value: u64) {
        self.percent_privileged_time = Some(value);
    }

    /// Gets the value of PercentPrivilegedTime
    pub fn get_percent_privileged_time(&self) -> Option<&u64> {
        self.percent_privileged_time.as_ref()
    }

    /// Sets the value of PercentProcessorTime
    pub fn set_percent_processor_time(&mut self, value: u64) {
        self.percent_processor_time = Some(value);
    }

    /// Gets the value of PercentProcessorTime
    pub fn get_percent_processor_time(&self) -> Option<&u64> {
        self.percent_processor_time.as_ref()
    }

    /// Sets the value of PercentUserTime
    pub fn set_percent_user_time(&mut self, value: u64) {
        self.percent_user_time = Some(value);
    }

    /// Gets the value of PercentUserTime
    pub fn get_percent_user_time(&self) -> Option<&u64> {
        self.percent_user_time.as_ref()
    }

    /// Sets the value of PriorityBase
    pub fn set_priority_base(&mut self, value: u32) {
        self.priority_base = Some(value);
    }

    /// Gets the value of PriorityBase
    pub fn get_priority_base(&self) -> Option<&u32> {
        self.priority_base.as_ref()
    }

    /// Sets the value of PriorityCurrent
    pub fn set_priority_current(&mut self, value: u32) {
        self.priority_current = Some(value);
    }

    /// Gets the value of PriorityCurrent
    pub fn get_priority_current(&self) -> Option<&u32> {
        self.priority_current.as_ref()
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

