// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2Erase_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2Erase_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2Erase_Flags {
    /// MsftDiscFormat2Erase_DebugConstructor
    #[serde(rename = "MsftDiscFormat2Erase_DebugConstructor")]
    MsftDiscFormat2EraseDebugConstructor = 1,
    /// MsftDiscFormat2Erase_DebugDestructor
    #[serde(rename = "MsftDiscFormat2Erase_DebugDestructor")]
    MsftDiscFormat2EraseDebugDestructor = 2,
    /// MsftDiscFormat2Erase_DebugGeneral
    #[serde(rename = "MsftDiscFormat2Erase_DebugGeneral")]
    MsftDiscFormat2EraseDebugGeneral = 3,
}

impl Default for MsftDiscFormat2Erase_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2EraseDebugConstructor
    }
}

