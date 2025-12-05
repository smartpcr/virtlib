// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReplicationCapabilities_SupportedObjectTypes
//////////////////////////////////////////////

/// ReplicationCapabilities_SupportedObjectTypes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReplicationCapabilities_SupportedObjectTypes {
    /// VirtualDisk
    #[serde(rename = "VirtualDisk")]
    VirtualDisk = 2,
    /// Volume
    #[serde(rename = "Volume")]
    Volume = 3,
    /// ReplicaPeer
    #[serde(rename = "ReplicaPeer")]
    ReplicaPeer = 4,
    /// Partition
    #[serde(rename = "Partition")]
    Partition = 5,
    /// ReplicationGroup
    #[serde(rename = "ReplicationGroup")]
    ReplicationGroup = 6,
    /// StorageSubSystem
    #[serde(rename = "StorageSubSystem")]
    StorageSubSystem = 7,
}

impl Default for ReplicationCapabilities_SupportedObjectTypes {
    fn default() -> Self {
        Self::VirtualDisk
    }
}

