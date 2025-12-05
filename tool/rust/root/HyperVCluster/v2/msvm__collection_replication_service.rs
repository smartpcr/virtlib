// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionReplicationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionReplicationService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_CollectionReplicationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `collection_replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn create_replication_relationship(&self, collection: CIM_CollectionOfMSEs, collection_replication_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "CollectionReplicationSettingData".to_string(), value: collection_replication_setting_data.into() });

        let result = self.invoke_method_with_job("CreateReplicationRelationship", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `initial_replication_export_location` -  (String)
    /// * `initial_replication_type` -  (CollectionReplicationService_InitialReplicationType)
    /// * `start_time` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn start_replication(&self, collection: CIM_CollectionOfMSEs, initial_replication_type: CollectionReplicationService_InitialReplicationType, initial_replication_export_location: &String, start_time: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "InitialReplicationType".to_string(), value: initial_replication_type.into() });
        args.push(MethodParameter { name: "InitialReplicationExportLocation".to_string(), value: initial_replication_export_location.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });

        let result = self.invoke_method_with_job("StartReplication", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `collection_replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_replication_settings(&self, collection: CIM_CollectionOfMSEs, collection_replication_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "CollectionReplicationSettingData".to_string(), value: collection_replication_setting_data.into() });

        let result = self.invoke_method_with_job("ModifyReplicationSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `collection_recovery_point` -  (Msvm_CollectionRecoveryPoint)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn initiate_failover(&self, collection: CIM_CollectionOfMSEs, collection_recovery_point: Msvm_CollectionRecoveryPoint, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "CollectionRecoveryPoint".to_string(), value: collection_recovery_point.into() });

        let result = self.invoke_method_with_job("InitiateFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn revert_failover(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("RevertFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn commit_failover(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("CommitFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn initiate_planned_failover(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("InitiatePlannedFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn revert_planned_failover(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("RevertPlannedFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_replication_relationship(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("RemoveReplicationRelationship", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn suspend_replication(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("SuspendReplication", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn resume_replication(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("ResumeReplication", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn prepare_for_reverse_replication(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("PrepareForReverseReplication", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `collection_replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn reverse_replication_relationship(&self, collection: CIM_CollectionOfMSEs, collection_replication_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "CollectionReplicationSettingData".to_string(), value: collection_replication_setting_data.into() });

        let result = self.invoke_method_with_job("ReverseReplicationRelationship", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `include_suspended` -  (bool)
    /// * `start_time` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn resynchronize(&self, collection: CIM_CollectionOfMSEs, start_time: &String, include_suspended: bool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });
        args.push(MethodParameter { name: "IncludeSuspended".to_string(), value: include_suspended.into() });

        let result = self.invoke_method_with_job("Resynchronize", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)
    /// * `collection_recovery_point` -  (Msvm_CollectionRecoveryPoint)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `resulting_collection` -  (CIM_CollectionOfMSEs)
    /// * `return_value` -  (u32)
    pub fn test_replica_collection(&self, collection: CIM_CollectionOfMSEs, collection_recovery_point: Msvm_CollectionRecoveryPoint, resulting_collection: &mut CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });
        args.push(MethodParameter { name: "CollectionRecoveryPoint".to_string(), value: collection_recovery_point.into() });

        let result = self.invoke_method_with_job("TestReplicaCollection", &args)?;
        let job = result.get_value("Job")?;
        let resulting_collection = result.get_value("ResultingCollection")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn cancel_initial_replication(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("CancelInitialReplication", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn cancel_resynchronize(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("CancelResynchronize", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn revert_test_replica_collection(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("RevertTestReplicaCollection", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn cancel_update_disk_set(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("CancelUpdateDiskSet", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `replication_health_issues` -  (String[])
    /// * `replication_statistics` -  (String)
    /// * `return_value` -  (u32)
    pub fn get_replication_statistics(&self, collection: CIM_CollectionOfMSEs, replication_statistics: &mut String, replication_health_issues: &mut Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("GetReplicationStatistics", &args)?;
        let job = result.get_value("Job")?;
        let replication_health_issues = result.get_value("ReplicationHealthIssues")?;
        let replication_statistics = result.get_value("ReplicationStatistics")?;
        Ok(result.return_value)

    }


/// 

    /// * `collection` -  (CIM_CollectionOfMSEs)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn reset_replication_statistics(&self, collection: CIM_CollectionOfMSEs, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Collection".to_string(), value: collection.into() });

        let result = self.invoke_method_with_job("ResetReplicationStatistics", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_CollectionReplicationService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

