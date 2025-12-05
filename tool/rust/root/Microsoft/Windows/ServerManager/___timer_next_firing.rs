// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __TimerNextFiring struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __TimerNextFiring {
    #[serde(flatten)]
    pub base: __IndicationRelated,

/// 
    #[serde(rename = "NextEvent64BitTime")]
    pub next_event64_bit_time: Option<i64>,

/// 
    #[serde(rename = "TimerId")]
    pub timer_id: Option<String>,
}

impl __TimerNextFiring {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __IndicationRelated::new(),
            next_event64_bit_time: None,
            timer_id: None,
        }
    }


    /// Sets the value of NextEvent64BitTime
    pub fn set_next_event64_bit_time(&mut self, value: i64) {
        self.next_event64_bit_time = Some(value);
    }

    /// Gets the value of NextEvent64BitTime
    pub fn get_next_event64_bit_time(&self) -> Option<&i64> {
        self.next_event64_bit_time.as_ref()
    }

    /// Sets the value of TimerId
    pub fn set_timer_id(&mut self, value: String) {
        self.timer_id = Some(value);
    }

    /// Gets the value of TimerId
    pub fn get_timer_id(&self) -> Option<&String> {
        self.timer_id.as_ref()
    }
}

