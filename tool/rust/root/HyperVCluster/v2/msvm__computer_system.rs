// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ComputerSystem {
    #[serde(flatten)]
    pub base: CIM_ComputerSystem,

/// 
    #[serde(rename = "EnhancedSessionModeState")]
    pub enhanced_session_mode_state: Option<u16>,

/// 
    #[serde(rename = "FailedOverReplicationType")]
    pub failed_over_replication_type: Option<u16>,

/// 
    #[serde(rename = "HwThreadsPerCoreRealized")]
    pub hw_threads_per_core_realized: Option<u32>,

/// 
    #[serde(rename = "LastApplicationConsistentReplicationTime")]
    pub last_application_consistent_replication_time: Option<String>,

/// 
    #[serde(rename = "LastReplicationTime")]
    pub last_replication_time: Option<String>,

/// 
    #[serde(rename = "LastReplicationType")]
    pub last_replication_type: Option<u16>,

/// 
    #[serde(rename = "LastSuccessfulBackupTime")]
    pub last_successful_backup_time: Option<String>,

/// 
    #[serde(rename = "ManagementVtlImageFileName")]
    pub management_vtl_image_file_name: Option<String>,

/// 
    #[serde(rename = "ManagementVtlImageVersion")]
    pub management_vtl_image_version: Option<String>,

/// 
    #[serde(rename = "NumberOfNumaNodes")]
    pub number_of_numa_nodes: Option<u16>,

/// 
    #[serde(rename = "OnTimeInMilliseconds")]
    pub on_time_in_milliseconds: Option<u64>,

/// 
    #[serde(rename = "ProcessID")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ReplicationHealth")]
    pub replication_health: Option<u16>,

/// 
    #[serde(rename = "ReplicationMode")]
    pub replication_mode: Option<u16>,

/// 
    #[serde(rename = "ReplicationState")]
    pub replication_state: Option<u16>,

/// 
    #[serde(rename = "TimeOfLastConfigurationChange")]
    pub time_of_last_configuration_change: Option<String>,
}

impl Msvm_ComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystem::new(),
            enhanced_session_mode_state: None,
            failed_over_replication_type: None,
            hw_threads_per_core_realized: None,
            last_application_consistent_replication_time: None,
            last_replication_time: None,
            last_replication_type: None,
            last_successful_backup_time: None,
            management_vtl_image_file_name: None,
            management_vtl_image_version: None,
            number_of_numa_nodes: None,
            on_time_in_milliseconds: None,
            process_id: None,
            replication_health: None,
            replication_mode: None,
            replication_state: None,
            time_of_last_configuration_change: None,
        }
    }


    /// Sets the value of EnhancedSessionModeState
    pub fn set_enhanced_session_mode_state(&mut self, value: u16) {
        self.enhanced_session_mode_state = Some(value);
    }

    /// Gets the value of EnhancedSessionModeState
    pub fn get_enhanced_session_mode_state(&self) -> Option<&u16> {
        self.enhanced_session_mode_state.as_ref()
    }

    /// Sets the value of FailedOverReplicationType
    pub fn set_failed_over_replication_type(&mut self, value: u16) {
        self.failed_over_replication_type = Some(value);
    }

    /// Gets the value of FailedOverReplicationType
    pub fn get_failed_over_replication_type(&self) -> Option<&u16> {
        self.failed_over_replication_type.as_ref()
    }

    /// Sets the value of HwThreadsPerCoreRealized
    pub fn set_hw_threads_per_core_realized(&mut self, value: u32) {
        self.hw_threads_per_core_realized = Some(value);
    }

    /// Gets the value of HwThreadsPerCoreRealized
    pub fn get_hw_threads_per_core_realized(&self) -> Option<&u32> {
        self.hw_threads_per_core_realized.as_ref()
    }

    /// Sets the value of LastApplicationConsistentReplicationTime
    pub fn set_last_application_consistent_replication_time(&mut self, value: String) {
        self.last_application_consistent_replication_time = Some(value);
    }

    /// Gets the value of LastApplicationConsistentReplicationTime
    pub fn get_last_application_consistent_replication_time(&self) -> Option<&String> {
        self.last_application_consistent_replication_time.as_ref()
    }

    /// Sets the value of LastReplicationTime
    pub fn set_last_replication_time(&mut self, value: String) {
        self.last_replication_time = Some(value);
    }

    /// Gets the value of LastReplicationTime
    pub fn get_last_replication_time(&self) -> Option<&String> {
        self.last_replication_time.as_ref()
    }

    /// Sets the value of LastReplicationType
    pub fn set_last_replication_type(&mut self, value: u16) {
        self.last_replication_type = Some(value);
    }

    /// Gets the value of LastReplicationType
    pub fn get_last_replication_type(&self) -> Option<&u16> {
        self.last_replication_type.as_ref()
    }

    /// Sets the value of LastSuccessfulBackupTime
    pub fn set_last_successful_backup_time(&mut self, value: String) {
        self.last_successful_backup_time = Some(value);
    }

    /// Gets the value of LastSuccessfulBackupTime
    pub fn get_last_successful_backup_time(&self) -> Option<&String> {
        self.last_successful_backup_time.as_ref()
    }

    /// Sets the value of ManagementVtlImageFileName
    pub fn set_management_vtl_image_file_name(&mut self, value: String) {
        self.management_vtl_image_file_name = Some(value);
    }

    /// Gets the value of ManagementVtlImageFileName
    pub fn get_management_vtl_image_file_name(&self) -> Option<&String> {
        self.management_vtl_image_file_name.as_ref()
    }

    /// Sets the value of ManagementVtlImageVersion
    pub fn set_management_vtl_image_version(&mut self, value: String) {
        self.management_vtl_image_version = Some(value);
    }

    /// Gets the value of ManagementVtlImageVersion
    pub fn get_management_vtl_image_version(&self) -> Option<&String> {
        self.management_vtl_image_version.as_ref()
    }

    /// Sets the value of NumberOfNumaNodes
    pub fn set_number_of_numa_nodes(&mut self, value: u16) {
        self.number_of_numa_nodes = Some(value);
    }

    /// Gets the value of NumberOfNumaNodes
    pub fn get_number_of_numa_nodes(&self) -> Option<&u16> {
        self.number_of_numa_nodes.as_ref()
    }

    /// Sets the value of OnTimeInMilliseconds
    pub fn set_on_time_in_milliseconds(&mut self, value: u64) {
        self.on_time_in_milliseconds = Some(value);
    }

    /// Gets the value of OnTimeInMilliseconds
    pub fn get_on_time_in_milliseconds(&self) -> Option<&u64> {
        self.on_time_in_milliseconds.as_ref()
    }

    /// Sets the value of ProcessID
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessID
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ReplicationHealth
    pub fn set_replication_health(&mut self, value: u16) {
        self.replication_health = Some(value);
    }

    /// Gets the value of ReplicationHealth
    pub fn get_replication_health(&self) -> Option<&u16> {
        self.replication_health.as_ref()
    }

    /// Sets the value of ReplicationMode
    pub fn set_replication_mode(&mut self, value: u16) {
        self.replication_mode = Some(value);
    }

    /// Gets the value of ReplicationMode
    pub fn get_replication_mode(&self) -> Option<&u16> {
        self.replication_mode.as_ref()
    }

    /// Sets the value of ReplicationState
    pub fn set_replication_state(&mut self, value: u16) {
        self.replication_state = Some(value);
    }

    /// Gets the value of ReplicationState
    pub fn get_replication_state(&self) -> Option<&u16> {
        self.replication_state.as_ref()
    }

    /// Sets the value of TimeOfLastConfigurationChange
    pub fn set_time_of_last_configuration_change(&mut self, value: String) {
        self.time_of_last_configuration_change = Some(value);
    }

    /// Gets the value of TimeOfLastConfigurationChange
    pub fn get_time_of_last_configuration_change(&self) -> Option<&String> {
        self.time_of_last_configuration_change.as_ref()
    }

/// 

    /// * `requested_state` -  (u16)
    /// * `timeout_period` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_replication_state_change(&self, requested_state: u16, job: &mut CIM_ConcreteJob, timeout_period: &Option<String>, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        if let Some(val) = timeout_period {
            args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: val.into() });
        }

        let result = self.invoke_method_with_job("RequestReplicationStateChange", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `job` - May contain a reference to the ConcreteJob created to track the status of the interrupt injection. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn inject_non_maskable_interrupt(&self, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {

        let result = self.invoke_method_with_job("InjectNonMaskableInterrupt", &[])?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `vtl` -  (u8)

    /// * `job` - May contain a reference to the ConcreteJob created to track the status of the interrupt injection. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn inject_non_maskable_interrupt_ex(&self, vtl: u8, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Vtl".to_string(), value: vtl.into() });

        let result = self.invoke_method_with_job("InjectNonMaskableInterruptEx", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_relationship` -  (String)
    /// * `requested_state` -  (u16)
    /// * `timeout_period` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_replication_state_change_ex(&self, replication_relationship: &String, requested_state: u16, job: &mut CIM_ConcreteJob, timeout_period: &Option<String>, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationRelationship".to_string(), value: replication_relationship.into() });
        args.push(MethodParameter { name: "RequestedState".to_string(), value: requested_state.into() });
        if let Some(val) = timeout_period {
            args.push(MethodParameter { name: "TimeoutPeriod".to_string(), value: val.into() });
        }

        let result = self.invoke_method_with_job("RequestReplicationStateChangeEx", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `restore_settings` - The restore setting requested for the element. This information will be placed into the RestoreSettings property of the instance if the return code of the RequestCustomRestore method is 0 ('Completed with No Error'), or 4096 (0x1000) ('Job Started').  (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn request_custom_restore(&self, restore_settings: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RestoreSettings".to_string(), value: restore_settings.into() });

        let result = self.invoke_method_with_job("RequestCustomRestore", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_ComputerSystem {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_RedundancySet object(s)
    pub fn get_related__redundancy_set(&self) -> Result<Msvm_RedundancySet, WmiError> {
        self.get_related("Msvm_RedundancySet")
    }

}

