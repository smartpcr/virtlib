// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2StreamRandom_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2StreamRandom_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2StreamRandom_Flags {
    /// MsftDiscFormat2StreamRandom_DebugConstructor
    #[serde(rename = "MsftDiscFormat2StreamRandom_DebugConstructor")]
    MsftDiscFormat2StreamRandomDebugConstructor = 1,
    /// MsftDiscFormat2StreamRandom_DebugDestructor
    #[serde(rename = "MsftDiscFormat2StreamRandom_DebugDestructor")]
    MsftDiscFormat2StreamRandomDebugDestructor = 2,
    /// MsftDiscFormat2StreamRandom_DebugGeneral
    #[serde(rename = "MsftDiscFormat2StreamRandom_DebugGeneral")]
    MsftDiscFormat2StreamRandomDebugGeneral = 3,
}

impl Default for MsftDiscFormat2StreamRandom_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2StreamRandomDebugConstructor
    }
}

