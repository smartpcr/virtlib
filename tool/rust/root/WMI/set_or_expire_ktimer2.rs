// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SetOrExpireKTimer2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetOrExpireKTimer2 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "Callback")]
    pub callback: Option<u32>,

/// 
    #[serde(rename = "CallbackContext")]
    pub callback_context: Option<u32>,

/// 
    #[serde(rename = "DueTime")]
    pub due_time: Option<u64>,

/// 
    #[serde(rename = "MaximumDueTime")]
    pub maximum_due_time: Option<u64>,

/// 
    #[serde(rename = "Period")]
    pub period: Option<u64>,

/// 
    #[serde(rename = "Timer")]
    pub timer: Option<u32>,

/// 
    #[serde(rename = "TimerFlags")]
    pub timer_flags: Option<u8>,
}

impl SetOrExpireKTimer2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            callback: None,
            callback_context: None,
            due_time: None,
            maximum_due_time: None,
            period: None,
            timer: None,
            timer_flags: None,
        }
    }


    /// Sets the value of Callback
    pub fn set_callback(&mut self, value: u32) {
        self.callback = Some(value);
    }

    /// Gets the value of Callback
    pub fn get_callback(&self) -> Option<&u32> {
        self.callback.as_ref()
    }

    /// Sets the value of CallbackContext
    pub fn set_callback_context(&mut self, value: u32) {
        self.callback_context = Some(value);
    }

    /// Gets the value of CallbackContext
    pub fn get_callback_context(&self) -> Option<&u32> {
        self.callback_context.as_ref()
    }

    /// Sets the value of DueTime
    pub fn set_due_time(&mut self, value: u64) {
        self.due_time = Some(value);
    }

    /// Gets the value of DueTime
    pub fn get_due_time(&self) -> Option<&u64> {
        self.due_time.as_ref()
    }

    /// Sets the value of MaximumDueTime
    pub fn set_maximum_due_time(&mut self, value: u64) {
        self.maximum_due_time = Some(value);
    }

    /// Gets the value of MaximumDueTime
    pub fn get_maximum_due_time(&self) -> Option<&u64> {
        self.maximum_due_time.as_ref()
    }

    /// Sets the value of Period
    pub fn set_period(&mut self, value: u64) {
        self.period = Some(value);
    }

    /// Gets the value of Period
    pub fn get_period(&self) -> Option<&u64> {
        self.period.as_ref()
    }

    /// Sets the value of Timer
    pub fn set_timer(&mut self, value: u32) {
        self.timer = Some(value);
    }

    /// Gets the value of Timer
    pub fn get_timer(&self) -> Option<&u32> {
        self.timer.as_ref()
    }

    /// Sets the value of TimerFlags
    pub fn set_timer_flags(&mut self, value: u8) {
        self.timer_flags = Some(value);
    }

    /// Gets the value of TimerFlags
    pub fn get_timer_flags(&self) -> Option<&u8> {
        self.timer_flags.as_ref()
    }
}

