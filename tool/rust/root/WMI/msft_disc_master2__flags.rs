// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscMaster2_Flags
//////////////////////////////////////////////

/// MsftDiscMaster2_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscMaster2_Flags {
    /// MsftDiscMaster2_DebugConstructor
    #[serde(rename = "MsftDiscMaster2_DebugConstructor")]
    MsftDiscMaster2DebugConstructor = 1,
    /// MsftDiscMaster2_DebugDestructor
    #[serde(rename = "MsftDiscMaster2_DebugDestructor")]
    MsftDiscMaster2DebugDestructor = 2,
    /// MsftDiscMaster2_DebugGeneral
    #[serde(rename = "MsftDiscMaster2_DebugGeneral")]
    MsftDiscMaster2DebugGeneral = 3,
}

impl Default for MsftDiscMaster2_Flags {
    fn default() -> Self {
        Self::MsftDiscMaster2DebugConstructor
    }
}

