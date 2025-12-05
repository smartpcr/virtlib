// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Memory_ErrorAccess
//////////////////////////////////////////////

/// Memory_ErrorAccess enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Memory_ErrorAccess {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// Read
    #[serde(rename = "Read")]
    Read = 3,
    /// Write
    #[serde(rename = "Write")]
    Write = 4,
    /// Partial_Write
    #[serde(rename = "Partial_Write")]
    PartialWrite = 5,
}

impl Default for Memory_ErrorAccess {
    fn default() -> Self {
        Self::Other
    }
}

