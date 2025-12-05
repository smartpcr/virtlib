// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscRecorder2_Flags
//////////////////////////////////////////////

/// MsftDiscRecorder2_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscRecorder2_Flags {
    /// MsftDiscRecorder2_DebugConstructor
    #[serde(rename = "MsftDiscRecorder2_DebugConstructor")]
    MsftDiscRecorder2DebugConstructor = 1,
    /// MsftDiscRecorder2_DebugDestructor
    #[serde(rename = "MsftDiscRecorder2_DebugDestructor")]
    MsftDiscRecorder2DebugDestructor = 2,
    /// MsftDiscRecorder2_DebugGeneral
    #[serde(rename = "MsftDiscRecorder2_DebugGeneral")]
    MsftDiscRecorder2DebugGeneral = 3,
}

impl Default for MsftDiscRecorder2_Flags {
    fn default() -> Self {
        Self::MsftDiscRecorder2DebugConstructor
    }
}

