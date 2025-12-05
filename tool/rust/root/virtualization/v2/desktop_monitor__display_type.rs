// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DesktopMonitor_DisplayType
//////////////////////////////////////////////

/// DesktopMonitor_DisplayType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DesktopMonitor_DisplayType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Multiscan_Color
    #[serde(rename = "Multiscan_Color")]
    MultiscanColor = 2,
    /// Multiscan_Monochrome
    #[serde(rename = "Multiscan_Monochrome")]
    MultiscanMonochrome = 3,
    /// Fixed_Frequency_Color
    #[serde(rename = "Fixed_Frequency_Color")]
    FixedFrequencyColor = 4,
    /// Fixed_Frequency_Monochrome
    #[serde(rename = "Fixed_Frequency_Monochrome")]
    FixedFrequencyMonochrome = 5,
}

impl Default for DesktopMonitor_DisplayType {
    fn default() -> Self {
        Self::Unknown
    }
}

