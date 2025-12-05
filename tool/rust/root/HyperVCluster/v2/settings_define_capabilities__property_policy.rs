// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SettingsDefineCapabilities_PropertyPolicy
//////////////////////////////////////////////

/// SettingsDefineCapabilities_PropertyPolicy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SettingsDefineCapabilities_PropertyPolicy {
    /// Independent
    #[serde(rename = "Independent")]
    Independent = 0,
    /// Correlated
    #[serde(rename = "Correlated")]
    Correlated = 1,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 2,
}

impl Default for SettingsDefineCapabilities_PropertyPolicy {
    fn default() -> Self {
        Self::Independent
    }
}

