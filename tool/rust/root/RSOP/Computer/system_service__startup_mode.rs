// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SystemService_StartupMode
//////////////////////////////////////////////

/// SystemService_StartupMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SystemService_StartupMode {
    /// Automatic
    #[serde(rename = "Automatic")]
    Automatic = 2,
    /// Manual
    #[serde(rename = "Manual")]
    Manual = 3,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 4,
}

impl Default for SystemService_StartupMode {
    fn default() -> Self {
        Self::Automatic
    }
}

