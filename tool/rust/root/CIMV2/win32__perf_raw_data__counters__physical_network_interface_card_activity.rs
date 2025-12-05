// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_PhysicalNetworkInterfaceCardActivity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_PhysicalNetworkInterfaceCardActivity {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "DevicePowerState")]
    pub device_power_state: Option<u32>,

/// 
    #[serde(rename = "LowPowerTransitionsLifetime")]
    pub low_power_transitions_lifetime: Option<u32>,

/// 
    #[serde(rename = "PercentTimeSuspendedInstantaneous")]
    pub percent_time_suspended_instantaneous: Option<u64>,

/// 
    #[serde(rename = "PercentTimeSuspendedLifetime")]
    pub percent_time_suspended_lifetime: Option<u64>,

/// 
    #[serde(rename = "PercentTimeSuspendedLifetime_Base")]
    pub percent_time_suspended_lifetime__base: Option<u64>,
}

impl Win32_PerfRawData_Counters_PhysicalNetworkInterfaceCardActivity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            device_power_state: None,
            low_power_transitions_lifetime: None,
            percent_time_suspended_instantaneous: None,
            percent_time_suspended_lifetime: None,
            percent_time_suspended_lifetime__base: None,
        }
    }


    /// Sets the value of DevicePowerState
    pub fn set_device_power_state(&mut self, value: u32) {
        self.device_power_state = Some(value);
    }

    /// Gets the value of DevicePowerState
    pub fn get_device_power_state(&self) -> Option<&u32> {
        self.device_power_state.as_ref()
    }

    /// Sets the value of LowPowerTransitionsLifetime
    pub fn set_low_power_transitions_lifetime(&mut self, value: u32) {
        self.low_power_transitions_lifetime = Some(value);
    }

    /// Gets the value of LowPowerTransitionsLifetime
    pub fn get_low_power_transitions_lifetime(&self) -> Option<&u32> {
        self.low_power_transitions_lifetime.as_ref()
    }

    /// Sets the value of PercentTimeSuspendedInstantaneous
    pub fn set_percent_time_suspended_instantaneous(&mut self, value: u64) {
        self.percent_time_suspended_instantaneous = Some(value);
    }

    /// Gets the value of PercentTimeSuspendedInstantaneous
    pub fn get_percent_time_suspended_instantaneous(&self) -> Option<&u64> {
        self.percent_time_suspended_instantaneous.as_ref()
    }

    /// Sets the value of PercentTimeSuspendedLifetime
    pub fn set_percent_time_suspended_lifetime(&mut self, value: u64) {
        self.percent_time_suspended_lifetime = Some(value);
    }

    /// Gets the value of PercentTimeSuspendedLifetime
    pub fn get_percent_time_suspended_lifetime(&self) -> Option<&u64> {
        self.percent_time_suspended_lifetime.as_ref()
    }

    /// Sets the value of PercentTimeSuspendedLifetime_Base
    pub fn set_percent_time_suspended_lifetime__base(&mut self, value: u64) {
        self.percent_time_suspended_lifetime__base = Some(value);
    }

    /// Gets the value of PercentTimeSuspendedLifetime_Base
    pub fn get_percent_time_suspended_lifetime__base(&self) -> Option<&u64> {
        self.percent_time_suspended_lifetime__base.as_ref()
    }
}

