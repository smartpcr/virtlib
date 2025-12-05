// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReplicaPeer_PeerObjectType
//////////////////////////////////////////////

/// ReplicaPeer_PeerObjectType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReplicaPeer_PeerObjectType {
    /// VirtualDisk
    #[serde(rename = "VirtualDisk")]
    VirtualDisk = 4,
    /// Volume
    #[serde(rename = "Volume")]
    Volume = 5,
    /// Partition
    #[serde(rename = "Partition")]
    Partition = 6,
    /// ReplicationGroup
    #[serde(rename = "ReplicationGroup")]
    ReplicationGroup = 7,
    /// StorageSubSystem
    #[serde(rename = "StorageSubSystem")]
    StorageSubSystem = 8,
}

impl Default for ReplicaPeer_PeerObjectType {
    fn default() -> Self {
        Self::VirtualDisk
    }
}

