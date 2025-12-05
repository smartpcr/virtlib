// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Processor_CPUStatus
//////////////////////////////////////////////

/// Processor_CPUStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Processor_CPUStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// CPU_Enabled
    #[serde(rename = "CPU_Enabled")]
    CPUEnabled = 1,
    /// CPU_Disabled_by_User
    #[serde(rename = "CPU_Disabled_by_User")]
    CPUDisabledByUser = 2,
    /// CPU_Disabled_By_BIOS__POST_Error_
    #[serde(rename = "CPU_Disabled_By_BIOS__POST_Error_")]
    CPUDisabledByBIOSPOSTError = 3,
    /// CPU_Is_Idle
    #[serde(rename = "CPU_Is_Idle")]
    CPUIsIdle = 4,
    /// Other
    #[serde(rename = "Other")]
    Other = 7,
}

impl Default for Processor_CPUStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

