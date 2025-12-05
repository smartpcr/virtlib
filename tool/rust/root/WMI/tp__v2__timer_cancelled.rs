// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TP_V2_TimerCancelled struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TP_V2_TimerCancelled {
    #[serde(flatten)]
    pub base: ThreadPoolTrace_V2,

/// 
    #[serde(rename = "SubQueue")]
    pub sub_queue: Option<u32>,

/// 
    #[serde(rename = "Timer")]
    pub timer: Option<u32>,
}

impl TP_V2_TimerCancelled {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ThreadPoolTrace_V2::new(),
            sub_queue: None,
            timer: None,
        }
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
}

