// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PowerMeterCounter_EnergyMeter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PowerMeterCounter_EnergyMeter {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Energy")]
    pub energy: Option<u64>,

/// 
    #[serde(rename = "Power")]
    pub power: Option<u64>,

/// 
    #[serde(rename = "Power_Base")]
    pub power__base: Option<u32>,

/// 
    #[serde(rename = "Time")]
    pub time: Option<u64>,
}

impl Win32_PerfRawData_PowerMeterCounter_EnergyMeter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            energy: None,
            power: None,
            power__base: None,
            time: None,
        }
    }


    /// Sets the value of Energy
    pub fn set_energy(&mut self, value: u64) {
        self.energy = Some(value);
    }

    /// Gets the value of Energy
    pub fn get_energy(&self) -> Option<&u64> {
        self.energy.as_ref()
    }

    /// Sets the value of Power
    pub fn set_power(&mut self, value: u64) {
        self.power = Some(value);
    }

    /// Gets the value of Power
    pub fn get_power(&self) -> Option<&u64> {
        self.power.as_ref()
    }

    /// Sets the value of Power_Base
    pub fn set_power__base(&mut self, value: u32) {
        self.power__base = Some(value);
    }

    /// Gets the value of Power_Base
    pub fn get_power__base(&self) -> Option<&u32> {
        self.power__base.as_ref()
    }

    /// Sets the value of Time
    pub fn set_time(&mut self, value: u64) {
        self.time = Some(value);
    }

    /// Gets the value of Time
    pub fn get_time(&self) -> Option<&u64> {
        self.time.as_ref()
    }
}

