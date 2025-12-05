// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskDailyTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskDailyTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,

/// 
    #[serde(rename = "DaysInterval")]
    pub days_interval: Option<i16>,

/// 
    #[serde(rename = "RandomDelay")]
    pub random_delay: Option<String>,
}

impl MSFT_TaskDailyTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
            days_interval: None,
            random_delay: None,
        }
    }


    /// Sets the value of DaysInterval
    pub fn set_days_interval(&mut self, value: i16) {
        self.days_interval = Some(value);
    }

    /// Gets the value of DaysInterval
    pub fn get_days_interval(&self) -> Option<&i16> {
        self.days_interval.as_ref()
    }

    /// Sets the value of RandomDelay
    pub fn set_random_delay(&mut self, value: String) {
        self.random_delay = Some(value);
    }

    /// Gets the value of RandomDelay
    pub fn get_random_delay(&self) -> Option<&String> {
        self.random_delay.as_ref()
    }
}

