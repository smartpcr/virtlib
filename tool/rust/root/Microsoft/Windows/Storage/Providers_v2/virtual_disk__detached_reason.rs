// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDisk_DetachedReason
//////////////////////////////////////////////

/// VirtualDisk_DetachedReason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDisk_DetachedReason {
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
    /// Incomplete
    #[serde(rename = "Incomplete")]
    Incomplete = 4,
    /// Timeout
    #[serde(rename = "Timeout")]
    Timeout = 5,
}

impl Default for VirtualDisk_DetachedReason {
    fn default() -> Self {
        Self::Unknown
    }
}

