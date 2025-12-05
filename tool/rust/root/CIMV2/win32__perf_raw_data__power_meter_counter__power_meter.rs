// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PowerMeterCounter_PowerMeter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PowerMeterCounter_PowerMeter {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Power")]
    pub power: Option<u32>,

/// 
    #[serde(rename = "PowerBudget")]
    pub power_budget: Option<u32>,
}

impl Win32_PerfRawData_PowerMeterCounter_PowerMeter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            power: None,
            power_budget: None,
        }
    }


    /// Sets the value of Power
    pub fn set_power(&mut self, value: u32) {
        self.power = Some(value);
    }

    /// Gets the value of Power
    pub fn get_power(&self) -> Option<&u32> {
        self.power.as_ref()
    }

    /// Sets the value of PowerBudget
    pub fn set_power_budget(&mut self, value: u32) {
        self.power_budget = Some(value);
    }

    /// Gets the value of PowerBudget
    pub fn get_power_budget(&self) -> Option<&u32> {
        self.power_budget.as_ref()
    }
}

