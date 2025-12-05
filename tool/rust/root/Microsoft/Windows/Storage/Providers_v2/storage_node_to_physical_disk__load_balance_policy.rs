// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNodeToPhysicalDisk_LoadBalancePolicy
//////////////////////////////////////////////

/// StorageNodeToPhysicalDisk_LoadBalancePolicy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNodeToPhysicalDisk_LoadBalancePolicy {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Fail_Over
    #[serde(rename = "Fail_Over")]
    FailOver = 1,
    /// Round_Robin
    #[serde(rename = "Round_Robin")]
    RoundRobin = 2,
    /// Round_Robin_with_Subset
    #[serde(rename = "Round_Robin_with_Subset")]
    RoundRobinWithSubset = 3,
    /// Least_Queue_Depth
    #[serde(rename = "Least_Queue_Depth")]
    LeastQueueDepth = 4,
    /// Weighted_Paths
    #[serde(rename = "Weighted_Paths")]
    WeightedPaths = 5,
    /// Least_Blocks
    #[serde(rename = "Least_Blocks")]
    LeastBlocks = 6,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 7,
}

impl Default for StorageNodeToPhysicalDisk_LoadBalancePolicy {
    fn default() -> Self {
        Self::Unknown
    }
}

