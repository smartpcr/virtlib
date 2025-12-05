// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_TM struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_TM {

/// 
    #[serde(rename = "tm_hour")]
    pub tm_hour: Option<u32>,

/// 
    #[serde(rename = "tm_isdst")]
    pub tm_isdst: Option<u32>,

/// 
    #[serde(rename = "tm_mday")]
    pub tm_mday: Option<u32>,

/// 
    #[serde(rename = "tm_min")]
    pub tm_min: Option<u32>,

/// 
    #[serde(rename = "tm_mon")]
    pub tm_mon: Option<u32>,

/// 
    #[serde(rename = "tm_sec")]
    pub tm_sec: Option<u32>,

/// 
    #[serde(rename = "tm_wday")]
    pub tm_wday: Option<u32>,

/// 
    #[serde(rename = "tm_yday")]
    pub tm_yday: Option<u32>,

/// 
    #[serde(rename = "tm_year")]
    pub tm_year: Option<u32>,
}

impl MSFC_TM {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            tm_hour: None,
            tm_isdst: None,
            tm_mday: None,
            tm_min: None,
            tm_mon: None,
            tm_sec: None,
            tm_wday: None,
            tm_yday: None,
            tm_year: None,
        }
    }


    /// Sets the value of tm_hour
    pub fn set_tm_hour(&mut self, value: u32) {
        self.tm_hour = Some(value);
    }

    /// Gets the value of tm_hour
    pub fn get_tm_hour(&self) -> Option<&u32> {
        self.tm_hour.as_ref()
    }

    /// Sets the value of tm_isdst
    pub fn set_tm_isdst(&mut self, value: u32) {
        self.tm_isdst = Some(value);
    }

    /// Gets the value of tm_isdst
    pub fn get_tm_isdst(&self) -> Option<&u32> {
        self.tm_isdst.as_ref()
    }

    /// Sets the value of tm_mday
    pub fn set_tm_mday(&mut self, value: u32) {
        self.tm_mday = Some(value);
    }

    /// Gets the value of tm_mday
    pub fn get_tm_mday(&self) -> Option<&u32> {
        self.tm_mday.as_ref()
    }

    /// Sets the value of tm_min
    pub fn set_tm_min(&mut self, value: u32) {
        self.tm_min = Some(value);
    }

    /// Gets the value of tm_min
    pub fn get_tm_min(&self) -> Option<&u32> {
        self.tm_min.as_ref()
    }

    /// Sets the value of tm_mon
    pub fn set_tm_mon(&mut self, value: u32) {
        self.tm_mon = Some(value);
    }

    /// Gets the value of tm_mon
    pub fn get_tm_mon(&self) -> Option<&u32> {
        self.tm_mon.as_ref()
    }

    /// Sets the value of tm_sec
    pub fn set_tm_sec(&mut self, value: u32) {
        self.tm_sec = Some(value);
    }

    /// Gets the value of tm_sec
    pub fn get_tm_sec(&self) -> Option<&u32> {
        self.tm_sec.as_ref()
    }

    /// Sets the value of tm_wday
    pub fn set_tm_wday(&mut self, value: u32) {
        self.tm_wday = Some(value);
    }

    /// Gets the value of tm_wday
    pub fn get_tm_wday(&self) -> Option<&u32> {
        self.tm_wday.as_ref()
    }

    /// Sets the value of tm_yday
    pub fn set_tm_yday(&mut self, value: u32) {
        self.tm_yday = Some(value);
    }

    /// Gets the value of tm_yday
    pub fn get_tm_yday(&self) -> Option<&u32> {
        self.tm_yday.as_ref()
    }

    /// Sets the value of tm_year
    pub fn set_tm_year(&mut self, value: u32) {
        self.tm_year = Some(value);
    }

    /// Gets the value of tm_year
    pub fn get_tm_year(&self) -> Option<&u32> {
        self.tm_year.as_ref()
    }
}

