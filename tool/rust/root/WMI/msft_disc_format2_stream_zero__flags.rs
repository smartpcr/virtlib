// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2StreamZero_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2StreamZero_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2StreamZero_Flags {
    /// MsftDiscFormat2StreamZero_DebugConstructor
    #[serde(rename = "MsftDiscFormat2StreamZero_DebugConstructor")]
    MsftDiscFormat2StreamZeroDebugConstructor = 1,
    /// MsftDiscFormat2StreamZero_DebugDestructor
    #[serde(rename = "MsftDiscFormat2StreamZero_DebugDestructor")]
    MsftDiscFormat2StreamZeroDebugDestructor = 2,
    /// MsftDiscFormat2StreamZero_DebugGeneral
    #[serde(rename = "MsftDiscFormat2StreamZero_DebugGeneral")]
    MsftDiscFormat2StreamZeroDebugGeneral = 3,
}

impl Default for MsftDiscFormat2StreamZero_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2StreamZeroDebugConstructor
    }
}

