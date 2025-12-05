// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TimeZone struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TimeZone {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "Bias")]
    pub bias: Option<i32>,

/// 
    #[serde(rename = "DaylightBias")]
    pub daylight_bias: Option<i32>,

/// 
    #[serde(rename = "DaylightDay")]
    pub daylight_day: Option<u32>,

/// 
    #[serde(rename = "DaylightDayOfWeek")]
    pub daylight_day_of_week: Option<u8>,

/// 
    #[serde(rename = "DaylightHour")]
    pub daylight_hour: Option<u32>,

/// 
    #[serde(rename = "DaylightMillisecond")]
    pub daylight_millisecond: Option<u32>,

/// 
    #[serde(rename = "DaylightMinute")]
    pub daylight_minute: Option<u32>,

/// 
    #[serde(rename = "DaylightMonth")]
    pub daylight_month: Option<u32>,

/// 
    #[serde(rename = "DaylightName")]
    pub daylight_name: Option<String>,

/// 
    #[serde(rename = "DaylightSecond")]
    pub daylight_second: Option<u32>,

/// 
    #[serde(rename = "DaylightYear")]
    pub daylight_year: Option<u32>,

/// 
    #[serde(rename = "StandardBias")]
    pub standard_bias: Option<u32>,

/// 
    #[serde(rename = "StandardDay")]
    pub standard_day: Option<u32>,

/// 
    #[serde(rename = "StandardDayOfWeek")]
    pub standard_day_of_week: Option<u8>,

/// 
    #[serde(rename = "StandardHour")]
    pub standard_hour: Option<u32>,

/// 
    #[serde(rename = "StandardMillisecond")]
    pub standard_millisecond: Option<u32>,

/// 
    #[serde(rename = "StandardMinute")]
    pub standard_minute: Option<u32>,

/// 
    #[serde(rename = "StandardMonth")]
    pub standard_month: Option<u32>,

/// 
    #[serde(rename = "StandardName")]
    pub standard_name: Option<String>,

/// 
    #[serde(rename = "StandardSecond")]
    pub standard_second: Option<u32>,

/// 
    #[serde(rename = "StandardYear")]
    pub standard_year: Option<u32>,
}

impl Win32_TimeZone {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            bias: None,
            daylight_bias: None,
            daylight_day: None,
            daylight_day_of_week: None,
            daylight_hour: None,
            daylight_millisecond: None,
            daylight_minute: None,
            daylight_month: None,
            daylight_name: None,
            daylight_second: None,
            daylight_year: None,
            standard_bias: None,
            standard_day: None,
            standard_day_of_week: None,
            standard_hour: None,
            standard_millisecond: None,
            standard_minute: None,
            standard_month: None,
            standard_name: None,
            standard_second: None,
            standard_year: None,
        }
    }


    /// Sets the value of Bias
    pub fn set_bias(&mut self, value: i32) {
        self.bias = Some(value);
    }

    /// Gets the value of Bias
    pub fn get_bias(&self) -> Option<&i32> {
        self.bias.as_ref()
    }

    /// Sets the value of DaylightBias
    pub fn set_daylight_bias(&mut self, value: i32) {
        self.daylight_bias = Some(value);
    }

    /// Gets the value of DaylightBias
    pub fn get_daylight_bias(&self) -> Option<&i32> {
        self.daylight_bias.as_ref()
    }

    /// Sets the value of DaylightDay
    pub fn set_daylight_day(&mut self, value: u32) {
        self.daylight_day = Some(value);
    }

    /// Gets the value of DaylightDay
    pub fn get_daylight_day(&self) -> Option<&u32> {
        self.daylight_day.as_ref()
    }

    /// Sets the value of DaylightDayOfWeek
    pub fn set_daylight_day_of_week(&mut self, value: u8) {
        self.daylight_day_of_week = Some(value);
    }

    /// Gets the value of DaylightDayOfWeek
    pub fn get_daylight_day_of_week(&self) -> Option<&u8> {
        self.daylight_day_of_week.as_ref()
    }

    /// Sets the value of DaylightHour
    pub fn set_daylight_hour(&mut self, value: u32) {
        self.daylight_hour = Some(value);
    }

    /// Gets the value of DaylightHour
    pub fn get_daylight_hour(&self) -> Option<&u32> {
        self.daylight_hour.as_ref()
    }

    /// Sets the value of DaylightMillisecond
    pub fn set_daylight_millisecond(&mut self, value: u32) {
        self.daylight_millisecond = Some(value);
    }

    /// Gets the value of DaylightMillisecond
    pub fn get_daylight_millisecond(&self) -> Option<&u32> {
        self.daylight_millisecond.as_ref()
    }

    /// Sets the value of DaylightMinute
    pub fn set_daylight_minute(&mut self, value: u32) {
        self.daylight_minute = Some(value);
    }

    /// Gets the value of DaylightMinute
    pub fn get_daylight_minute(&self) -> Option<&u32> {
        self.daylight_minute.as_ref()
    }

    /// Sets the value of DaylightMonth
    pub fn set_daylight_month(&mut self, value: u32) {
        self.daylight_month = Some(value);
    }

    /// Gets the value of DaylightMonth
    pub fn get_daylight_month(&self) -> Option<&u32> {
        self.daylight_month.as_ref()
    }

    /// Sets the value of DaylightName
    pub fn set_daylight_name(&mut self, value: String) {
        self.daylight_name = Some(value);
    }

    /// Gets the value of DaylightName
    pub fn get_daylight_name(&self) -> Option<&String> {
        self.daylight_name.as_ref()
    }

    /// Sets the value of DaylightSecond
    pub fn set_daylight_second(&mut self, value: u32) {
        self.daylight_second = Some(value);
    }

    /// Gets the value of DaylightSecond
    pub fn get_daylight_second(&self) -> Option<&u32> {
        self.daylight_second.as_ref()
    }

    /// Sets the value of DaylightYear
    pub fn set_daylight_year(&mut self, value: u32) {
        self.daylight_year = Some(value);
    }

    /// Gets the value of DaylightYear
    pub fn get_daylight_year(&self) -> Option<&u32> {
        self.daylight_year.as_ref()
    }

    /// Sets the value of StandardBias
    pub fn set_standard_bias(&mut self, value: u32) {
        self.standard_bias = Some(value);
    }

    /// Gets the value of StandardBias
    pub fn get_standard_bias(&self) -> Option<&u32> {
        self.standard_bias.as_ref()
    }

    /// Sets the value of StandardDay
    pub fn set_standard_day(&mut self, value: u32) {
        self.standard_day = Some(value);
    }

    /// Gets the value of StandardDay
    pub fn get_standard_day(&self) -> Option<&u32> {
        self.standard_day.as_ref()
    }

    /// Sets the value of StandardDayOfWeek
    pub fn set_standard_day_of_week(&mut self, value: u8) {
        self.standard_day_of_week = Some(value);
    }

    /// Gets the value of StandardDayOfWeek
    pub fn get_standard_day_of_week(&self) -> Option<&u8> {
        self.standard_day_of_week.as_ref()
    }

    /// Sets the value of StandardHour
    pub fn set_standard_hour(&mut self, value: u32) {
        self.standard_hour = Some(value);
    }

    /// Gets the value of StandardHour
    pub fn get_standard_hour(&self) -> Option<&u32> {
        self.standard_hour.as_ref()
    }

    /// Sets the value of StandardMillisecond
    pub fn set_standard_millisecond(&mut self, value: u32) {
        self.standard_millisecond = Some(value);
    }

    /// Gets the value of StandardMillisecond
    pub fn get_standard_millisecond(&self) -> Option<&u32> {
        self.standard_millisecond.as_ref()
    }

    /// Sets the value of StandardMinute
    pub fn set_standard_minute(&mut self, value: u32) {
        self.standard_minute = Some(value);
    }

    /// Gets the value of StandardMinute
    pub fn get_standard_minute(&self) -> Option<&u32> {
        self.standard_minute.as_ref()
    }

    /// Sets the value of StandardMonth
    pub fn set_standard_month(&mut self, value: u32) {
        self.standard_month = Some(value);
    }

    /// Gets the value of StandardMonth
    pub fn get_standard_month(&self) -> Option<&u32> {
        self.standard_month.as_ref()
    }

    /// Sets the value of StandardName
    pub fn set_standard_name(&mut self, value: String) {
        self.standard_name = Some(value);
    }

    /// Gets the value of StandardName
    pub fn get_standard_name(&self) -> Option<&String> {
        self.standard_name.as_ref()
    }

    /// Sets the value of StandardSecond
    pub fn set_standard_second(&mut self, value: u32) {
        self.standard_second = Some(value);
    }

    /// Gets the value of StandardSecond
    pub fn get_standard_second(&self) -> Option<&u32> {
        self.standard_second.as_ref()
    }

    /// Sets the value of StandardYear
    pub fn set_standard_year(&mut self, value: u32) {
        self.standard_year = Some(value);
    }

    /// Gets the value of StandardYear
    pub fn get_standard_year(&self) -> Option<&u32> {
        self.standard_year.as_ref()
    }
}

