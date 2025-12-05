// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ComputerSystem_PowerManagementCapabilities
//////////////////////////////////////////////

/// ComputerSystem_PowerManagementCapabilities enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ComputerSystem_PowerManagementCapabilities {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Supported
    #[serde(rename = "Not_Supported")]
    NotSupported = 1,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 2,
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 3,
    /// Power_Saving_Modes_Entered_Automatically
    #[serde(rename = "Power_Saving_Modes_Entered_Automatically")]
    PowerSavingModesEnteredAutomatically = 4,
    /// Power_State_Settable
    #[serde(rename = "Power_State_Settable")]
    PowerStateSettable = 5,
    /// Power_Cycling_Supported
    #[serde(rename = "Power_Cycling_Supported")]
    PowerCyclingSupported = 6,
    /// Timed_Power_On_Supported
    #[serde(rename = "Timed_Power_On_Supported")]
    TimedPowerOnSupported = 7,
}

impl Default for ComputerSystem_PowerManagementCapabilities {
    fn default() -> Self {
        Self::Unknown
    }
}

