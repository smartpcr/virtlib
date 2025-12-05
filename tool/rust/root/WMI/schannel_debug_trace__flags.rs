// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SchannelDebugTrace_Flags
//////////////////////////////////////////////

/// SchannelDebugTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SchannelDebugTrace_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Trace
    #[serde(rename = "Trace")]
    Trace = 3,
    /// Alloc
    #[serde(rename = "Alloc")]
    Alloc = 4,
    /// Res
    #[serde(rename = "Res")]
    Res = 5,
    /// Func
    #[serde(rename = "Func")]
    Func = 6,
    /// Cred
    #[serde(rename = "Cred")]
    Cred = 7,
    /// Ctxt
    #[serde(rename = "Ctxt")]
    Ctxt = 8,
    /// Mapper
    #[serde(rename = "Mapper")]
    Mapper = 9,
    /// Buffers
    #[serde(rename = "Buffers")]
    Buffers = 10,
}

impl Default for SchannelDebugTrace_Flags {
    fn default() -> Self {
        Self::Error
    }
}

