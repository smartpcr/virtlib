// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PortableBattery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PortableBattery {
    #[serde(flatten)]
    pub base: CIM_Battery,

/// 
    #[serde(rename = "CapacityMultiplier")]
    pub capacity_multiplier: Option<u16>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "ManufactureDate")]
    pub manufacture_date: Option<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MaxBatteryError")]
    pub max_battery_error: Option<u16>,
}

impl Win32_PortableBattery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Battery::new(),
            capacity_multiplier: None,
            location: None,
            manufacture_date: None,
            manufacturer: None,
            max_battery_error: None,
        }
    }


    /// Sets the value of CapacityMultiplier
    pub fn set_capacity_multiplier(&mut self, value: u16) {
        self.capacity_multiplier = Some(value);
    }

    /// Gets the value of CapacityMultiplier
    pub fn get_capacity_multiplier(&self) -> Option<&u16> {
        self.capacity_multiplier.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of ManufactureDate
    pub fn set_manufacture_date(&mut self, value: String) {
        self.manufacture_date = Some(value);
    }

    /// Gets the value of ManufactureDate
    pub fn get_manufacture_date(&self) -> Option<&String> {
        self.manufacture_date.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MaxBatteryError
    pub fn set_max_battery_error(&mut self, value: u16) {
        self.max_battery_error = Some(value);
    }

    /// Gets the value of MaxBatteryError
    pub fn get_max_battery_error(&self) -> Option<&u16> {
        self.max_battery_error.as_ref()
    }
}

