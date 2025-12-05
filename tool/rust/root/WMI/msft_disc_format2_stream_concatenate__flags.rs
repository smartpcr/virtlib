// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2StreamConcatenate_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2StreamConcatenate_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2StreamConcatenate_Flags {
    /// MsftDiscFormat2StreamConcatenate_DebugConstructor
    #[serde(rename = "MsftDiscFormat2StreamConcatenate_DebugConstructor")]
    MsftDiscFormat2StreamConcatenateDebugConstructor = 1,
    /// MsftDiscFormat2StreamConcatenate_DebugDestructor
    #[serde(rename = "MsftDiscFormat2StreamConcatenate_DebugDestructor")]
    MsftDiscFormat2StreamConcatenateDebugDestructor = 2,
    /// MsftDiscFormat2StreamConcatenate_DebugGeneral
    #[serde(rename = "MsftDiscFormat2StreamConcatenate_DebugGeneral")]
    MsftDiscFormat2StreamConcatenateDebugGeneral = 3,
}

impl Default for MsftDiscFormat2StreamConcatenate_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2StreamConcatenateDebugConstructor
    }
}

