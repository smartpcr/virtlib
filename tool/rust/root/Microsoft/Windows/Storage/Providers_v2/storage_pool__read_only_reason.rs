// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_ReadOnlyReason
//////////////////////////////////////////////

/// StoragePool_ReadOnlyReason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_ReadOnlyReason {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// None
    #[serde(rename = "None")]
    None = 1,
    /// By_Policy
    #[serde(rename = "By_Policy")]
    ByPolicy = 2,
    /// Majority_Disks_Unhealthy
    #[serde(rename = "Majority_Disks_Unhealthy")]
    MajorityDisksUnhealthy = 3,
    /// Starting
    #[serde(rename = "Starting")]
    Starting = 4,
}

impl Default for StoragePool_ReadOnlyReason {
    fn default() -> Self {
        Self::Unknown
    }
}

