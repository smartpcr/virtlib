// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualDisk_Usage
//////////////////////////////////////////////

/// VirtualDisk_Usage enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualDisk_Usage {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unrestricted
    #[serde(rename = "Unrestricted")]
    Unrestricted = 2,
    /// Reserved_for_ComputerSystem__the_block_server_
    #[serde(rename = "Reserved_for_ComputerSystem__the_block_server_")]
    ReservedForComputerSystemTheBlockServer = 3,
    /// Reserved_by_Replication_Services
    #[serde(rename = "Reserved_by_Replication_Services")]
    ReservedByReplicationServices = 4,
    /// Reserved_by_Migration_Services
    #[serde(rename = "Reserved_by_Migration_Services")]
    ReservedByMigrationServices = 5,
    /// Local_Replica_Source
    #[serde(rename = "Local_Replica_Source")]
    LocalReplicaSource = 6,
    /// Remote_Replica_Source
    #[serde(rename = "Remote_Replica_Source")]
    RemoteReplicaSource = 7,
    /// Local_Replica_Target
    #[serde(rename = "Local_Replica_Target")]
    LocalReplicaTarget = 8,
    /// Remote_Replica_Target
    #[serde(rename = "Remote_Replica_Target")]
    RemoteReplicaTarget = 9,
    /// Local_Replica_Source_or_Target
    #[serde(rename = "Local_Replica_Source_or_Target")]
    LocalReplicaSourceOrTarget = 10,
    /// Remote_Replica_Source_or_Target
    #[serde(rename = "Remote_Replica_Source_or_Target")]
    RemoteReplicaSourceOrTarget = 11,
    /// Delta_Replica_Target
    #[serde(rename = "Delta_Replica_Target")]
    DeltaReplicaTarget = 12,
    /// Element_Component
    #[serde(rename = "Element_Component")]
    ElementComponent = 13,
    /// Reserved_as_Pool_Contributor
    #[serde(rename = "Reserved_as_Pool_Contributor")]
    ReservedAsPoolContributor = 14,
    /// Composite_Volume_Member
    #[serde(rename = "Composite_Volume_Member")]
    CompositeVolumeMember = 15,
    /// Composite_VirtualDisk_Member
    #[serde(rename = "Composite_VirtualDisk_Member")]
    CompositeVirtualDiskMember = 16,
    /// Reserved_for_Sparing
    #[serde(rename = "Reserved_for_Sparing")]
    ReservedForSparing = 17,
}

impl Default for VirtualDisk_Usage {
    fn default() -> Self {
        Self::Unknown
    }
}

