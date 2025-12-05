// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskMonthlyTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskMonthlyTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,

/// 
    #[serde(rename = "DaysOfMonth")]
    pub days_of_month: Option<u16>,

/// 
    #[serde(rename = "MonthOfYear")]
    pub month_of_year: Option<u16>,

/// 
    #[serde(rename = "RandomDelay")]
    pub random_delay: Option<String>,

/// 
    #[serde(rename = "RunOnLastDayOfMonth")]
    pub run_on_last_day_of_month: Option<bool>,
}

impl MSFT_TaskMonthlyTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
            days_of_month: None,
            month_of_year: None,
            random_delay: None,
            run_on_last_day_of_month: None,
        }
    }


    /// Sets the value of DaysOfMonth
    pub fn set_days_of_month(&mut self, value: u16) {
        self.days_of_month = Some(value);
    }

    /// Gets the value of DaysOfMonth
    pub fn get_days_of_month(&self) -> Option<&u16> {
        self.days_of_month.as_ref()
    }

    /// Sets the value of MonthOfYear
    pub fn set_month_of_year(&mut self, value: u16) {
        self.month_of_year = Some(value);
    }

    /// Gets the value of MonthOfYear
    pub fn get_month_of_year(&self) -> Option<&u16> {
        self.month_of_year.as_ref()
    }

    /// Sets the value of RandomDelay
    pub fn set_random_delay(&mut self, value: String) {
        self.random_delay = Some(value);
    }

    /// Gets the value of RandomDelay
    pub fn get_random_delay(&self) -> Option<&String> {
        self.random_delay.as_ref()
    }

    /// Sets the value of RunOnLastDayOfMonth
    pub fn set_run_on_last_day_of_month(&mut self, value: bool) {
        self.run_on_last_day_of_month = Some(value);
    }

    /// Gets the value of RunOnLastDayOfMonth
    pub fn get_run_on_last_day_of_month(&self) -> Option<&bool> {
        self.run_on_last_day_of_month.as_ref()
    }
}

