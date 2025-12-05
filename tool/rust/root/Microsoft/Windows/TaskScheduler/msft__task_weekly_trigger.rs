// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskWeeklyTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskWeeklyTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,

/// 
    #[serde(rename = "DaysOfWeek")]
    pub days_of_week: Option<u16>,

/// 
    #[serde(rename = "RandomDelay")]
    pub random_delay: Option<String>,

/// 
    #[serde(rename = "WeeksInterval")]
    pub weeks_interval: Option<u16>,
}

impl MSFT_TaskWeeklyTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
            days_of_week: None,
            random_delay: None,
            weeks_interval: None,
        }
    }


    /// Sets the value of DaysOfWeek
    pub fn set_days_of_week(&mut self, value: u16) {
        self.days_of_week = Some(value);
    }

    /// Gets the value of DaysOfWeek
    pub fn get_days_of_week(&self) -> Option<&u16> {
        self.days_of_week.as_ref()
    }

    /// Sets the value of RandomDelay
    pub fn set_random_delay(&mut self, value: String) {
        self.random_delay = Some(value);
    }

    /// Gets the value of RandomDelay
    pub fn get_random_delay(&self) -> Option<&String> {
        self.random_delay.as_ref()
    }

    /// Sets the value of WeeksInterval
    pub fn set_weeks_interval(&mut self, value: u16) {
        self.weeks_interval = Some(value);
    }

    /// Gets the value of WeeksInterval
    pub fn get_weeks_interval(&self) -> Option<&u16> {
        self.weeks_interval.as_ref()
    }
}

