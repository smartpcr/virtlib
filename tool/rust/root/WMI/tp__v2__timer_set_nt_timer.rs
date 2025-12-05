// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_TimerSetNtTimer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_TimerSetNtTimer {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "DueTime")]
    pub due_time: Option<u64>,

/// 
    #[serde(rename = "SubQueue")]
    pub sub_queue: Option<u32>,

/// 
    #[serde(rename = "TolerableDelay")]
    pub tolerable_delay: Option<u32>,
}

impl TP_V2_TimerSetNtTimer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            due_time: None,
            sub_queue: None,
            tolerable_delay: None,
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

    /// Sets the value of SubQueue
    pub fn set_sub_queue(&mut self, value: u32) {
        self.sub_queue = Some(value);
    }

    /// Gets the value of SubQueue
    pub fn get_sub_queue(&self) -> Option<&u32> {
        self.sub_queue.as_ref()
    }

    /// Sets the value of TolerableDelay
    pub fn set_tolerable_delay(&mut self, value: u32) {
        self.tolerable_delay = Some(value);
    }

    /// Gets the value of TolerableDelay
    pub fn get_tolerable_delay(&self) -> Option<&u32> {
        self.tolerable_delay.as_ref()
    }
}

