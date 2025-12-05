// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_FaultDomainAwarenessDefault
//////////////////////////////////////////////

/// StorageSubSystem_FaultDomainAwarenessDefault enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_FaultDomainAwarenessDefault {
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

impl Default for StorageSubSystem_FaultDomainAwarenessDefault {
    fn default() -> Self {
        Self::PhysicalDisk
    }
}

