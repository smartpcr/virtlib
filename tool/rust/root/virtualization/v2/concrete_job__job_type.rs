// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ConcreteJob_JobType
//////////////////////////////////////////////

/// ConcreteJob_JobType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ConcreteJob_JobType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Define_Virtual_Machine
    #[serde(rename = "Define_Virtual_Machine")]
    DefineVirtualMachine = 1,
    /// Modify_Virtual_Machine
    #[serde(rename = "Modify_Virtual_Machine")]
    ModifyVirtualMachine = 2,
    /// Destroy_Virtual_Machine
    #[serde(rename = "Destroy_Virtual_Machine")]
    DestroyVirtualMachine = 3,
    /// Modify_Management_Service_Settings
    #[serde(rename = "Modify_Management_Service_Settings")]
    ModifyManagementServiceSettings = 4,
    /// Initialize_Virtual_Machine
    #[serde(rename = "Initialize_Virtual_Machine")]
    InitializeVirtualMachine = 10,
    /// Waiting_to_Start_Virtual_Machine
    #[serde(rename = "Waiting_to_Start_Virtual_Machine")]
    WaitingToStartVirtualMachine = 11,
    /// Start_Virtual_Machine
    #[serde(rename = "Start_Virtual_Machine")]
    StartVirtualMachine = 12,
    /// Power_Off_Virtual_Machine
    #[serde(rename = "Power_Off_Virtual_Machine")]
    PowerOffVirtualMachine = 13,
    /// Save_Virtual_Machine
    #[serde(rename = "Save_Virtual_Machine")]
    SaveVirtualMachine = 14,
    /// Restore_Virtual_Machine
    #[serde(rename = "Restore_Virtual_Machine")]
    RestoreVirtualMachine = 15,
    /// Shut_Down_Virtual_Machine
    #[serde(rename = "Shut_Down_Virtual_Machine")]
    ShutDownVirtualMachine = 16,
    /// Pause_Virtual_Machine
    #[serde(rename = "Pause_Virtual_Machine")]
    PauseVirtualMachine = 26,
    /// Resume_Virtual_Machine
    #[serde(rename = "Resume_Virtual_Machine")]
    ResumeVirtualMachine = 27,
    /// Reset_Virtual_Machine
    #[serde(rename = "Reset_Virtual_Machine")]
    ResetVirtualMachine = 28,
    /// Reboot_Virtual_Machine
    #[serde(rename = "Reboot_Virtual_Machine")]
    RebootVirtualMachine = 29,
    /// Add_Virtual_Machine_Resources
    #[serde(rename = "Add_Virtual_Machine_Resources")]
    AddVirtualMachineResources = 30,
    /// Modify_Virtual_Machine_Resources
    #[serde(rename = "Modify_Virtual_Machine_Resources")]
    ModifyVirtualMachineResources = 31,
    /// Remove_Virtual_Machine_Resources
    #[serde(rename = "Remove_Virtual_Machine_Resources")]
    RemoveVirtualMachineResources = 32,
    /// Request_Initial_Virtual_Machine_Memory
    #[serde(rename = "Request_Initial_Virtual_Machine_Memory")]
    RequestInitialVirtualMachineMemory = 40,
    /// Add_Memory_to_Virtual_Machine
    #[serde(rename = "Add_Memory_to_Virtual_Machine")]
    AddMemoryToVirtualMachine = 41,
    /// Remove_Memory_from_Virtual_Machine
    #[serde(rename = "Remove_Memory_from_Virtual_Machine")]
    RemoveMemoryFromVirtualMachine = 42,
    /// Merging_VHD_Disks
    #[serde(rename = "Merging_VHD_Disks")]
    MergingVHDDisks = 50,
    /// Create_VSS_Snapshot_inside_Virtual_Machine
    #[serde(rename = "Create_VSS_Snapshot_inside_Virtual_Machine")]
    CreateVSSSnapshotInsideVirtualMachine = 51,
    /// Get_Import_Setting_Data
    #[serde(rename = "Get_Import_Setting_Data")]
    GetImportSettingData = 60,
    /// Import_Virtual_Machine
    #[serde(rename = "Import_Virtual_Machine")]
    ImportVirtualMachine = 61,
    /// Export_Virtual_Machine
    #[serde(rename = "Export_Virtual_Machine")]
    ExportVirtualMachine = 62,
    /// Register_Configuration
    #[serde(rename = "Register_Configuration")]
    RegisterConfiguration = 63,
    /// Unregister_Configuration
    #[serde(rename = "Unregister_Configuration")]
    UnregisterConfiguration = 64,
    /// Snapshot_Virtual_Machine
    #[serde(rename = "Snapshot_Virtual_Machine")]
    SnapshotVirtualMachine = 70,
    /// Apply_Virtual_Machine_Snapshot
    #[serde(rename = "Apply_Virtual_Machine_Snapshot")]
    ApplyVirtualMachineSnapshot = 71,
    /// Delete_Virtual_Machine_Snapshot
    #[serde(rename = "Delete_Virtual_Machine_Snapshot")]
    DeleteVirtualMachineSnapshot = 72,
    /// Clear_Virtual_Machine_Snapshot_State
    #[serde(rename = "Clear_Virtual_Machine_Snapshot_State")]
    ClearVirtualMachineSnapshotState = 73,
    /// Add_Resources_to_Resource_Pool
    #[serde(rename = "Add_Resources_to_Resource_Pool")]
    AddResourcesToResourcePool = 80,
    /// Remove_Resources_from_Resource_Pool
    #[serde(rename = "Remove_Resources_from_Resource_Pool")]
    RemoveResourcesFromResourcePool = 81,
    /// Modify_Replication_Server_Settings
    #[serde(rename = "Modify_Replication_Server_Settings")]
    ModifyReplicationServerSettings = 90,
    /// Create_Replication_Relationship
    #[serde(rename = "Create_Replication_Relationship")]
    CreateReplicationRelationship = 91,
    /// Modify_Replication_Relationship_Settings
    #[serde(rename = "Modify_Replication_Relationship_Settings")]
    ModifyReplicationRelationshipSettings = 92,
    /// Remove_Replication_Relationship
    #[serde(rename = "Remove_Replication_Relationship")]
    RemoveReplicationRelationship = 93,
    /// Start_Inband_Initial_Replication
    #[serde(rename = "Start_Inband_Initial_Replication")]
    StartInbandInitialReplication = 94,
    /// Import_Replication
    #[serde(rename = "Import_Replication")]
    ImportReplication = 95,
    /// Replicate_State_Change
    #[serde(rename = "Replicate_State_Change")]
    ReplicateStateChange = 96,
    /// Initiate_Failover
    #[serde(rename = "Initiate_Failover")]
    InitiateFailover = 97,
    /// Revert_Failover
    #[serde(rename = "Revert_Failover")]
    RevertFailover = 98,
    /// Commit_Failover
    #[serde(rename = "Commit_Failover")]
    CommitFailover = 99,
    /// Inititate_Synced_Replication
    #[serde(rename = "Inititate_Synced_Replication")]
    InititateSyncedReplication = 100,
    /// Cancel_Synced_Replication
    #[serde(rename = "Cancel_Synced_Replication")]
    CancelSyncedReplication = 101,
    /// Initiate_Test_Replica
    #[serde(rename = "Initiate_Test_Replica")]
    InitiateTestReplica = 102,
    /// Remove_Test_Replica
    #[serde(rename = "Remove_Test_Replica")]
    RemoveTestReplica = 103,
    /// Reverse_Replication
    #[serde(rename = "Reverse_Replication")]
    ReverseReplication = 104,
    /// Replication_Sending_Delta
    #[serde(rename = "Replication_Sending_Delta")]
    ReplicationSendingDelta = 105,
    /// Replication_Receiving_Delta
    #[serde(rename = "Replication_Receiving_Delta")]
    ReplicationReceivingDelta = 106,
    /// Resynchronizing
    #[serde(rename = "Resynchronizing")]
    Resynchronizing = 107,
    /// Apply_change_log
    #[serde(rename = "Apply_change_log")]
    ApplyChangeLog = 108,
    /// Stop_Initial_Replication
    #[serde(rename = "Stop_Initial_Replication")]
    StopInitialReplication = 109,
    /// Stop_Resynchronizing
    #[serde(rename = "Stop_Resynchronizing")]
    StopResynchronizing = 110,
    /// Get_Replica_statistics
    #[serde(rename = "Get_Replica_statistics")]
    GetReplicaStatistics = 111,
    /// Prepare_for_Consistency_Checker
    #[serde(rename = "Prepare_for_Consistency_Checker")]
    PrepareForConsistencyChecker = 112,
    /// Consistency_Checker
    #[serde(rename = "Consistency_Checker")]
    ConsistencyChecker = 113,
    /// Stop_Consistency_Checker
    #[serde(rename = "Stop_Consistency_Checker")]
    StopConsistencyChecker = 114,
    /// Test_Replication_Connection
    #[serde(rename = "Test_Replication_Connection")]
    TestReplicationConnection = 115,
    /// Sending_Initial_Replica
    #[serde(rename = "Sending_Initial_Replica")]
    SendingInitialReplica = 116,
    /// Start_Resync_Initial_Replication
    #[serde(rename = "Start_Resync_Initial_Replication")]
    StartResyncInitialReplication = 117,
    /// Start_Export_Initial_Replication
    #[serde(rename = "Start_Export_Initial_Replication")]
    StartExportInitialReplication = 118,
    /// Reset_Replica_Statistics
    #[serde(rename = "Reset_Replica_Statistics")]
    ResetReplicaStatistics = 119,
    /// Apply_Registered_Deltas
    #[serde(rename = "Apply_Registered_Deltas")]
    ApplyRegisteredDeltas = 120,
    /// Resynchronizing_Extended_Replication
    #[serde(rename = "Resynchronizing_Extended_Replication")]
    ResynchronizingExtendedReplication = 121,
    /// Reading_Test_Replica_Configuration
    #[serde(rename = "Reading_Test_Replica_Configuration")]
    ReadingTestReplicaConfiguration = 122,
    /// Change_the_replication_mode_to_primary
    #[serde(rename = "Change_the_replication_mode_to_primary")]
    ChangeTheReplicationModeToPrimary = 123,
    /// Initiate_Failback
    #[serde(rename = "Initiate_Failback")]
    InitiateFailback = 124,
    /// Update_Disk_Set
    #[serde(rename = "Update_Disk_Set")]
    UpdateDiskSet = 125,
    /// Define_Ethernet_Switch
    #[serde(rename = "Define_Ethernet_Switch")]
    DefineEthernetSwitch = 130,
    /// Modify_Ethernet_Switch_Settings
    #[serde(rename = "Modify_Ethernet_Switch_Settings")]
    ModifyEthernetSwitchSettings = 131,
    /// Destroy_Ethernet_Switch
    #[serde(rename = "Destroy_Ethernet_Switch")]
    DestroyEthernetSwitch = 132,
    /// Add_Ethernet_Switch_Resources
    #[serde(rename = "Add_Ethernet_Switch_Resources")]
    AddEthernetSwitchResources = 133,
    /// Modify_Ethernet_Switch_Resources
    #[serde(rename = "Modify_Ethernet_Switch_Resources")]
    ModifyEthernetSwitchResources = 134,
    /// Remove_Ethernet_Switch_Resources
    #[serde(rename = "Remove_Ethernet_Switch_Resources")]
    RemoveEthernetSwitchResources = 135,
    /// Validate_Planned_Virtual_Machine
    #[serde(rename = "Validate_Planned_Virtual_Machine")]
    ValidatePlannedVirtualMachine = 140,
    /// Realizing_Virtual_Machine
    #[serde(rename = "Realizing_Virtual_Machine")]
    RealizingVirtualMachine = 141,
    /// Creating_a_Resource_Pool
    #[serde(rename = "Creating_a_Resource_Pool")]
    CreatingAResourcePool = 150,
    /// Changing_the_Parent_Resources_of_a_Resource_Pool
    #[serde(rename = "Changing_the_Parent_Resources_of_a_Resource_Pool")]
    ChangingTheParentResourcesOfAResourcePool = 151,
    /// Changing_the_Non_alloction_Settings_of_a_Resource_Pool
    #[serde(rename = "Changing_the_Non_alloction_Settings_of_a_Resource_Pool")]
    ChangingTheNonAlloctionSettingsOfAResourcePool = 152,
    /// Deleting_a_Resource_Pool
    #[serde(rename = "Deleting_a_Resource_Pool")]
    DeletingAResourcePool = 153,
    /// Enable_RemoteFx_GPU
    #[serde(rename = "Enable_RemoteFx_GPU")]
    EnableRemoteFxGPU = 160,
    /// Disable_RemoteFx_GPU
    #[serde(rename = "Disable_RemoteFx_GPU")]
    DisableRemoteFxGPU = 161,
    /// Modify_3D_Service_Settings
    #[serde(rename = "Modify_3D_Service_Settings")]
    Modify3DServiceSettings = 162,
    /// Backup_Virtual_Machine
    #[serde(rename = "Backup_Virtual_Machine")]
    BackupVirtualMachine = 170,
    /// Guest_Service_Interface
    #[serde(rename = "Guest_Service_Interface")]
    GuestServiceInterface = 180,
    /// Query_Guest_Cluster_Information
    #[serde(rename = "Query_Guest_Cluster_Information")]
    QueryGuestClusterInformation = 181,
    /// Define_Collection
    #[serde(rename = "Define_Collection")]
    DefineCollection = 190,
    /// Destroy_Collection
    #[serde(rename = "Destroy_Collection")]
    DestroyCollection = 191,
    /// Rename_Collection
    #[serde(rename = "Rename_Collection")]
    RenameCollection = 192,
    /// Add_Member_to_Collection
    #[serde(rename = "Add_Member_to_Collection")]
    AddMemberToCollection = 193,
    /// Remove_Member_from_Collection
    #[serde(rename = "Remove_Member_from_Collection")]
    RemoveMemberFromCollection = 194,
    /// Add_Setting_to_Collection
    #[serde(rename = "Add_Setting_to_Collection")]
    AddSettingToCollection = 195,
    /// Remove_Setting_from_Collection
    #[serde(rename = "Remove_Setting_from_Collection")]
    RemoveSettingFromCollection = 196,
    /// Modify_Setting_on_Collection
    #[serde(rename = "Modify_Setting_on_Collection")]
    ModifySettingOnCollection = 197,
    /// Snapshot_Collection
    #[serde(rename = "Snapshot_Collection")]
    SnapshotCollection = 198,
    /// Convert_Snapshot_to_Reference_Point
    #[serde(rename = "Convert_Snapshot_to_Reference_Point")]
    ConvertSnapshotToReferencePoint = 200,
    /// Create_Reference_Point
    #[serde(rename = "Create_Reference_Point")]
    CreateReferencePoint = 201,
    /// Delete_Reference_Point
    #[serde(rename = "Delete_Reference_Point")]
    DeleteReferencePoint = 202,
    /// Export_Reference_Point
    #[serde(rename = "Export_Reference_Point")]
    ExportReferencePoint = 203,
    /// Remove_Associated_Data_from_Reference_Point
    #[serde(rename = "Remove_Associated_Data_from_Reference_Point")]
    RemoveAssociatedDataFromReferencePoint = 204,
    /// Create_Reference_Point_on_Collection
    #[serde(rename = "Create_Reference_Point_on_Collection")]
    CreateReferencePointOnCollection = 205,
    /// Export_Reference_Point_on_Collection
    #[serde(rename = "Export_Reference_Point_on_Collection")]
    ExportReferencePointOnCollection = 206,
    /// Remove_Associated_Data_from_Reference_Point_on_Collection
    #[serde(rename = "Remove_Associated_Data_from_Reference_Point_on_Collection")]
    RemoveAssociatedDataFromReferencePointOnCollection = 207,
    /// Delete_Reference_Point_on_Collection
    #[serde(rename = "Delete_Reference_Point_on_Collection")]
    DeleteReferencePointOnCollection = 208,
    /// Import_Reference_Point_metadata
    #[serde(rename = "Import_Reference_Point_metadata")]
    ImportReferencePointMetadata = 209,
    /// Mount_or_Dismount_Assignable_Device
    #[serde(rename = "Mount_or_Dismount_Assignable_Device")]
    MountOrDismountAssignableDevice = 260,
}

impl Default for ConcreteJob_JobType {
    fn default() -> Self {
        Self::Unknown
    }
}

