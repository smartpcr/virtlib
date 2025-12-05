// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftWriteEngine2_Flags
//////////////////////////////////////////////

/// MsftWriteEngine2_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftWriteEngine2_Flags {
    /// MsftWriteEngine2_DebugConstructor
    #[serde(rename = "MsftWriteEngine2_DebugConstructor")]
    MsftWriteEngine2DebugConstructor = 1,
    /// MsftWriteEngine2_DebugDestructor
    #[serde(rename = "MsftWriteEngine2_DebugDestructor")]
    MsftWriteEngine2DebugDestructor = 2,
    /// MsftWriteEngine2_DebugGeneral
    #[serde(rename = "MsftWriteEngine2_DebugGeneral")]
    MsftWriteEngine2DebugGeneral = 3,
}

impl Default for MsftWriteEngine2_Flags {
    fn default() -> Self {
        Self::MsftWriteEngine2DebugConstructor
    }
}

