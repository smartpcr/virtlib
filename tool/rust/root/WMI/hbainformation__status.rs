// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source HBAInformation_Status
//////////////////////////////////////////////

/// HBAInformation_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum HBAInformation_Status {
    /// Working
    #[serde(rename = "Working")]
    Working = 0,
    /// Degraded
    #[serde(rename = "Degraded")]
    Degraded = 1,
    /// Critical
    #[serde(rename = "Critical")]
    Critical = 2,
    /// Failed
    #[serde(rename = "Failed")]
    Failed = 3,
}

impl Default for HBAInformation_Status {
    fn default() -> Self {
        Self::Working
    }
}

