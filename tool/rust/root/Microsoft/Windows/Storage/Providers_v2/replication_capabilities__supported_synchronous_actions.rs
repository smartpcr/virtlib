// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ReplicationCapabilities_SupportedSynchronousActions
//////////////////////////////////////////////

/// ReplicationCapabilities_SupportedSynchronousActions enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ReplicationCapabilities_SupportedSynchronousActions {
    /// CreateElementReplica
    #[serde(rename = "CreateElementReplica")]
    CreateElementReplica = 2,
    /// CreateGroupReplica
    #[serde(rename = "CreateGroupReplica")]
    CreateGroupReplica = 3,
    /// CreateSynchronizationAspect
    #[serde(rename = "CreateSynchronizationAspect")]
    CreateSynchronizationAspect = 4,
    /// ModifyReplicaSynchronization
    #[serde(rename = "ModifyReplicaSynchronization")]
    ModifyReplicaSynchronization = 5,
    /// ModifyListSynchronization
    #[serde(rename = "ModifyListSynchronization")]
    ModifyListSynchronization = 6,
    /// ModifySettingsDefineState
    #[serde(rename = "ModifySettingsDefineState")]
    ModifySettingsDefineState = 7,
    /// GetAvailableTargetElements
    #[serde(rename = "GetAvailableTargetElements")]
    GetAvailableTargetElements = 8,
    /// GetPeerSystems
    #[serde(rename = "GetPeerSystems")]
    GetPeerSystems = 9,
    /// GetReplicationRelationships
    #[serde(rename = "GetReplicationRelationships")]
    GetReplicationRelationships = 10,
    /// GetServiceAccessPoints
    #[serde(rename = "GetServiceAccessPoints")]
    GetServiceAccessPoints = 11,
    /// CreateGroup
    #[serde(rename = "CreateGroup")]
    CreateGroup = 12,
    /// DeleteGroup
    #[serde(rename = "DeleteGroup")]
    DeleteGroup = 13,
    /// AddMembers
    #[serde(rename = "AddMembers")]
    AddMembers = 14,
    /// RemoveMembers
    #[serde(rename = "RemoveMembers")]
    RemoveMembers = 15,
    /// AddReplicationEntity
    #[serde(rename = "AddReplicationEntity")]
    AddReplicationEntity = 16,
    /// AddServiceAccessPoint
    #[serde(rename = "AddServiceAccessPoint")]
    AddServiceAccessPoint = 17,
    /// AddSharedSecret
    #[serde(rename = "AddSharedSecret")]
    AddSharedSecret = 18,
    /// CreateListReplica
    #[serde(rename = "CreateListReplica")]
    CreateListReplica = 19,
    /// CreateGroupReplicaFromElements
    #[serde(rename = "CreateGroupReplicaFromElements")]
    CreateGroupReplicaFromElements = 20,
    /// GetReplicationRelationshipInstances
    #[serde(rename = "GetReplicationRelationshipInstances")]
    GetReplicationRelationshipInstances = 21,
    /// ModifyListSettingsDefineState
    #[serde(rename = "ModifyListSettingsDefineState")]
    ModifyListSettingsDefineState = 22,
    /// CreateRemoteReplicationCollection
    #[serde(rename = "CreateRemoteReplicationCollection")]
    CreateRemoteReplicationCollection = 23,
    /// AddToRemoteReplicationCollection
    #[serde(rename = "AddToRemoteReplicationCollection")]
    AddToRemoteReplicationCollection = 24,
    /// RemoveFromRemoteReplicationCollection
    #[serde(rename = "RemoveFromRemoteReplicationCollection")]
    RemoveFromRemoteReplicationCollection = 25,
    /// GetSynchronizationAspects
    #[serde(rename = "GetSynchronizationAspects")]
    GetSynchronizationAspects = 26,
    /// GetSynchronizationAspectInstances
    #[serde(rename = "GetSynchronizationAspectInstances")]
    GetSynchronizationAspectInstances = 27,
    /// CreateGroupReplicaFromElementSynchronizations
    #[serde(rename = "CreateGroupReplicaFromElementSynchronizations")]
    CreateGroupReplicaFromElementSynchronizations = 28,
    /// AddElementsToGroupSynchronized
    #[serde(rename = "AddElementsToGroupSynchronized")]
    AddElementsToGroupSynchronized = 29,
    /// ConfirmTargetData
    #[serde(rename = "ConfirmTargetData")]
    ConfirmTargetData = 30,
    /// CreateListSynchronizationAspect
    #[serde(rename = "CreateListSynchronizationAspect")]
    CreateListSynchronizationAspect = 31,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 32,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 33,
}

impl Default for ReplicationCapabilities_SupportedSynchronousActions {
    fn default() -> Self {
        Self::CreateElementReplica
    }
}

