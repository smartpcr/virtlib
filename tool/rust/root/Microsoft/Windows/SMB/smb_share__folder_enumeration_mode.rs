// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_FolderEnumerationMode
//////////////////////////////////////////////

/// SmbShare_FolderEnumerationMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_FolderEnumerationMode {
    /// _16
    #[serde(rename = "_16")]
    V16 = 0,
    /// _17
    #[serde(rename = "_17")]
    V17 = 1,
}

impl Default for SmbShare_FolderEnumerationMode {
    fn default() -> Self {
        Self::V16
    }
}

