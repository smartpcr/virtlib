// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source API_Flags
//////////////////////////////////////////////

/// API_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum API_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// EAAPI
    #[serde(rename = "EAAPI")]
    EAAPI = 3,
    /// EntryPoint
    #[serde(rename = "EntryPoint")]
    EntryPoint = 4,
    /// ImeDDI
    #[serde(rename = "ImeDDI")]
    ImeDDI = 5,
    /// TipInterface
    #[serde(rename = "TipInterface")]
    TipInterface = 6,
    /// FileOp
    #[serde(rename = "FileOp")]
    FileOp = 7,
    /// Misc
    #[serde(rename = "Misc")]
    Misc = 8,
}

impl Default for API_Flags {
    fn default() -> Self {
        Self::Error
    }
}

