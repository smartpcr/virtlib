// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuidFDWSD_Flags
//////////////////////////////////////////////

/// CtlGuidFDWSD_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuidFDWSD_Flags {
    /// DUMMY
    #[serde(rename = "DUMMY")]
    DUMMY = 1,
}

impl Default for CtlGuidFDWSD_Flags {
    fn default() -> Self {
        Self::DUMMY
    }
}

