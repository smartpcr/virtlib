// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReplicationCapabilities_SupportedReplicationTypes
//////////////////////////////////////////////

/// ReplicationCapabilities_SupportedReplicationTypes enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReplicationCapabilities_SupportedReplicationTypes {
    /// Synchronous_Mirror_Local
    #[serde(rename = "Synchronous_Mirror_Local")]
    SynchronousMirrorLocal = 2,
    /// Asynchronous_Mirror_Local
    #[serde(rename = "Asynchronous_Mirror_Local")]
    AsynchronousMirrorLocal = 3,
    /// Synchronous_Mirror_Remote
    #[serde(rename = "Synchronous_Mirror_Remote")]
    SynchronousMirrorRemote = 4,
    /// Asynchronous_Mirror_Remote
    #[serde(rename = "Asynchronous_Mirror_Remote")]
    AsynchronousMirrorRemote = 5,
    /// Synchronous_Snapshot_Local
    #[serde(rename = "Synchronous_Snapshot_Local")]
    SynchronousSnapshotLocal = 6,
    /// Asynchronous_Snapshot_Local
    #[serde(rename = "Asynchronous_Snapshot_Local")]
    AsynchronousSnapshotLocal = 7,
    /// Synchronous_Snapshot_Remote
    #[serde(rename = "Synchronous_Snapshot_Remote")]
    SynchronousSnapshotRemote = 8,
    /// Asynchronous_Snapshot_Remote
    #[serde(rename = "Asynchronous_Snapshot_Remote")]
    AsynchronousSnapshotRemote = 9,
    /// Synchronous_Clone_Local
    #[serde(rename = "Synchronous_Clone_Local")]
    SynchronousCloneLocal = 10,
    /// Asynchronous_Clone_Local
    #[serde(rename = "Asynchronous_Clone_Local")]
    AsynchronousCloneLocal = 11,
    /// Synchronous_Clone_Remote
    #[serde(rename = "Synchronous_Clone_Remote")]
    SynchronousCloneRemote = 12,
    /// Asynchronous_Clone_Remote
    #[serde(rename = "Asynchronous_Clone_Remote")]
    AsynchronousCloneRemote = 13,
    /// Synchronous_TokenizedClone_Local
    #[serde(rename = "Synchronous_TokenizedClone_Local")]
    SynchronousTokenizedCloneLocal = 14,
    /// Asynchronous_TokenizedClone_Local
    #[serde(rename = "Asynchronous_TokenizedClone_Local")]
    AsynchronousTokenizedCloneLocal = 15,
    /// Synchronous_TokenizedClone_Remote
    #[serde(rename = "Synchronous_TokenizedClone_Remote")]
    SynchronousTokenizedCloneRemote = 16,
    /// Asynchronous_TokenizedClone_Remote
    #[serde(rename = "Asynchronous_TokenizedClone_Remote")]
    AsynchronousTokenizedCloneRemote = 17,
    /// Adaptive_Mirror_Local
    #[serde(rename = "Adaptive_Mirror_Local")]
    AdaptiveMirrorLocal = 18,
    /// Adaptive_Mirror_Remote
    #[serde(rename = "Adaptive_Mirror_Remote")]
    AdaptiveMirrorRemote = 19,
    /// Adaptive_Snapshot_Local
    #[serde(rename = "Adaptive_Snapshot_Local")]
    AdaptiveSnapshotLocal = 20,
    /// Adaptive_Snapshot_Remote
    #[serde(rename = "Adaptive_Snapshot_Remote")]
    AdaptiveSnapshotRemote = 21,
    /// Adaptive_Clone_Local
    #[serde(rename = "Adaptive_Clone_Local")]
    AdaptiveCloneLocal = 22,
    /// Adaptive_Clone_Remote
    #[serde(rename = "Adaptive_Clone_Remote")]
    AdaptiveCloneRemote = 23,
    /// Adaptive_TokenizedClone_Local
    #[serde(rename = "Adaptive_TokenizedClone_Local")]
    AdaptiveTokenizedCloneLocal = 24,
    /// Adaptive_TokenizedClone_Remote
    #[serde(rename = "Adaptive_TokenizedClone_Remote")]
    AdaptiveTokenizedCloneRemote = 25,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 26,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 27,
}

impl Default for ReplicationCapabilities_SupportedReplicationTypes {
    fn default() -> Self {
        Self::SynchronousMirrorLocal
    }
}

