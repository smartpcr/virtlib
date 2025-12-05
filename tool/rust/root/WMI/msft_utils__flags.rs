// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftUtils_Flags
//////////////////////////////////////////////

/// MsftUtils_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftUtils_Flags {
    /// MsftUtils_DebugConstructor
    #[serde(rename = "MsftUtils_DebugConstructor")]
    MsftUtilsDebugConstructor = 1,
    /// MsftUtils_DebugDestructor
    #[serde(rename = "MsftUtils_DebugDestructor")]
    MsftUtilsDebugDestructor = 2,
    /// MsftUtils_DebugGeneral
    #[serde(rename = "MsftUtils_DebugGeneral")]
    MsftUtilsDebugGeneral = 3,
}

impl Default for MsftUtils_Flags {
    fn default() -> Self {
        Self::MsftUtilsDebugConstructor
    }
}

