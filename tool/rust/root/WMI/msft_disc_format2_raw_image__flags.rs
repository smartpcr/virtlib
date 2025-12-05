// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2RawImage_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2RawImage_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2RawImage_Flags {
    /// MsftDiscFormat2RawImage_DebugConstructor
    #[serde(rename = "MsftDiscFormat2RawImage_DebugConstructor")]
    MsftDiscFormat2RawImageDebugConstructor = 1,
    /// MsftDiscFormat2RawImage_DebugDestructor
    #[serde(rename = "MsftDiscFormat2RawImage_DebugDestructor")]
    MsftDiscFormat2RawImageDebugDestructor = 2,
    /// MsftDiscFormat2RawImage_DebugAddRef
    #[serde(rename = "MsftDiscFormat2RawImage_DebugAddRef")]
    MsftDiscFormat2RawImageDebugAddRef = 3,
    /// MsftDiscFormat2RawImage_DebugRelease
    #[serde(rename = "MsftDiscFormat2RawImage_DebugRelease")]
    MsftDiscFormat2RawImageDebugRelease = 4,
    /// MsftDiscFormat2RawImage_DebugGeneral
    #[serde(rename = "MsftDiscFormat2RawImage_DebugGeneral")]
    MsftDiscFormat2RawImageDebugGeneral = 5,
    /// MsftDiscFormat2RawImage_DebugX
    #[serde(rename = "MsftDiscFormat2RawImage_DebugX")]
    MsftDiscFormat2RawImageDebugX = 6,
}

impl Default for MsftDiscFormat2RawImage_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2RawImageDebugConstructor
    }
}

