// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WMPNsSciGuid_Flags
//////////////////////////////////////////////

/// WMPNsSciGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WMPNsSciGuid_Flags {
    /// EntryExit
    #[serde(rename = "EntryExit")]
    EntryExit = 1,
    /// API
    #[serde(rename = "API")]
    API = 2,
}

impl Default for WMPNsSciGuid_Flags {
    fn default() -> Self {
        Self::EntryExit
    }
}

