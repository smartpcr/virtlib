// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDisk_FaultDomainAwareness
//////////////////////////////////////////////

/// VirtualDisk_FaultDomainAwareness enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDisk_FaultDomainAwareness {
    /// PhysicalDisk
    #[serde(rename = "PhysicalDisk")]
    PhysicalDisk = 1,
    /// StorageEnclosure
    #[serde(rename = "StorageEnclosure")]
    StorageEnclosure = 2,
    /// StorageScaleUnit
    #[serde(rename = "StorageScaleUnit")]
    StorageScaleUnit = 3,
    /// StorageChassis
    #[serde(rename = "StorageChassis")]
    StorageChassis = 4,
    /// StorageRack
    #[serde(rename = "StorageRack")]
    StorageRack = 5,
}

impl Default for VirtualDisk_FaultDomainAwareness {
    fn default() -> Self {
        Self::PhysicalDisk
    }
}

