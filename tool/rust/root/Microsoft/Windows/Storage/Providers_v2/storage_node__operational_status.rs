// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNode_OperationalStatus
//////////////////////////////////////////////

/// StorageNode_OperationalStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNode_OperationalStatus {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Up
    #[serde(rename = "Up")]
    Up = 2,
    /// Down
    #[serde(rename = "Down")]
    Down = 6,
    /// Joining
    #[serde(rename = "Joining")]
    Joining = 8,
    /// Paused
    #[serde(rename = "Paused")]
    Paused = 10,
}

impl Default for StorageNode_OperationalStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

