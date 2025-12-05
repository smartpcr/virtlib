// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FDWNetCtlGuid_Flags
//////////////////////////////////////////////

/// FDWNetCtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FDWNetCtlGuid_Flags {
    /// DUMMY
    #[serde(rename = "DUMMY")]
    DUMMY = 1,
}

impl Default for FDWNetCtlGuid_Flags {
    fn default() -> Self {
        Self::DUMMY
    }
}

