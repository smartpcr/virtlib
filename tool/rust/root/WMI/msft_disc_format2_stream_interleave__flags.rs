// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2StreamInterleave_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2StreamInterleave_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2StreamInterleave_Flags {
    /// MsftDiscFormat2StreamInterleave_DebugConstructor
    #[serde(rename = "MsftDiscFormat2StreamInterleave_DebugConstructor")]
    MsftDiscFormat2StreamInterleaveDebugConstructor = 1,
    /// MsftDiscFormat2StreamInterleave_DebugDestructor
    #[serde(rename = "MsftDiscFormat2StreamInterleave_DebugDestructor")]
    MsftDiscFormat2StreamInterleaveDebugDestructor = 2,
    /// MsftDiscFormat2StreamInterleave_DebugGeneral
    #[serde(rename = "MsftDiscFormat2StreamInterleave_DebugGeneral")]
    MsftDiscFormat2StreamInterleaveDebugGeneral = 3,
}

impl Default for MsftDiscFormat2StreamInterleave_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2StreamInterleaveDebugConstructor
    }
}

