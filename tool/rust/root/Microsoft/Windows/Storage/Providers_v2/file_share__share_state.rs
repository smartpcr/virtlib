// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileShare_ShareState
//////////////////////////////////////////////

/// FileShare_ShareState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileShare_ShareState {
    /// Pending
    #[serde(rename = "Pending")]
    Pending = 0,
    /// Online
    #[serde(rename = "Online")]
    Online = 1,
    /// Offline
    #[serde(rename = "Offline")]
    Offline = 2,
}

impl Default for FileShare_ShareState {
    fn default() -> Self {
        Self::Pending
    }
}

