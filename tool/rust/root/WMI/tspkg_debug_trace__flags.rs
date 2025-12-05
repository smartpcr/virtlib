// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSPkgDebugTrace_Flags
//////////////////////////////////////////////

/// TSPkgDebugTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSPkgDebugTrace_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Trace
    #[serde(rename = "Trace")]
    Trace = 3,
    /// Creds
    #[serde(rename = "Creds")]
    Creds = 4,
    /// Context
    #[serde(rename = "Context")]
    Context = 5,
    /// Calls
    #[serde(rename = "Calls")]
    Calls = 6,
    /// Auth
    #[serde(rename = "Auth")]
    Auth = 7,
    /// Session
    #[serde(rename = "Session")]
    Session = 8,
}

impl Default for TSPkgDebugTrace_Flags {
    fn default() -> Self {
        Self::Error
    }
}

