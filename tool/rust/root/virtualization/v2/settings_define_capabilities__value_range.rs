// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SettingsDefineCapabilities_ValueRange
//////////////////////////////////////////////

/// SettingsDefineCapabilities_ValueRange enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SettingsDefineCapabilities_ValueRange {
    /// Point
    #[serde(rename = "Point")]
    Point = 0,
    /// Minimums
    #[serde(rename = "Minimums")]
    Minimums = 1,
    /// Maximums
    #[serde(rename = "Maximums")]
    Maximums = 2,
    /// Increments
    #[serde(rename = "Increments")]
    Increments = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
}

impl Default for SettingsDefineCapabilities_ValueRange {
    fn default() -> Self {
        Self::Point
    }
}

