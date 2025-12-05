// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WMPNsCfgGuid_Flags
//////////////////////////////////////////////

/// WMPNsCfgGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WMPNsCfgGuid_Flags {
    /// EntryExit
    #[serde(rename = "EntryExit")]
    EntryExit = 1,
    /// Ux
    #[serde(rename = "Ux")]
    Ux = 2,
}

impl Default for WMPNsCfgGuid_Flags {
    fn default() -> Self {
        Self::EntryExit
    }
}

