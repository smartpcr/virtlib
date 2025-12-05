// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2Tao_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2Tao_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2Tao_Flags {
    /// MsftDiscFormat2Tao_DebugConstructor
    #[serde(rename = "MsftDiscFormat2Tao_DebugConstructor")]
    MsftDiscFormat2TaoDebugConstructor = 1,
    /// MsftDiscFormat2Tao_DebugDestructor
    #[serde(rename = "MsftDiscFormat2Tao_DebugDestructor")]
    MsftDiscFormat2TaoDebugDestructor = 2,
    /// MsftDiscFormat2Tao_DebugGeneral
    #[serde(rename = "MsftDiscFormat2Tao_DebugGeneral")]
    MsftDiscFormat2TaoDebugGeneral = 3,
}

impl Default for MsftDiscFormat2Tao_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2TaoDebugConstructor
    }
}

