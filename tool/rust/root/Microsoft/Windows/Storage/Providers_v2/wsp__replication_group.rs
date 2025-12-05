// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WSP_ReplicationGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WSP_ReplicationGroup {
    #[serde(flatten)]
    pub base: MSFT_ReplicationGroup,
}

impl WSP_ReplicationGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_ReplicationGroup::new(),
        }
    }


/// 

    /// * `description` -  (String)
    /// * `friendly_name` -  (String)
    /// * `log_device` -  (String)
    /// * `log_size_in_bytes` -  (u64)
    /// * `replication_quorum` -  (u16)
    /// * `storage_elements` -  (String[])
    /// * `sync_mode` -  (u16)

    /// * `created_replication_group` -  (String)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_replication_group(&self, friendly_name: &String, description: &String, storage_elements: &Vec<String>, log_device: &String, log_size_in_bytes: u64, replication_quorum: u16, sync_mode: u16, created_replication_group: &mut String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "StorageElements".to_string(), value: storage_elements.into() });
        args.push(MethodParameter { name: "LogDevice".to_string(), value: log_device.into() });
        args.push(MethodParameter { name: "LogSizeInBytes".to_string(), value: log_size_in_bytes.into() });
        args.push(MethodParameter { name: "ReplicationQuorum".to_string(), value: replication_quorum.into() });
        args.push(MethodParameter { name: "SyncMode".to_string(), value: sync_mode.into() });

        let result = self.invoke_method("CreateReplicationGroup", &args)?;
        let created_replication_group = result.get_value("CreatedReplicationGroup")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `recovery_point_objective` -  (u16)
    /// * `source_group_settings` -  (MSFT_ReplicationSettings)
    /// * `source_replication_group_description` -  (String)
    /// * `source_replication_group_friendly_name` -  (String)
    /// * `source_storage_elements` -  (MSFT_StorageObject[])
    /// * `target_group_settings` -  (MSFT_ReplicationSettings)
    /// * `target_replication_group_description` -  (String)
    /// * `target_replication_group_friendly_name` -  (String)
    /// * `target_storage_elements` -  (MSFT_StorageObject[])
    /// * `target_storage_pool` -  (MSFT_StoragePool)
    /// * `target_storage_subsystem` -  (MSFT_ReplicaPeer)

    /// * `created_replica_peer` -  (MSFT_ReplicaPeer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `source_group` -  (MSFT_ReplicationGroup)
    /// * `target_group` -  (MSFT_ReplicationGroup)
    pub fn create_replication_relationship(&self, target_storage_subsystem: MSFT_ReplicaPeer, source_replication_group_friendly_name: &String, source_replication_group_description: &String, source_storage_elements: &Vec<MSFT_StorageObject>, source_group_settings: MSFT_ReplicationSettings, target_replication_group_friendly_name: &String, target_replication_group_description: &String, target_storage_elements: &Vec<MSFT_StorageObject>, target_storage_pool: MSFT_StoragePool, target_group_settings: MSFT_ReplicationSettings, recovery_point_objective: u16, source_group: &mut MSFT_ReplicationGroup, target_group: &mut MSFT_ReplicationGroup, created_replica_peer: &mut MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetStorageSubsystem".to_string(), value: target_storage_subsystem.into() });
        args.push(MethodParameter { name: "SourceReplicationGroupFriendlyName".to_string(), value: source_replication_group_friendly_name.into() });
        args.push(MethodParameter { name: "SourceReplicationGroupDescription".to_string(), value: source_replication_group_description.into() });
        args.push(MethodParameter { name: "SourceStorageElements".to_string(), value: source_storage_elements.into() });
        args.push(MethodParameter { name: "SourceGroupSettings".to_string(), value: source_group_settings.into() });
        args.push(MethodParameter { name: "TargetReplicationGroupFriendlyName".to_string(), value: target_replication_group_friendly_name.into() });
        args.push(MethodParameter { name: "TargetReplicationGroupDescription".to_string(), value: target_replication_group_description.into() });
        args.push(MethodParameter { name: "TargetStorageElements".to_string(), value: target_storage_elements.into() });
        args.push(MethodParameter { name: "TargetStoragePool".to_string(), value: target_storage_pool.into() });
        args.push(MethodParameter { name: "TargetGroupSettings".to_string(), value: target_group_settings.into() });
        args.push(MethodParameter { name: "RecoveryPointObjective".to_string(), value: recovery_point_objective.into() });

        let result = self.invoke_method("CreateReplicationRelationship", &args)?;
        let created_replica_peer = result.get_value("CreatedReplicaPeer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let source_group = result.get_value("SourceGroup")?;
        let target_group = result.get_value("TargetGroup")?;
        Ok(result.return_value)

    }


/// 

    /// * `source_replication_group` -  (MSFT_ReplicationGroup)
    /// * `target_group_replica_peer` -  (MSFT_ReplicaPeer)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_replication_relationship(&self, source_replication_group: MSFT_ReplicationGroup, target_group_replica_peer: MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceReplicationGroup".to_string(), value: source_replication_group.into() });
        args.push(MethodParameter { name: "TargetGroupReplicaPeer".to_string(), value: target_group_replica_peer.into() });

        let result = self.invoke_method("DeleteReplicationRelationship", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

