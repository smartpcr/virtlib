// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CollectionReplicationService_InitialReplicationType
//////////////////////////////////////////////

/// CollectionReplicationService_InitialReplicationType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CollectionReplicationService_InitialReplicationType {
    /// Network_Transfer
    #[serde(rename = "Network_Transfer")]
    NetworkTransfer = 1,
    /// Export
    #[serde(rename = "Export")]
    Export = 2,
    /// Seeded_Network_Transfer
    #[serde(rename = "Seeded_Network_Transfer")]
    SeededNetworkTransfer = 3,
}

impl Default for CollectionReplicationService_InitialReplicationType {
    fn default() -> Self {
        Self::NetworkTransfer
    }
}

