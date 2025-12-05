// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2Data_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2Data_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2Data_Flags {
    /// MsftDiscFormat2Data_DebugConstructor
    #[serde(rename = "MsftDiscFormat2Data_DebugConstructor")]
    MsftDiscFormat2DataDebugConstructor = 1,
    /// MsftDiscFormat2Data_DebugDestructor
    #[serde(rename = "MsftDiscFormat2Data_DebugDestructor")]
    MsftDiscFormat2DataDebugDestructor = 2,
    /// MsftDiscFormat2Data_DebugGeneral
    #[serde(rename = "MsftDiscFormat2Data_DebugGeneral")]
    MsftDiscFormat2DataDebugGeneral = 3,
}

impl Default for MsftDiscFormat2Data_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2DataDebugConstructor
    }
}

