// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Memory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Memory {

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "SizeInBytes")]
    pub size_in_bytes: Option<u64>,

/// 
    #[serde(rename = "SpeedInMHz")]
    pub speed_in_mhz: Option<u32>,
}

impl SDDC_Memory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            manufacturer: None,
            model: None,
            serial_number: None,
            size_in_bytes: None,
            speed_in_mhz: None,
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

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of SizeInBytes
    pub fn set_size_in_bytes(&mut self, value: u64) {
        self.size_in_bytes = Some(value);
    }

    /// Gets the value of SizeInBytes
    pub fn get_size_in_bytes(&self) -> Option<&u64> {
        self.size_in_bytes.as_ref()
    }

    /// Sets the value of SpeedInMHz
    pub fn set_speed_in_mhz(&mut self, value: u32) {
        self.speed_in_mhz = Some(value);
    }

    /// Gets the value of SpeedInMHz
    pub fn get_speed_in_mhz(&self) -> Option<&u32> {
        self.speed_in_mhz.as_ref()
    }
}

