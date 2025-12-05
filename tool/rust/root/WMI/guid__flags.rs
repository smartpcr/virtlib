// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source GUID_Flags
//////////////////////////////////////////////

/// GUID_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum GUID_Flags {
    /// Critical
    #[serde(rename = "Critical")]
    Critical = 1,
    /// Error
    #[serde(rename = "Error")]
    Error = 2,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 3,
    /// Info
    #[serde(rename = "Info")]
    Info = 4,
    /// Verbose
    #[serde(rename = "Verbose")]
    Verbose = 5,
}

impl Default for GUID_Flags {
    fn default() -> Self {
        Self::Critical
    }
}

