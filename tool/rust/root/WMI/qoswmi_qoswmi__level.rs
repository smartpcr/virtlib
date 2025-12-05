// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source qoswmiQOSWMI_Level
//////////////////////////////////////////////

/// qoswmiQOSWMI_Level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum qoswmiQOSWMI_Level {
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
    /// Verbose
    #[serde(rename = "Verbose")]
    Verbose = 5,
}

impl Default for qoswmiQOSWMI_Level {
    fn default() -> Self {
        Self::Fatal
    }
}

