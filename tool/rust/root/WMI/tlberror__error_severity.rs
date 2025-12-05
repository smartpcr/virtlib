// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TLBError_ErrorSeverity
//////////////////////////////////////////////

/// TLBError_ErrorSeverity enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TLBError_ErrorSeverity {
    /// Recoverable
    #[serde(rename = "Recoverable")]
    Recoverable = 0,
    /// Fatal
    #[serde(rename = "Fatal")]
    Fatal = 1,
    /// Correctable
    #[serde(rename = "Correctable")]
    Correctable = 2,
}

impl Default for TLBError_ErrorSeverity {
    fn default() -> Self {
        Self::Recoverable
    }
}

