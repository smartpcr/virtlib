// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LogicalDevice_PowerState
//////////////////////////////////////////////

/// LogicalDevice_PowerState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LogicalDevice_PowerState {
    /// Full_Power
    #[serde(rename = "Full_Power")]
    FullPower = 1,
    /// Power_Save___Low_Power_Mode
    #[serde(rename = "Power_Save___Low_Power_Mode")]
    PowerSaveLowPowerMode = 2,
    /// Power_Save___Standby
    #[serde(rename = "Power_Save___Standby")]
    PowerSaveStandby = 3,
    /// Power_Save___Other
    #[serde(rename = "Power_Save___Other")]
    PowerSaveOther = 4,
    /// Power_Cycle
    #[serde(rename = "Power_Cycle")]
    PowerCycle = 5,
    /// Power_Off
    #[serde(rename = "Power_Off")]
    PowerOff = 6,
}

impl Default for LogicalDevice_PowerState {
    fn default() -> Self {
        Self::FullPower
    }
}

