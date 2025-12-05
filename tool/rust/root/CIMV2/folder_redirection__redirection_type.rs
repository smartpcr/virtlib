// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FolderRedirection_RedirectionType
//////////////////////////////////////////////

/// FolderRedirection_RedirectionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FolderRedirection_RedirectionType {
    /// ToFullPath
    #[serde(rename = "ToFullPath")]
    ToFullPath = 0,
    /// ToLocalUserProfile
    #[serde(rename = "ToLocalUserProfile")]
    ToLocalUserProfile = 1,
}

impl Default for FolderRedirection_RedirectionType {
    fn default() -> Self {
        Self::ToFullPath
    }
}

