// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DebugTrace_Flags
//////////////////////////////////////////////

/// DebugTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DebugTrace_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Init
    #[serde(rename = "Init")]
    Init = 3,
    /// Misc
    #[serde(rename = "Misc")]
    Misc = 4,
    /// LogonSess
    #[serde(rename = "LogonSess")]
    LogonSess = 5,
    /// Leak
    #[serde(rename = "Leak")]
    Leak = 6,
    /// Lpc
    #[serde(rename = "Lpc")]
    Lpc = 7,
    /// LpcMore
    #[serde(rename = "LpcMore")]
    LpcMore = 8,
    /// Api
    #[serde(rename = "Api")]
    Api = 9,
    /// ApiMore
    #[serde(rename = "ApiMore")]
    ApiMore = 10,
    /// SKey
    #[serde(rename = "SKey")]
    SKey = 11,
    /// Nego
    #[serde(rename = "Nego")]
    Nego = 12,
    /// Updates
    #[serde(rename = "Updates")]
    Updates = 13,
    /// NtLmV2
    #[serde(rename = "NtLmV2")]
    NtLmV2 = 14,
    /// Cred
    #[serde(rename = "Cred")]
    Cred = 15,
    /// Version
    #[serde(rename = "Version")]
    Version = 16,
    /// Target
    #[serde(rename = "Target")]
    Target = 17,
}

impl Default for DebugTrace_Flags {
    fn default() -> Self {
        Self::Error
    }
}

