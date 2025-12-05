// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Policies_LoadBalancePolicy
//////////////////////////////////////////////

/// Policies_LoadBalancePolicy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Policies_LoadBalancePolicy {
    /// Fail_Over_Only
    #[serde(rename = "Fail_Over_Only")]
    FailOverOnly = 1,
    /// Round_Robin
    #[serde(rename = "Round_Robin")]
    RoundRobin = 2,
    /// Round_Robin_with_Subset
    #[serde(rename = "Round_Robin_with_Subset")]
    RoundRobinWithSubset = 3,
    /// Dynamic_Least_Queue_Depth
    #[serde(rename = "Dynamic_Least_Queue_Depth")]
    DynamicLeastQueueDepth = 4,
    /// Weighted_Paths
    #[serde(rename = "Weighted_Paths")]
    WeightedPaths = 5,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 6,
}

impl Default for Policies_LoadBalancePolicy {
    fn default() -> Self {
        Self::FailOverOnly
    }
}

