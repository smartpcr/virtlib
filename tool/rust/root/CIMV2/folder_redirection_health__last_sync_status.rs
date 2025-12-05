// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FolderRedirectionHealth_LastSyncStatus
//////////////////////////////////////////////

/// FolderRedirectionHealth_LastSyncStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FolderRedirectionHealth_LastSyncStatus {
    /// Success
    #[serde(rename = "Success")]
    Success = 0,
    /// Conflict
    #[serde(rename = "Conflict")]
    Conflict = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Error
    #[serde(rename = "Error")]
    Error = 3,
}

impl Default for FolderRedirectionHealth_LastSyncStatus {
    fn default() -> Self {
        Self::Success
    }
}

