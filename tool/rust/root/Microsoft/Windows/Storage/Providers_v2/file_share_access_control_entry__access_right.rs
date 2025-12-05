// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileShareAccessControlEntry_AccessRight
//////////////////////////////////////////////

/// FileShareAccessControlEntry_AccessRight enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileShareAccessControlEntry_AccessRight {
    /// Full
    #[serde(rename = "Full")]
    Full = 0,
    /// Modify
    #[serde(rename = "Modify")]
    Modify = 1,
    /// Read
    #[serde(rename = "Read")]
    Read = 2,
    /// Custom
    #[serde(rename = "Custom")]
    Custom = 3,
}

impl Default for FileShareAccessControlEntry_AccessRight {
    fn default() -> Self {
        Self::Full
    }
}

