// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_BalancerStats_HyperVDynamicMemoryBalancer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_BalancerStats_HyperVDynamicMemoryBalancer {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AvailableMemory")]
    pub available_memory: Option<u32>,

/// 
    #[serde(rename = "AvailableMemoryForBalancing")]
    pub available_memory_for_balancing: Option<u32>,

/// 
    #[serde(rename = "AveragePressure")]
    pub average_pressure: Option<u32>,

/// 
    #[serde(rename = "SystemCurrentPressure")]
    pub system_current_pressure: Option<u32>,
}

impl Win32_PerfFormattedData_BalancerStats_HyperVDynamicMemoryBalancer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            available_memory: None,
            available_memory_for_balancing: None,
            average_pressure: None,
            system_current_pressure: None,
        }
    }


    /// Sets the value of AvailableMemory
    pub fn set_available_memory(&mut self, value: u32) {
        self.available_memory = Some(value);
    }

    /// Gets the value of AvailableMemory
    pub fn get_available_memory(&self) -> Option<&u32> {
        self.available_memory.as_ref()
    }

    /// Sets the value of AvailableMemoryForBalancing
    pub fn set_available_memory_for_balancing(&mut self, value: u32) {
        self.available_memory_for_balancing = Some(value);
    }

    /// Gets the value of AvailableMemoryForBalancing
    pub fn get_available_memory_for_balancing(&self) -> Option<&u32> {
        self.available_memory_for_balancing.as_ref()
    }

    /// Sets the value of AveragePressure
    pub fn set_average_pressure(&mut self, value: u32) {
        self.average_pressure = Some(value);
    }

    /// Gets the value of AveragePressure
    pub fn get_average_pressure(&self) -> Option<&u32> {
        self.average_pressure.as_ref()
    }

    /// Sets the value of SystemCurrentPressure
    pub fn set_system_current_pressure(&mut self, value: u32) {
        self.system_current_pressure = Some(value);
    }

    /// Gets the value of SystemCurrentPressure
    pub fn get_system_current_pressure(&self) -> Option<&u32> {
        self.system_current_pressure.as_ref()
    }
}

