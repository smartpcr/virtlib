// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Battery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Battery {
    #[serde(flatten)]
    pub base: CIM_Battery,

/// 
    #[serde(rename = "BatteryRechargeTime")]
    pub battery_recharge_time: Option<u32>,

/// 
    #[serde(rename = "ExpectedBatteryLife")]
    pub expected_battery_life: Option<u32>,
}

impl Win32_Battery {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Battery::new(),
            battery_recharge_time: None,
            expected_battery_life: None,
        }
    }


    /// Sets the value of BatteryRechargeTime
    pub fn set_battery_recharge_time(&mut self, value: u32) {
        self.battery_recharge_time = Some(value);
    }

    /// Gets the value of BatteryRechargeTime
    pub fn get_battery_recharge_time(&self) -> Option<&u32> {
        self.battery_recharge_time.as_ref()
    }

    /// Sets the value of ExpectedBatteryLife
    pub fn set_expected_battery_life(&mut self, value: u32) {
        self.expected_battery_life = Some(value);
    }

    /// Gets the value of ExpectedBatteryLife
    pub fn get_expected_battery_life(&self) -> Option<&u32> {
        self.expected_battery_life.as_ref()
    }
}

