// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MsftDiscFormat2MsfAddress_Flags
//////////////////////////////////////////////

/// MsftDiscFormat2MsfAddress_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MsftDiscFormat2MsfAddress_Flags {
    /// MsftDiscFormat2MsfAddress_DebugConstructor
    #[serde(rename = "MsftDiscFormat2MsfAddress_DebugConstructor")]
    MsftDiscFormat2MsfAddressDebugConstructor = 1,
    /// MsftDiscFormat2MsfAddress_DebugDestructor
    #[serde(rename = "MsftDiscFormat2MsfAddress_DebugDestructor")]
    MsftDiscFormat2MsfAddressDebugDestructor = 2,
    /// MsftDiscFormat2MsfAddress_DebugGeneral
    #[serde(rename = "MsftDiscFormat2MsfAddress_DebugGeneral")]
    MsftDiscFormat2MsfAddressDebugGeneral = 3,
}

impl Default for MsftDiscFormat2MsfAddress_Flags {
    fn default() -> Self {
        Self::MsftDiscFormat2MsfAddressDebugConstructor
    }
}

