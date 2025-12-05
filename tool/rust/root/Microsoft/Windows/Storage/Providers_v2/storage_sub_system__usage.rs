// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageSubSystem_Usage
//////////////////////////////////////////////

/// StorageSubSystem_Usage enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageSubSystem_Usage {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unrestricted
    #[serde(rename = "Unrestricted")]
    Unrestricted = 2,
    /// Reserved_for_ComputerSystem__the_block_server_
    #[serde(rename = "Reserved_for_ComputerSystem__the_block_server_")]
    ReservedForComputerSystemTheBlockServer = 3,
    /// Reserved_as_a_Delta_Replica_Container
    #[serde(rename = "Reserved_as_a_Delta_Replica_Container")]
    ReservedAsADeltaReplicaContainer = 4,
    /// Reserved_for_Migration_Services
    #[serde(rename = "Reserved_for_Migration_Services")]
    ReservedForMigrationServices = 5,
    /// Reserved_for_Local_Replication_Services
    #[serde(rename = "Reserved_for_Local_Replication_Services")]
    ReservedForLocalReplicationServices = 6,
    /// Reserved_for_Remote_Replication_Services
    #[serde(rename = "Reserved_for_Remote_Replication_Services")]
    ReservedForRemoteReplicationServices = 7,
    /// Reserved_for_Sparing
    #[serde(rename = "Reserved_for_Sparing")]
    ReservedForSparing = 8,
}

impl Default for StorageSubSystem_Usage {
    fn default() -> Self {
        Self::Other
    }
}

