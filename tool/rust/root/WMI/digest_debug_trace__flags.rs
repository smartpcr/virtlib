// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DigestDebugTrace_Flags
//////////////////////////////////////////////

/// DigestDebugTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DigestDebugTrace_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Trace
    #[serde(rename = "Trace")]
    Trace = 3,
    /// TraceASC
    #[serde(rename = "TraceASC")]
    TraceASC = 4,
    /// TraceICS
    #[serde(rename = "TraceICS")]
    TraceICS = 5,
    /// TraceLSA
    #[serde(rename = "TraceLSA")]
    TraceLSA = 6,
    /// TraceUser
    #[serde(rename = "TraceUser")]
    TraceUser = 7,
    /// TraceFunc
    #[serde(rename = "TraceFunc")]
    TraceFunc = 8,
    /// TraceMem
    #[serde(rename = "TraceMem")]
    TraceMem = 9,
    /// TraceStuff
    #[serde(rename = "TraceStuff")]
    TraceStuff = 10,
}

impl Default for DigestDebugTrace_Flags {
    fn default() -> Self {
        Self::Error
    }
}

