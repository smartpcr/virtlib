// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NetworkProfileManagerTraceGUID_Flags
//////////////////////////////////////////////

/// NetworkProfileManagerTraceGUID_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NetworkProfileManagerTraceGUID_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Informational
    #[serde(rename = "Informational")]
    Informational = 3,
    /// Verbose
    #[serde(rename = "Verbose")]
    Verbose = 4,
}

impl Default for NetworkProfileManagerTraceGUID_Flags {
    fn default() -> Self {
        Self::Error
    }
}

