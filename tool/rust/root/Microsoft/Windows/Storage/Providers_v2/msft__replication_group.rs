// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicationGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicationGroup {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// A user-friendly string representing the description of the replication group.
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// A user-friendly string representing the name of the replication group.
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// Denotes the current health status of the replication group. Health of a group is derived from the health of the backing storage replicas.
///  0 - 'Healthy': All replicas are in a healthy state. 
/// 1 - 'Warning': The majority of replicas are healthy, but one or more may be not fully synchronized. 
/// 2 - 'Unhealthy': The majority of replicas are unhealthy or in a failed state.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<ReplicationGroup_HealthStatus>,

/// Indicates the current operating conditions of the group. Unlike HealthStatus, this field indicates the status of hardware, software, and infrastructure issues related to this group, and can contain multiple values.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<ReplicationGroup_OperationalStatus>,
}

impl MSFT_ReplicationGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            description: None,
            friendly_name: None,
            health_status: None,
            operational_status: Vec::new(),
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: ReplicationGroup_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&ReplicationGroup_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<ReplicationGroup_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<ReplicationGroup_OperationalStatus> {
        &self.operational_status
    }

/// 

    /// * `friendly_name` -  (String)
    /// * `recovery_point_objective` -  (u32)
    /// * `replication_settings` -  (MSFT_ReplicationSettings)
    /// * `sync_type` -  (u16)
    /// * `target_group_object_id` -  (String)
    /// * `target_storage_pool_object_id` -  (String)
    /// * `target_storage_subsystem` -  (MSFT_ReplicaPeer)

    /// * `created_replica_peer` -  (MSFT_ReplicaPeer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_replica(&self, friendly_name: &String, target_storage_subsystem: MSFT_ReplicaPeer, target_group_object_id: &String, target_storage_pool_object_id: &String, recovery_point_objective: u32, replication_settings: MSFT_ReplicationSettings, sync_type: u16, created_replica_peer: &mut MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStorageSubsystem".to_string(), value: target_storage_subsystem.into() });
        args.push(MethodParameter { name: "TargetGroupObjectId".to_string(), value: target_group_object_id.into() });
        args.push(MethodParameter { name: "TargetStoragePoolObjectId".to_string(), value: target_storage_pool_object_id.into() });
        args.push(MethodParameter { name: "RecoveryPointObjective".to_string(), value: recovery_point_objective.into() });
        args.push(MethodParameter { name: "ReplicationSettings".to_string(), value: replication_settings.into() });
        args.push(MethodParameter { name: "SyncType".to_string(), value: sync_type.into() });

        let result = self.invoke_method("CreateReplica", &args)?;
        let created_replica_peer = result.get_value("CreatedReplicaPeer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `operation` -  (u16)
    /// * `source_storage_objects` -  (MSFT_StorageObject[])
    /// * `sync_pairs` -  (MSFT_Synchronized[])
    /// * `target_group` -  (MSFT_ReplicaPeer)
    /// * `target_storage_objects` -  (MSFT_StorageObject[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_replication_relationship(&self, operation: u16, target_group: MSFT_ReplicaPeer, source_storage_objects: &Vec<MSFT_StorageObject>, target_storage_objects: &Vec<MSFT_StorageObject>, sync_pairs: &Vec<MSFT_Synchronized>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Operation".to_string(), value: operation.into() });
        args.push(MethodParameter { name: "TargetGroup".to_string(), value: target_group.into() });
        args.push(MethodParameter { name: "SourceStorageObjects".to_string(), value: source_storage_objects.into() });
        args.push(MethodParameter { name: "TargetStorageObjects".to_string(), value: target_storage_objects.into() });
        args.push(MethodParameter { name: "SyncPairs".to_string(), value: sync_pairs.into() });

        let result = self.invoke_method("SetReplicationRelationship", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_settings` -  (MSFT_ReplicationSettings)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_replication_settings(&self, replication_settings: MSFT_ReplicationSettings, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationSettings".to_string(), value: replication_settings.into() });

        let result = self.invoke_method("SetReplicationSettings", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `replication_settings` -  (MSFT_ReplicationSettings)
    /// * `return_value` -  (u32)
    pub fn get_replication_settings(&self, replication_settings: &mut MSFT_ReplicationSettings, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetReplicationSettings", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let replication_settings = result.get_value("ReplicationSettings")?;
        Ok(result.return_value)

    }


/// 

    /// * `storage_objects` -  (MSFT_StorageObject[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_member(&self, storage_objects: &Vec<MSFT_StorageObject>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageObjects".to_string(), value: storage_objects.into() });

        let result = self.invoke_method("AddMember", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `storage_objects` -  (MSFT_StorageObject[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_member(&self, storage_objects: &Vec<MSFT_StorageObject>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageObjects".to_string(), value: storage_objects.into() });

        let result = self.invoke_method("RemoveMember", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_friendly_name(&self, friendly_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });

        let result = self.invoke_method("SetFriendlyName", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

