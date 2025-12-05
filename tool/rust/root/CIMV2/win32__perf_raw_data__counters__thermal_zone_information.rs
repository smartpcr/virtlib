// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_ThermalZoneInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_ThermalZoneInformation {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "HighPrecisionTemperature")]
    pub high_precision_temperature: Option<u32>,

/// 
    #[serde(rename = "PercentPassiveLimit")]
    pub percent_passive_limit: Option<u32>,

/// 
    #[serde(rename = "Temperature")]
    pub temperature: Option<u32>,

/// 
    #[serde(rename = "ThrottleReasons")]
    pub throttle_reasons: Option<u32>,
}

impl Win32_PerfRawData_Counters_ThermalZoneInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            high_precision_temperature: None,
            percent_passive_limit: None,
            temperature: None,
            throttle_reasons: None,
        }
    }


    /// Sets the value of HighPrecisionTemperature
    pub fn set_high_precision_temperature(&mut self, value: u32) {
        self.high_precision_temperature = Some(value);
    }

    /// Gets the value of HighPrecisionTemperature
    pub fn get_high_precision_temperature(&self) -> Option<&u32> {
        self.high_precision_temperature.as_ref()
    }

    /// Sets the value of PercentPassiveLimit
    pub fn set_percent_passive_limit(&mut self, value: u32) {
        self.percent_passive_limit = Some(value);
    }

    /// Gets the value of PercentPassiveLimit
    pub fn get_percent_passive_limit(&self) -> Option<&u32> {
        self.percent_passive_limit.as_ref()
    }

    /// Sets the value of Temperature
    pub fn set_temperature(&mut self, value: u32) {
        self.temperature = Some(value);
    }

    /// Gets the value of Temperature
    pub fn get_temperature(&self) -> Option<&u32> {
        self.temperature.as_ref()
    }

    /// Sets the value of ThrottleReasons
    pub fn set_throttle_reasons(&mut self, value: u32) {
        self.throttle_reasons = Some(value);
    }

    /// Gets the value of ThrottleReasons
    pub fn get_throttle_reasons(&self) -> Option<&u32> {
        self.throttle_reasons.as_ref()
    }
}

