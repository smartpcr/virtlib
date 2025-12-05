// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileShareAccessControlEntry_AccessControlType
//////////////////////////////////////////////

/// FileShareAccessControlEntry_AccessControlType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileShareAccessControlEntry_AccessControlType {
    /// Allow
    #[serde(rename = "Allow")]
    Allow = 0,
    /// Deny
    #[serde(rename = "Deny")]
    Deny = 1,
}

impl Default for FileShareAccessControlEntry_AccessControlType {
    fn default() -> Self {
        Self::Allow
    }
}

