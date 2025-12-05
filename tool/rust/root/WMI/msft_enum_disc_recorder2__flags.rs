// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftEnumDiscRecorder2_Flags
//////////////////////////////////////////////

/// MsftEnumDiscRecorder2_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftEnumDiscRecorder2_Flags {
    /// MsftEnumDiscRecorder2_DebugConstructor
    #[serde(rename = "MsftEnumDiscRecorder2_DebugConstructor")]
    MsftEnumDiscRecorder2DebugConstructor = 1,
    /// MsftEnumDiscRecorder2_DebugDestructor
    #[serde(rename = "MsftEnumDiscRecorder2_DebugDestructor")]
    MsftEnumDiscRecorder2DebugDestructor = 2,
    /// MsftEnumDiscRecorder2_DebugGeneral
    #[serde(rename = "MsftEnumDiscRecorder2_DebugGeneral")]
    MsftEnumDiscRecorder2DebugGeneral = 3,
}

impl Default for MsftEnumDiscRecorder2_Flags {
    fn default() -> Self {
        Self::MsftEnumDiscRecorder2DebugConstructor
    }
}

