// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ComputerSystem_ResetCapability
//////////////////////////////////////////////

/// ComputerSystem_ResetCapability enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ComputerSystem_ResetCapability {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 3,
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 4,
    /// Not_Implemented
    #[serde(rename = "Not_Implemented")]
    NotImplemented = 5,
}

impl Default for ComputerSystem_ResetCapability {
    fn default() -> Self {
        Self::Other
    }
}

