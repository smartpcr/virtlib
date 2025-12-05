// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UmsContextSwitch struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UmsContextSwitch {
    #[serde(flatten)]
    pub base: UmsEvent,

/// 
    #[serde(rename = "KernelYieldCount")]
    pub kernel_yield_count: Option<u32>,

/// 
    #[serde(rename = "MixedYieldCount")]
    pub mixed_yield_count: Option<u32>,

/// 
    #[serde(rename = "ScheduledThreadId")]
    pub scheduled_thread_id: Option<u32>,

/// 
    #[serde(rename = "SwitchCount")]
    pub switch_count: Option<u32>,

/// 
    #[serde(rename = "YieldCount")]
    pub yield_count: Option<u32>,
}

impl UmsContextSwitch {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: UmsEvent::new(),
            kernel_yield_count: None,
            mixed_yield_count: None,
            scheduled_thread_id: None,
            switch_count: None,
            yield_count: None,
        }
    }


    /// Sets the value of KernelYieldCount
    pub fn set_kernel_yield_count(&mut self, value: u32) {
        self.kernel_yield_count = Some(value);
    }

    /// Gets the value of KernelYieldCount
    pub fn get_kernel_yield_count(&self) -> Option<&u32> {
        self.kernel_yield_count.as_ref()
    }

    /// Sets the value of MixedYieldCount
    pub fn set_mixed_yield_count(&mut self, value: u32) {
        self.mixed_yield_count = Some(value);
    }

    /// Gets the value of MixedYieldCount
    pub fn get_mixed_yield_count(&self) -> Option<&u32> {
        self.mixed_yield_count.as_ref()
    }

    /// Sets the value of ScheduledThreadId
    pub fn set_scheduled_thread_id(&mut self, value: u32) {
        self.scheduled_thread_id = Some(value);
    }

    /// Gets the value of ScheduledThreadId
    pub fn get_scheduled_thread_id(&self) -> Option<&u32> {
        self.scheduled_thread_id.as_ref()
    }

    /// Sets the value of SwitchCount
    pub fn set_switch_count(&mut self, value: u32) {
        self.switch_count = Some(value);
    }

    /// Gets the value of SwitchCount
    pub fn get_switch_count(&self) -> Option<&u32> {
        self.switch_count.as_ref()
    }

    /// Sets the value of YieldCount
    pub fn set_yield_count(&mut self, value: u32) {
        self.yield_count = Some(value);
    }

    /// Gets the value of YieldCount
    pub fn get_yield_count(&self) -> Option<&u32> {
        self.yield_count.as_ref()
    }
}

