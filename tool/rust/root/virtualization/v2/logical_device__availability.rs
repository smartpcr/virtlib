// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LogicalDevice_Availability
//////////////////////////////////////////////

/// LogicalDevice_Availability enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LogicalDevice_Availability {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Running_Full_Power
    #[serde(rename = "Running_Full_Power")]
    RunningFullPower = 3,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 4,
    /// In_Test
    #[serde(rename = "In_Test")]
    InTest = 5,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 6,
    /// Power_Off
    #[serde(rename = "Power_Off")]
    PowerOff = 7,
    /// Off_Line
    #[serde(rename = "Off_Line")]
    OffLine = 8,
    /// Off_Duty
    #[serde(rename = "Off_Duty")]
    OffDuty = 9,
    /// Degraded
    #[serde(rename = "Degraded")]
    Degraded = 10,
    /// Not_Installed
    #[serde(rename = "Not_Installed")]
    NotInstalled = 11,
    /// Install_Error
    #[serde(rename = "Install_Error")]
    InstallError = 12,
    /// Power_Save___Unknown
    #[serde(rename = "Power_Save___Unknown")]
    PowerSaveUnknown = 13,
    /// Power_Save___Low_Power_Mode
    #[serde(rename = "Power_Save___Low_Power_Mode")]
    PowerSaveLowPowerMode = 14,
    /// Power_Save___Standby
    #[serde(rename = "Power_Save___Standby")]
    PowerSaveStandby = 15,
    /// Power_Cycle
    #[serde(rename = "Power_Cycle")]
    PowerCycle = 16,
    /// Power_Save___Warning
    #[serde(rename = "Power_Save___Warning")]
    PowerSaveWarning = 17,
    /// Paused
    #[serde(rename = "Paused")]
    Paused = 18,
    /// Not_Ready
    #[serde(rename = "Not_Ready")]
    NotReady = 19,
    /// Not_Configured
    #[serde(rename = "Not_Configured")]
    NotConfigured = 20,
    /// Quiesced
    #[serde(rename = "Quiesced")]
    Quiesced = 21,
}

impl Default for LogicalDevice_Availability {
    fn default() -> Self {
        Self::Other
    }
}

