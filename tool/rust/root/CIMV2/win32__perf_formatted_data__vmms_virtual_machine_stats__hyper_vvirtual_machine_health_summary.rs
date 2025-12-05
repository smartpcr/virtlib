// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_VmmsVirtualMachineStats_HyperVVirtualMachineHealthSummary struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_VmmsVirtualMachineStats_HyperVVirtualMachineHealthSummary {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "HealthCritical")]
    pub health_critical: Option<u32>,

/// 
    #[serde(rename = "HealthOk")]
    pub health_ok: Option<u32>,
}

impl Win32_PerfFormattedData_VmmsVirtualMachineStats_HyperVVirtualMachineHealthSummary {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            health_critical: None,
            health_ok: None,
        }
    }


    /// Sets the value of HealthCritical
    pub fn set_health_critical(&mut self, value: u32) {
        self.health_critical = Some(value);
    }

    /// Gets the value of HealthCritical
    pub fn get_health_critical(&self) -> Option<&u32> {
        self.health_critical.as_ref()
    }

    /// Sets the value of HealthOk
    pub fn set_health_ok(&mut self, value: u32) {
        self.health_ok = Some(value);
    }

    /// Gets the value of HealthOk
    pub fn get_health_ok(&self) -> Option<&u32> {
        self.health_ok.as_ref()
    }
}

