// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MediaAccessDevice_Security
//////////////////////////////////////////////

/// MediaAccessDevice_Security enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MediaAccessDevice_Security {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// None
    #[serde(rename = "None")]
    None = 3,
    /// Read_Only
    #[serde(rename = "Read_Only")]
    ReadOnly = 4,
    /// Locked_Out
    #[serde(rename = "Locked_Out")]
    LockedOut = 5,
    /// Boot_Bypass
    #[serde(rename = "Boot_Bypass")]
    BootBypass = 6,
    /// Boot_Bypass_and_Read_Only
    #[serde(rename = "Boot_Bypass_and_Read_Only")]
    BootBypassAndReadOnly = 7,
}

impl Default for MediaAccessDevice_Security {
    fn default() -> Self {
        Self::Other
    }
}

