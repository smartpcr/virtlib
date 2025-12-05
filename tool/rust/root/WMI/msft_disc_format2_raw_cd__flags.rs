// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2RawCD_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2RawCD_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2RawCD_Flags {
    /// MsftDiscFormat2Raw_DebugConstructor
    #[serde(rename = "MsftDiscFormat2Raw_DebugConstructor")]
    MsftDiscFormat2RawDebugConstructor = 1,
    /// MsftDiscFormat2Raw_DebugDestructor
    #[serde(rename = "MsftDiscFormat2Raw_DebugDestructor")]
    MsftDiscFormat2RawDebugDestructor = 2,
    /// MsftDiscFormat2Raw_DebugGeneral
    #[serde(rename = "MsftDiscFormat2Raw_DebugGeneral")]
    MsftDiscFormat2RawDebugGeneral = 3,
}

impl Default for MsftDiscFormat2RawCD_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2RawDebugConstructor
    }
}

