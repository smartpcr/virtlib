// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SpinLock struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpinLock {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "AcquireDepth")]
    pub acquire_depth: Option<u8>,

/// 
    #[serde(rename = "AcquireTime")]
    pub acquire_time: Option<u64>,

/// 
    #[serde(rename = "CallerAddress")]
    pub caller_address: Option<u32>,

/// 
    #[serde(rename = "Flag")]
    pub flag: Option<u8>,

/// 
    #[serde(rename = "InterruptCount")]
    pub interrupt_count: Option<u32>,

/// 
    #[serde(rename = "Irql")]
    pub irql: Option<u8>,

/// 
    #[serde(rename = "ReleaseTime")]
    pub release_time: Option<u64>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Vec<u8>,

/// 
    #[serde(rename = "SpinCount")]
    pub spin_count: Option<u32>,

/// 
    #[serde(rename = "SpinLockAddress")]
    pub spin_lock_address: Option<u32>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,

/// 
    #[serde(rename = "WaitTimeInCycles")]
    pub wait_time_in_cycles: Option<u32>,
}

impl SpinLock {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            acquire_depth: None,
            acquire_time: None,
            caller_address: None,
            flag: None,
            interrupt_count: None,
            irql: None,
            release_time: None,
            reserved: Vec::new(),
            spin_count: None,
            spin_lock_address: None,
            thread_id: None,
            wait_time_in_cycles: None,
        }
    }


    /// Sets the value of AcquireDepth
    pub fn set_acquire_depth(&mut self, value: u8) {
        self.acquire_depth = Some(value);
    }

    /// Gets the value of AcquireDepth
    pub fn get_acquire_depth(&self) -> Option<&u8> {
        self.acquire_depth.as_ref()
    }

    /// Sets the value of AcquireTime
    pub fn set_acquire_time(&mut self, value: u64) {
        self.acquire_time = Some(value);
    }

    /// Gets the value of AcquireTime
    pub fn get_acquire_time(&self) -> Option<&u64> {
        self.acquire_time.as_ref()
    }

    /// Sets the value of CallerAddress
    pub fn set_caller_address(&mut self, value: u32) {
        self.caller_address = Some(value);
    }

    /// Gets the value of CallerAddress
    pub fn get_caller_address(&self) -> Option<&u32> {
        self.caller_address.as_ref()
    }

    /// Sets the value of Flag
    pub fn set_flag(&mut self, value: u8) {
        self.flag = Some(value);
    }

    /// Gets the value of Flag
    pub fn get_flag(&self) -> Option<&u8> {
        self.flag.as_ref()
    }

    /// Sets the value of InterruptCount
    pub fn set_interrupt_count(&mut self, value: u32) {
        self.interrupt_count = Some(value);
    }

    /// Gets the value of InterruptCount
    pub fn get_interrupt_count(&self) -> Option<&u32> {
        self.interrupt_count.as_ref()
    }

    /// Sets the value of Irql
    pub fn set_irql(&mut self, value: u8) {
        self.irql = Some(value);
    }

    /// Gets the value of Irql
    pub fn get_irql(&self) -> Option<&u8> {
        self.irql.as_ref()
    }

    /// Sets the value of ReleaseTime
    pub fn set_release_time(&mut self, value: u64) {
        self.release_time = Some(value);
    }

    /// Gets the value of ReleaseTime
    pub fn get_release_time(&self) -> Option<&u64> {
        self.release_time.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: Vec<u8>) {
        self.reserved = value;
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> &Vec<u8> {
        &self.reserved
    }

    /// Sets the value of SpinCount
    pub fn set_spin_count(&mut self, value: u32) {
        self.spin_count = Some(value);
    }

    /// Gets the value of SpinCount
    pub fn get_spin_count(&self) -> Option<&u32> {
        self.spin_count.as_ref()
    }

    /// Sets the value of SpinLockAddress
    pub fn set_spin_lock_address(&mut self, value: u32) {
        self.spin_lock_address = Some(value);
    }

    /// Gets the value of SpinLockAddress
    pub fn get_spin_lock_address(&self) -> Option<&u32> {
        self.spin_lock_address.as_ref()
    }

    /// Sets the value of ThreadId
    pub fn set_thread_id(&mut self, value: u32) {
        self.thread_id = Some(value);
    }

    /// Gets the value of ThreadId
    pub fn get_thread_id(&self) -> Option<&u32> {
        self.thread_id.as_ref()
    }

    /// Sets the value of WaitTimeInCycles
    pub fn set_wait_time_in_cycles(&mut self, value: u32) {
        self.wait_time_in_cycles = Some(value);
    }

    /// Gets the value of WaitTimeInCycles
    pub fn get_wait_time_in_cycles(&self) -> Option<&u32> {
        self.wait_time_in_cycles.as_ref()
    }
}

