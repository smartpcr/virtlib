// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Processor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Processor {

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MaxClockSpeedInMHz")]
    pub max_clock_speed_in_mhz: Option<u32>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "NumberOfCores")]
    pub number_of_cores: Option<u32>,

/// 
    #[serde(rename = "NumberOfLogicalProcessors")]
    pub number_of_logical_processors: Option<u32>,
}

impl SDDC_Processor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            manufacturer: None,
            max_clock_speed_in_mhz: None,
            model: None,
            number_of_cores: None,
            number_of_logical_processors: None,
        }
    }


    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MaxClockSpeedInMHz
    pub fn set_max_clock_speed_in_mhz(&mut self, value: u32) {
        self.max_clock_speed_in_mhz = Some(value);
    }

    /// Gets the value of MaxClockSpeedInMHz
    pub fn get_max_clock_speed_in_mhz(&self) -> Option<&u32> {
        self.max_clock_speed_in_mhz.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of NumberOfCores
    pub fn set_number_of_cores(&mut self, value: u32) {
        self.number_of_cores = Some(value);
    }

    /// Gets the value of NumberOfCores
    pub fn get_number_of_cores(&self) -> Option<&u32> {
        self.number_of_cores.as_ref()
    }

    /// Sets the value of NumberOfLogicalProcessors
    pub fn set_number_of_logical_processors(&mut self, value: u32) {
        self.number_of_logical_processors = Some(value);
    }

    /// Gets the value of NumberOfLogicalProcessors
    pub fn get_number_of_logical_processors(&self) -> Option<&u32> {
        self.number_of_logical_processors.as_ref()
    }
}

