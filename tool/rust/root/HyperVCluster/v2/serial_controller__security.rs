// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SerialController_Security
//////////////////////////////////////////////

/// SerialController_Security enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SerialController_Security {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// None
    #[serde(rename = "None")]
    None = 3,
    /// External_Interface_Locked_Out
    #[serde(rename = "External_Interface_Locked_Out")]
    ExternalInterfaceLockedOut = 4,
    /// External_Interface_Enabled
    #[serde(rename = "External_Interface_Enabled")]
    ExternalInterfaceEnabled = 5,
    /// Boot_Bypass
    #[serde(rename = "Boot_Bypass")]
    BootBypass = 6,
}

impl Default for SerialController_Security {
    fn default() -> Self {
        Self::Other
    }
}

