// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNodeToPhysicalDisk_PathState
//////////////////////////////////////////////

/// StorageNodeToPhysicalDisk_PathState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNodeToPhysicalDisk_PathState {
    /// Unavailable
    #[serde(rename = "Unavailable")]
    Unavailable = 0,
    /// Active_Unoptimized
    #[serde(rename = "Active_Unoptimized")]
    ActiveUnoptimized = 1,
    /// Standby
    #[serde(rename = "Standby")]
    Standby = 2,
    /// Active_Optimized
    #[serde(rename = "Active_Optimized")]
    ActiveOptimized = 3,
}

impl Default for StorageNodeToPhysicalDisk_PathState {
    fn default() -> Self {
        Self::Unavailable
    }
}

