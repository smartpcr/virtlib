// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VideoHead_CurrentScanMode
//////////////////////////////////////////////

/// VideoHead_CurrentScanMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VideoHead_CurrentScanMode {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Not_Supported
    #[serde(rename = "Not_Supported")]
    NotSupported = 2,
    /// Non_Interlaced_Operation
    #[serde(rename = "Non_Interlaced_Operation")]
    NonInterlacedOperation = 3,
    /// Interlaced_Operation
    #[serde(rename = "Interlaced_Operation")]
    InterlacedOperation = 4,
}

impl Default for VideoHead_CurrentScanMode {
    fn default() -> Self {
        Self::Unknown
    }
}

