// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SettingsDefineCapabilities_ValueRole
//////////////////////////////////////////////

/// SettingsDefineCapabilities_ValueRole enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SettingsDefineCapabilities_ValueRole {
    /// Default
    #[serde(rename = "Default")]
    Default = 0,
    /// Optimal
    #[serde(rename = "Optimal")]
    Optimal = 1,
    /// Mean
    #[serde(rename = "Mean")]
    Mean = 2,
    /// Supported
    #[serde(rename = "Supported")]
    Supported = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
}

impl Default for SettingsDefineCapabilities_ValueRole {
    fn default() -> Self {
        Self::Default
    }
}

