// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CurrentTime struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CurrentTime {

/// 
    #[serde(rename = "Day")]
    pub day: Option<u32>,

/// 
    #[serde(rename = "DayOfWeek")]
    pub day_of_week: Option<u32>,

/// 
    #[serde(rename = "Hour")]
    pub hour: Option<u32>,

/// 
    #[serde(rename = "Milliseconds")]
    pub milliseconds: Option<u32>,

/// 
    #[serde(rename = "Minute")]
    pub minute: Option<u32>,

/// 
    #[serde(rename = "Month")]
    pub month: Option<u32>,

/// 
    #[serde(rename = "Quarter")]
    pub quarter: Option<u32>,

/// 
    #[serde(rename = "Second")]
    pub second: Option<u32>,

/// 
    #[serde(rename = "WeekInMonth")]
    pub week_in_month: Option<u32>,

/// 
    #[serde(rename = "Year")]
    pub year: Option<u32>,
}

impl Win32_CurrentTime {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            day: None,
            day_of_week: None,
            hour: None,
            milliseconds: None,
            minute: None,
            month: None,
            quarter: None,
            second: None,
            week_in_month: None,
            year: None,
        }
    }


    /// Sets the value of Day
    pub fn set_day(&mut self, value: u32) {
        self.day = Some(value);
    }

    /// Gets the value of Day
    pub fn get_day(&self) -> Option<&u32> {
        self.day.as_ref()
    }

    /// Sets the value of DayOfWeek
    pub fn set_day_of_week(&mut self, value: u32) {
        self.day_of_week = Some(value);
    }

    /// Gets the value of DayOfWeek
    pub fn get_day_of_week(&self) -> Option<&u32> {
        self.day_of_week.as_ref()
    }

    /// Sets the value of Hour
    pub fn set_hour(&mut self, value: u32) {
        self.hour = Some(value);
    }

    /// Gets the value of Hour
    pub fn get_hour(&self) -> Option<&u32> {
        self.hour.as_ref()
    }

    /// Sets the value of Milliseconds
    pub fn set_milliseconds(&mut self, value: u32) {
        self.milliseconds = Some(value);
    }

    /// Gets the value of Milliseconds
    pub fn get_milliseconds(&self) -> Option<&u32> {
        self.milliseconds.as_ref()
    }

    /// Sets the value of Minute
    pub fn set_minute(&mut self, value: u32) {
        self.minute = Some(value);
    }

    /// Gets the value of Minute
    pub fn get_minute(&self) -> Option<&u32> {
        self.minute.as_ref()
    }

    /// Sets the value of Month
    pub fn set_month(&mut self, value: u32) {
        self.month = Some(value);
    }

    /// Gets the value of Month
    pub fn get_month(&self) -> Option<&u32> {
        self.month.as_ref()
    }

    /// Sets the value of Quarter
    pub fn set_quarter(&mut self, value: u32) {
        self.quarter = Some(value);
    }

    /// Gets the value of Quarter
    pub fn get_quarter(&self) -> Option<&u32> {
        self.quarter.as_ref()
    }

    /// Sets the value of Second
    pub fn set_second(&mut self, value: u32) {
        self.second = Some(value);
    }

    /// Gets the value of Second
    pub fn get_second(&self) -> Option<&u32> {
        self.second.as_ref()
    }

    /// Sets the value of WeekInMonth
    pub fn set_week_in_month(&mut self, value: u32) {
        self.week_in_month = Some(value);
    }

    /// Gets the value of WeekInMonth
    pub fn get_week_in_month(&self) -> Option<&u32> {
        self.week_in_month.as_ref()
    }

    /// Sets the value of Year
    pub fn set_year(&mut self, value: u32) {
        self.year = Some(value);
    }

    /// Gets the value of Year
    pub fn get_year(&self) -> Option<&u32> {
        self.year.as_ref()
    }
}

