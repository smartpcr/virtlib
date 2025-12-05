// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EthernetSwitchPortOffloadSettingData_IOVInterruptModeration
//////////////////////////////////////////////

/// EthernetSwitchPortOffloadSettingData_IOVInterruptModeration enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EthernetSwitchPortOffloadSettingData_IOVInterruptModeration {
    /// Default
    #[serde(rename = "Default")]
    Default = 0,
    /// Adaptive
    #[serde(rename = "Adaptive")]
    Adaptive = 1,
    /// Off
    #[serde(rename = "Off")]
    Off = 2,
    /// Low
    #[serde(rename = "Low")]
    Low = 100,
    /// Medium
    #[serde(rename = "Medium")]
    Medium = 200,
    /// High
    #[serde(rename = "High")]
    High = 300,
}

impl Default for EthernetSwitchPortOffloadSettingData_IOVInterruptModeration {
    fn default() -> Self {
        Self::Default
    }
}

