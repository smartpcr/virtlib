// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNodeToDisk_OfflineReason
//////////////////////////////////////////////

/// StorageNodeToDisk_OfflineReason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNodeToDisk_OfflineReason {
    /// Policy
    #[serde(rename = "Policy")]
    Policy = 1,
    /// Redundant_Path
    #[serde(rename = "Redundant_Path")]
    RedundantPath = 2,
    /// Snapshot
    #[serde(rename = "Snapshot")]
    Snapshot = 3,
    /// Collision
    #[serde(rename = "Collision")]
    Collision = 4,
    /// Resource_Exhaustion
    #[serde(rename = "Resource_Exhaustion")]
    ResourceExhaustion = 5,
    /// Critical_Write_Failures
    #[serde(rename = "Critical_Write_Failures")]
    CriticalWriteFailures = 6,
    /// Data_Integrity_Scan_Required
    #[serde(rename = "Data_Integrity_Scan_Required")]
    DataIntegrityScanRequired = 7,
}

impl Default for StorageNodeToDisk_OfflineReason {
    fn default() -> Self {
        Self::Policy
    }
}

