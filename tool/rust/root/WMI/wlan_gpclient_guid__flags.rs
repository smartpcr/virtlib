// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WlanGPClientGuid_Flags
//////////////////////////////////////////////

/// WlanGPClientGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WlanGPClientGuid_Flags {
    /// Fatal
    #[serde(rename = "Fatal")]
    Fatal = 1,
    /// Error
    #[serde(rename = "Error")]
    Error = 2,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 3,
    /// Information
    #[serde(rename = "Information")]
    Information = 4,
    /// Trace
    #[serde(rename = "Trace")]
    Trace = 5,
    /// Noise
    #[serde(rename = "Noise")]
    Noise = 6,
    /// Perf
    #[serde(rename = "Perf")]
    Perf = 7,
}

impl Default for WlanGPClientGuid_Flags {
    fn default() -> Self {
        Self::Fatal
    }
}

