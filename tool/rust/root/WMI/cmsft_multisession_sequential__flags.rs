// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CMsftMultisessionSequential_Flags
//////////////////////////////////////////////

/// CMsftMultisessionSequential_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CMsftMultisessionSequential_Flags {
    /// MsftMultisessionSequential_DebugConstructor
    #[serde(rename = "MsftMultisessionSequential_DebugConstructor")]
    MsftMultisessionSequentialDebugConstructor = 1,
    /// MsftMultisessionSequential_DebugDestructor
    #[serde(rename = "MsftMultisessionSequential_DebugDestructor")]
    MsftMultisessionSequentialDebugDestructor = 2,
    /// MsftMultisessionSequential_DebugGeneral
    #[serde(rename = "MsftMultisessionSequential_DebugGeneral")]
    MsftMultisessionSequentialDebugGeneral = 3,
}

impl Default for CMsftMultisessionSequential_Flags {
    fn default() -> Self {
        Self::MsftMultisessionSequentialDebugConstructor
    }
}

