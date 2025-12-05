// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LogicalDevice_StatusInfo
//////////////////////////////////////////////

/// LogicalDevice_StatusInfo enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LogicalDevice_StatusInfo {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 3,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 4,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 5,
}

impl Default for LogicalDevice_StatusInfo {
    fn default() -> Self {
        Self::Other
    }
}

