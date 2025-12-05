// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ReplicationService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ReplicationService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_ReplicationService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_service_settings(&self, setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SettingData".to_string(), value: setting_data.into() });

        let result = self.invoke_method_with_job("ModifyServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `authorization_entry` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn add_authorization_entry(&self, authorization_entry: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AuthorizationEntry".to_string(), value: authorization_entry.into() });

        let result = self.invoke_method_with_job("AddAuthorizationEntry", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `authorization_entry` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_authorization_entry(&self, authorization_entry: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AuthorizationEntry".to_string(), value: authorization_entry.into() });

        let result = self.invoke_method_with_job("ModifyAuthorizationEntry", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `allowed_primary_host_system` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_authorization_entry(&self, allowed_primary_host_system: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AllowedPrimaryHostSystem".to_string(), value: allowed_primary_host_system.into() });

        let result = self.invoke_method_with_job("RemoveAuthorizationEntry", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `authentication_type` -  (u16)
    /// * `bypass_proxy_server` -  (bool)
    /// * `certificate_thumb_print` -  (String)
    /// * `recovery_connection_point` -  (String)
    /// * `recovery_server_port_number` -  (u16)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn test_replication_connection(&self, recovery_connection_point: &String, recovery_server_port_number: u16, authentication_type: u16, certificate_thumb_print: &String, bypass_proxy_server: bool, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RecoveryConnectionPoint".to_string(), value: recovery_connection_point.into() });
        args.push(MethodParameter { name: "RecoveryServerPortNumber".to_string(), value: recovery_server_port_number.into() });
        args.push(MethodParameter { name: "AuthenticationType".to_string(), value: authentication_type.into() });
        args.push(MethodParameter { name: "CertificateThumbPrint".to_string(), value: certificate_thumb_print.into() });
        args.push(MethodParameter { name: "BypassProxyServer".to_string(), value: bypass_proxy_server.into() });

        let result = self.invoke_method_with_job("TestReplicationConnection", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn create_replication_relationship(&self, computer_system: CIM_ComputerSystem, replication_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationSettingData".to_string(), value: replication_setting_data.into() });

        let result = self.invoke_method_with_job("CreateReplicationRelationship", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_replication_settings(&self, computer_system: CIM_ComputerSystem, replication_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationSettingData".to_string(), value: replication_setting_data.into() });

        let result = self.invoke_method_with_job("ModifyReplicationSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_replication_relationship(&self, computer_system: CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method_with_job("RemoveReplicationRelationship", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_relationship` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_replication_relationship_ex(&self, computer_system: CIM_ComputerSystem, replication_relationship: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationRelationship".to_string(), value: replication_relationship.into() });

        let result = self.invoke_method_with_job("RemoveReplicationRelationshipEx", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `initial_replication_export_location` -  (String)
    /// * `initial_replication_type` -  (ReplicationService_InitialReplicationType)
    /// * `start_time` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn start_replication(&self, computer_system: CIM_ComputerSystem, initial_replication_type: ReplicationService_InitialReplicationType, initial_replication_export_location: &String, start_time: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "InitialReplicationType".to_string(), value: initial_replication_type.into() });
        args.push(MethodParameter { name: "InitialReplicationExportLocation".to_string(), value: initial_replication_export_location.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });

        let result = self.invoke_method_with_job("StartReplication", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `initial_replication_import_location` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn import_initial_replica(&self, computer_system: CIM_ComputerSystem, initial_replication_import_location: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "InitialReplicationImportLocation".to_string(), value: initial_replication_import_location.into() });

        let result = self.invoke_method_with_job("ImportInitialReplica", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn reverse_replication_relationship(&self, computer_system: CIM_ComputerSystem, replication_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationSettingData".to_string(), value: replication_setting_data.into() });

        let result = self.invoke_method_with_job("ReverseReplicationRelationship", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `snapshot_setting_data` -  (CIM_VirtualSystemSettingData)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn initiate_failover(&self, computer_system: CIM_ComputerSystem, snapshot_setting_data: CIM_VirtualSystemSettingData, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "SnapshotSettingData".to_string(), value: snapshot_setting_data.into() });

        let result = self.invoke_method_with_job("InitiateFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn revert_failover(&self, computer_system: CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method_with_job("RevertFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn commit_failover(&self, computer_system: CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method_with_job("CommitFailover", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `snapshot_setting_data` -  (CIM_VirtualSystemSettingData)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `resulting_system` -  (CIM_ComputerSystem)
    /// * `return_value` -  (u32)
    pub fn test_replica_system(&self, computer_system: CIM_ComputerSystem, snapshot_setting_data: CIM_VirtualSystemSettingData, resulting_system: &mut CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "SnapshotSettingData".to_string(), value: snapshot_setting_data.into() });

        let result = self.invoke_method_with_job("TestReplicaSystem", &args)?;
        let job = result.get_value("Job")?;
        let resulting_system = result.get_value("ResultingSystem")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `start_time` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn resynchronize(&self, computer_system: CIM_ComputerSystem, start_time: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "StartTime".to_string(), value: start_time.into() });

        let result = self.invoke_method_with_job("Resynchronize", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `network_settings` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_failover_network_adapter_settings(&self, computer_system: CIM_ComputerSystem, network_settings: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "NetworkSettings".to_string(), value: network_settings.into() });

        let result = self.invoke_method_with_job("SetFailoverNetworkAdapterSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `replication_health_issues` -  (String[])
    /// * `replication_statistics` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_replication_statistics(&self, computer_system: CIM_ComputerSystem, replication_statistics: &mut Vec<String>, replication_health_issues: &mut Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method_with_job("GetReplicationStatistics", &args)?;
        let job = result.get_value("Job")?;
        let replication_health_issues = result.get_value("ReplicationHealthIssues")?;
        let replication_statistics = result.get_value("ReplicationStatistics")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_relationship` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `replication_health_issues` -  (String[])
    /// * `replication_statistics` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_replication_statistics_ex(&self, computer_system: CIM_ComputerSystem, replication_relationship: &String, replication_statistics: &mut Vec<String>, replication_health_issues: &mut Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationRelationship".to_string(), value: replication_relationship.into() });

        let result = self.invoke_method_with_job("GetReplicationStatisticsEx", &args)?;
        let job = result.get_value("Job")?;
        let replication_health_issues = result.get_value("ReplicationHealthIssues")?;
        let replication_statistics = result.get_value("ReplicationStatistics")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn reset_replication_statistics(&self, computer_system: CIM_ComputerSystem, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method_with_job("ResetReplicationStatistics", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_relationship` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn reset_replication_statistics_ex(&self, computer_system: CIM_ComputerSystem, replication_relationship: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationRelationship".to_string(), value: replication_relationship.into() });

        let result = self.invoke_method_with_job("ResetReplicationStatisticsEx", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `authorization_entry` -  (String)
    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_authorization_entry(&self, computer_system: CIM_ComputerSystem, authorization_entry: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "AuthorizationEntry".to_string(), value: authorization_entry.into() });

        let result = self.invoke_method_with_job("SetAuthorizationEntry", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `encoded_certificates` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_system_certificates(&self, encoded_certificates: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSystemCertificates", &[])?;
        let encoded_certificates = result.get_value("EncodedCertificates")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `replication_relationship` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn change_replication_mode_to_primary(&self, computer_system: CIM_ComputerSystem, replication_relationship: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationRelationship".to_string(), value: replication_relationship.into() });

        let result = self.invoke_method_with_job("ChangeReplicationModeToPrimary", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `recovery_point_identifier` -  (String)
    /// * `replication_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn initiate_failback(&self, computer_system: CIM_ComputerSystem, replication_setting_data: &String, recovery_point_identifier: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "ReplicationSettingData".to_string(), value: replication_setting_data.into() });
        args.push(MethodParameter { name: "RecoveryPointIdentifier".to_string(), value: recovery_point_identifier.into() });

        let result = self.invoke_method_with_job("InitiateFailback", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_ReplicationService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Vec<Msvm_ComputerSystem>, WmiError> {
        self.get_all_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_ReplicationServiceSettingData object(s)
    pub fn get_related__replication_service_setting_data(&self) -> Result<Msvm_ReplicationServiceSettingData, WmiError> {
        self.get_related("Msvm_ReplicationServiceSettingData")
    }

}

