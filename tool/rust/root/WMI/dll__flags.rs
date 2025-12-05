// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Dll_Flags
//////////////////////////////////////////////

/// Dll_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Dll_Flags {
    /// IMAPI2_DebugGeneral
    #[serde(rename = "IMAPI2_DebugGeneral")]
    IMAPI2DebugGeneral = 1,
}

impl Default for Dll_Flags {
    fn default() -> Self {
        Self::IMAPI2DebugGeneral
    }
}

