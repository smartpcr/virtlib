// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WinInit_Flags
//////////////////////////////////////////////

/// WinInit_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WinInit_Flags {
    /// Core
    #[serde(rename = "Core")]
    Core = 0,
    /// Boot
    #[serde(rename = "Boot")]
    Boot = 1,
    /// Setup
    #[serde(rename = "Setup")]
    Setup = 2,
    /// Shutdown
    #[serde(rename = "Shutdown")]
    Shutdown = 3,
    /// Power
    #[serde(rename = "Power")]
    Power = 4,
    /// WMsg
    #[serde(rename = "WMsg")]
    WMsg = 5,
    /// Sas
    #[serde(rename = "Sas")]
    Sas = 6,
}

impl Default for WinInit_Flags {
    fn default() -> Self {
        Self::Core
    }
}

