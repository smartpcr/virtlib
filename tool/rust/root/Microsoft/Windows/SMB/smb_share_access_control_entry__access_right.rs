// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShareAccessControlEntry_AccessRight
//////////////////////////////////////////////

/// SmbShareAccessControlEntry_AccessRight enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShareAccessControlEntry_AccessRight {
    /// _3
    #[serde(rename = "_3")]
    V3 = 0,
    /// _4
    #[serde(rename = "_4")]
    V4 = 1,
    /// _5
    #[serde(rename = "_5")]
    V5 = 2,
    /// _6
    #[serde(rename = "_6")]
    V6 = 3,
}

impl Default for SmbShareAccessControlEntry_AccessRight {
    fn default() -> Self {
        Self::V3
    }
}

