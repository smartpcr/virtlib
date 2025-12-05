// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShareAccessControlEntry_AccessControlType
//////////////////////////////////////////////

/// SmbShareAccessControlEntry_AccessControlType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShareAccessControlEntry_AccessControlType {
    /// _1
    #[serde(rename = "_1")]
    V1 = 0,
    /// _2
    #[serde(rename = "_2")]
    V2 = 1,
}

impl Default for SmbShareAccessControlEntry_AccessControlType {
    fn default() -> Self {
        Self::V1
    }
}

