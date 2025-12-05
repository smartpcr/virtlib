// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_TimerExpiration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_TimerExpiration {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "DueTime")]
    pub due_time: Option<u64>,

/// 
    #[serde(rename = "Period")]
    pub period: Option<u32>,

/// 
    #[serde(rename = "SubQueue")]
    pub sub_queue: Option<u32>,

/// 
    #[serde(rename = "Timer")]
    pub timer: Option<u32>,

/// 
    #[serde(rename = "WindowLength")]
    pub window_length: Option<u32>,
}

impl TP_V2_TimerExpiration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            due_time: None,
            period: None,
            sub_queue: None,
            timer: None,
            window_length: None,
        }
    }


    /// Sets the value of DueTime
    pub fn set_due_time(&mut self, value: u64) {
        self.due_time = Some(value);
    }

    /// Gets the value of DueTime
    pub fn get_due_time(&self) -> Option<&u64> {
        self.due_time.as_ref()
    }

    /// Sets the value of Period
    pub fn set_period(&mut self, value: u32) {
        self.period = Some(value);
    }

    /// Gets the value of Period
    pub fn get_period(&self) -> Option<&u32> {
        self.period.as_ref()
    }

    /// Sets the value of SubQueue
    pub fn set_sub_queue(&mut self, value: u32) {
        self.sub_queue = Some(value);
    }

    /// Gets the value of SubQueue
    pub fn get_sub_queue(&self) -> Option<&u32> {
        self.sub_queue.as_ref()
    }

    /// Sets the value of Timer
    pub fn set_timer(&mut self, value: u32) {
        self.timer = Some(value);
    }

    /// Gets the value of Timer
    pub fn get_timer(&self) -> Option<&u32> {
        self.timer.as_ref()
    }

    /// Sets the value of WindowLength
    pub fn set_window_length(&mut self, value: u32) {
        self.window_length = Some(value);
    }

    /// Gets the value of WindowLength
    pub fn get_window_length(&self) -> Option<&u32> {
        self.window_length.as_ref()
    }
}

