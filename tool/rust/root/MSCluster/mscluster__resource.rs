// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Resource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Resource {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "CoreResource")]
    pub core_resource: Option<bool>,

/// 
    #[serde(rename = "CryptoCheckpoints")]
    pub crypto_checkpoints: Vec<String>,

/// 
    #[serde(rename = "DeadlockTimeout")]
    pub deadlock_timeout: Option<u32>,

/// 
    #[serde(rename = "DeleteRequiresAllNodes")]
    pub delete_requires_all_nodes: Option<bool>,

/// 
    #[serde(rename = "EmbeddedFailureAction")]
    pub embedded_failure_action: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IsAlivePollInterval")]
    pub is_alive_poll_interval: Option<u32>,

/// 
    #[serde(rename = "IsClusterSharedVolume")]
    pub is_cluster_shared_volume: Option<bool>,

/// 
    #[serde(rename = "LastOperationStatusCode")]
    pub last_operation_status_code: Option<u64>,

/// 
    #[serde(rename = "LocalQuorumCapable")]
    pub local_quorum_capable: Option<bool>,

/// 
    #[serde(rename = "LooksAlivePollInterval")]
    pub looks_alive_poll_interval: Option<u32>,

/// 
    #[serde(rename = "MonitorProcessId")]
    pub monitor_process_id: Option<u32>,

/// 
    #[serde(rename = "OwnerGroup")]
    pub owner_group: Option<String>,

/// 
    #[serde(rename = "OwnerNode")]
    pub owner_node: Option<String>,

/// 
    #[serde(rename = "PendingTimeout")]
    pub pending_timeout: Option<u32>,

/// 
    #[serde(rename = "PersistentState")]
    pub persistent_state: Option<bool>,

/// 
    #[serde(rename = "PrivateProperties")]
    pub private_properties: Option<MSCluster_Property>,

/// 
    #[serde(rename = "QuorumCapable")]
    pub quorum_capable: Option<bool>,

/// 
    #[serde(rename = "RegistryCheckpoints")]
    pub registry_checkpoints: Vec<String>,

/// 
    #[serde(rename = "RequiredDependencyClasses")]
    pub required_dependency_classes: Vec<u32>,

/// 
    #[serde(rename = "RequiredDependencyTypes")]
    pub required_dependency_types: Vec<String>,

/// 
    #[serde(rename = "ResourceClass")]
    pub resource_class: Option<u32>,

/// 
    #[serde(rename = "ResourceSpecificData1")]
    pub resource_specific_data1: Option<u64>,

/// 
    #[serde(rename = "ResourceSpecificData2")]
    pub resource_specific_data2: Option<u64>,

/// 
    #[serde(rename = "ResourceSpecificStatus")]
    pub resource_specific_status: Option<String>,

/// 
    #[serde(rename = "RestartAction")]
    pub restart_action: Option<u32>,

/// 
    #[serde(rename = "RestartDelay")]
    pub restart_delay: Option<u32>,

/// 
    #[serde(rename = "RestartPeriod")]
    pub restart_period: Option<u32>,

/// 
    #[serde(rename = "RestartThreshold")]
    pub restart_threshold: Option<u32>,

/// 
    #[serde(rename = "RetryPeriodOnFailure")]
    pub retry_period_on_failure: Option<u32>,

/// 
    #[serde(rename = "SeparateMonitor")]
    pub separate_monitor: Option<bool>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "StatusInformation")]
    pub status_information: Option<u64>,

/// 
    #[serde(rename = "Subclass")]
    pub subclass: Option<u32>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl MSCluster_Resource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            core_resource: None,
            crypto_checkpoints: Vec::new(),
            deadlock_timeout: None,
            delete_requires_all_nodes: None,
            embedded_failure_action: None,
            id: None,
            is_alive_poll_interval: None,
            is_cluster_shared_volume: None,
            last_operation_status_code: None,
            local_quorum_capable: None,
            looks_alive_poll_interval: None,
            monitor_process_id: None,
            owner_group: None,
            owner_node: None,
            pending_timeout: None,
            persistent_state: None,
            private_properties: None,
            quorum_capable: None,
            registry_checkpoints: Vec::new(),
            required_dependency_classes: Vec::new(),
            required_dependency_types: Vec::new(),
            resource_class: None,
            resource_specific_data1: None,
            resource_specific_data2: None,
            resource_specific_status: None,
            restart_action: None,
            restart_delay: None,
            restart_period: None,
            restart_threshold: None,
            retry_period_on_failure: None,
            separate_monitor: None,
            state: None,
            status_information: None,
            subclass: None,
            type: None,
        }
    }


    /// Sets the value of CoreResource
    pub fn set_core_resource(&mut self, value: bool) {
        self.core_resource = Some(value);
    }

    /// Gets the value of CoreResource
    pub fn get_core_resource(&self) -> Option<&bool> {
        self.core_resource.as_ref()
    }

    /// Sets the value of CryptoCheckpoints
    pub fn set_crypto_checkpoints(&mut self, value: Vec<String>) {
        self.crypto_checkpoints = value;
    }

    /// Gets the value of CryptoCheckpoints
    pub fn get_crypto_checkpoints(&self) -> &Vec<String> {
        &self.crypto_checkpoints
    }

    /// Sets the value of DeadlockTimeout
    pub fn set_deadlock_timeout(&mut self, value: u32) {
        self.deadlock_timeout = Some(value);
    }

    /// Gets the value of DeadlockTimeout
    pub fn get_deadlock_timeout(&self) -> Option<&u32> {
        self.deadlock_timeout.as_ref()
    }

    /// Sets the value of DeleteRequiresAllNodes
    pub fn set_delete_requires_all_nodes(&mut self, value: bool) {
        self.delete_requires_all_nodes = Some(value);
    }

    /// Gets the value of DeleteRequiresAllNodes
    pub fn get_delete_requires_all_nodes(&self) -> Option<&bool> {
        self.delete_requires_all_nodes.as_ref()
    }

    /// Sets the value of EmbeddedFailureAction
    pub fn set_embedded_failure_action(&mut self, value: u32) {
        self.embedded_failure_action = Some(value);
    }

    /// Gets the value of EmbeddedFailureAction
    pub fn get_embedded_failure_action(&self) -> Option<&u32> {
        self.embedded_failure_action.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IsAlivePollInterval
    pub fn set_is_alive_poll_interval(&mut self, value: u32) {
        self.is_alive_poll_interval = Some(value);
    }

    /// Gets the value of IsAlivePollInterval
    pub fn get_is_alive_poll_interval(&self) -> Option<&u32> {
        self.is_alive_poll_interval.as_ref()
    }

    /// Sets the value of IsClusterSharedVolume
    pub fn set_is_cluster_shared_volume(&mut self, value: bool) {
        self.is_cluster_shared_volume = Some(value);
    }

    /// Gets the value of IsClusterSharedVolume
    pub fn get_is_cluster_shared_volume(&self) -> Option<&bool> {
        self.is_cluster_shared_volume.as_ref()
    }

    /// Sets the value of LastOperationStatusCode
    pub fn set_last_operation_status_code(&mut self, value: u64) {
        self.last_operation_status_code = Some(value);
    }

    /// Gets the value of LastOperationStatusCode
    pub fn get_last_operation_status_code(&self) -> Option<&u64> {
        self.last_operation_status_code.as_ref()
    }

    /// Sets the value of LocalQuorumCapable
    pub fn set_local_quorum_capable(&mut self, value: bool) {
        self.local_quorum_capable = Some(value);
    }

    /// Gets the value of LocalQuorumCapable
    pub fn get_local_quorum_capable(&self) -> Option<&bool> {
        self.local_quorum_capable.as_ref()
    }

    /// Sets the value of LooksAlivePollInterval
    pub fn set_looks_alive_poll_interval(&mut self, value: u32) {
        self.looks_alive_poll_interval = Some(value);
    }

    /// Gets the value of LooksAlivePollInterval
    pub fn get_looks_alive_poll_interval(&self) -> Option<&u32> {
        self.looks_alive_poll_interval.as_ref()
    }

    /// Sets the value of MonitorProcessId
    pub fn set_monitor_process_id(&mut self, value: u32) {
        self.monitor_process_id = Some(value);
    }

    /// Gets the value of MonitorProcessId
    pub fn get_monitor_process_id(&self) -> Option<&u32> {
        self.monitor_process_id.as_ref()
    }

    /// Sets the value of OwnerGroup
    pub fn set_owner_group(&mut self, value: String) {
        self.owner_group = Some(value);
    }

    /// Gets the value of OwnerGroup
    pub fn get_owner_group(&self) -> Option<&String> {
        self.owner_group.as_ref()
    }

    /// Sets the value of OwnerNode
    pub fn set_owner_node(&mut self, value: String) {
        self.owner_node = Some(value);
    }

    /// Gets the value of OwnerNode
    pub fn get_owner_node(&self) -> Option<&String> {
        self.owner_node.as_ref()
    }

    /// Sets the value of PendingTimeout
    pub fn set_pending_timeout(&mut self, value: u32) {
        self.pending_timeout = Some(value);
    }

    /// Gets the value of PendingTimeout
    pub fn get_pending_timeout(&self) -> Option<&u32> {
        self.pending_timeout.as_ref()
    }

    /// Sets the value of PersistentState
    pub fn set_persistent_state(&mut self, value: bool) {
        self.persistent_state = Some(value);
    }

    /// Gets the value of PersistentState
    pub fn get_persistent_state(&self) -> Option<&bool> {
        self.persistent_state.as_ref()
    }

    /// Sets the value of PrivateProperties
    pub fn set_private_properties(&mut self, value: MSCluster_Property) {
        self.private_properties = Some(value);
    }

    /// Gets the value of PrivateProperties
    pub fn get_private_properties(&self) -> Option<&MSCluster_Property> {
        self.private_properties.as_ref()
    }

    /// Sets the value of QuorumCapable
    pub fn set_quorum_capable(&mut self, value: bool) {
        self.quorum_capable = Some(value);
    }

    /// Gets the value of QuorumCapable
    pub fn get_quorum_capable(&self) -> Option<&bool> {
        self.quorum_capable.as_ref()
    }

    /// Sets the value of RegistryCheckpoints
    pub fn set_registry_checkpoints(&mut self, value: Vec<String>) {
        self.registry_checkpoints = value;
    }

    /// Gets the value of RegistryCheckpoints
    pub fn get_registry_checkpoints(&self) -> &Vec<String> {
        &self.registry_checkpoints
    }

    /// Sets the value of RequiredDependencyClasses
    pub fn set_required_dependency_classes(&mut self, value: Vec<u32>) {
        self.required_dependency_classes = value;
    }

    /// Gets the value of RequiredDependencyClasses
    pub fn get_required_dependency_classes(&self) -> &Vec<u32> {
        &self.required_dependency_classes
    }

    /// Sets the value of RequiredDependencyTypes
    pub fn set_required_dependency_types(&mut self, value: Vec<String>) {
        self.required_dependency_types = value;
    }

    /// Gets the value of RequiredDependencyTypes
    pub fn get_required_dependency_types(&self) -> &Vec<String> {
        &self.required_dependency_types
    }

    /// Sets the value of ResourceClass
    pub fn set_resource_class(&mut self, value: u32) {
        self.resource_class = Some(value);
    }

    /// Gets the value of ResourceClass
    pub fn get_resource_class(&self) -> Option<&u32> {
        self.resource_class.as_ref()
    }

    /// Sets the value of ResourceSpecificData1
    pub fn set_resource_specific_data1(&mut self, value: u64) {
        self.resource_specific_data1 = Some(value);
    }

    /// Gets the value of ResourceSpecificData1
    pub fn get_resource_specific_data1(&self) -> Option<&u64> {
        self.resource_specific_data1.as_ref()
    }

    /// Sets the value of ResourceSpecificData2
    pub fn set_resource_specific_data2(&mut self, value: u64) {
        self.resource_specific_data2 = Some(value);
    }

    /// Gets the value of ResourceSpecificData2
    pub fn get_resource_specific_data2(&self) -> Option<&u64> {
        self.resource_specific_data2.as_ref()
    }

    /// Sets the value of ResourceSpecificStatus
    pub fn set_resource_specific_status(&mut self, value: String) {
        self.resource_specific_status = Some(value);
    }

    /// Gets the value of ResourceSpecificStatus
    pub fn get_resource_specific_status(&self) -> Option<&String> {
        self.resource_specific_status.as_ref()
    }

    /// Sets the value of RestartAction
    pub fn set_restart_action(&mut self, value: u32) {
        self.restart_action = Some(value);
    }

    /// Gets the value of RestartAction
    pub fn get_restart_action(&self) -> Option<&u32> {
        self.restart_action.as_ref()
    }

    /// Sets the value of RestartDelay
    pub fn set_restart_delay(&mut self, value: u32) {
        self.restart_delay = Some(value);
    }

    /// Gets the value of RestartDelay
    pub fn get_restart_delay(&self) -> Option<&u32> {
        self.restart_delay.as_ref()
    }

    /// Sets the value of RestartPeriod
    pub fn set_restart_period(&mut self, value: u32) {
        self.restart_period = Some(value);
    }

    /// Gets the value of RestartPeriod
    pub fn get_restart_period(&self) -> Option<&u32> {
        self.restart_period.as_ref()
    }

    /// Sets the value of RestartThreshold
    pub fn set_restart_threshold(&mut self, value: u32) {
        self.restart_threshold = Some(value);
    }

    /// Gets the value of RestartThreshold
    pub fn get_restart_threshold(&self) -> Option<&u32> {
        self.restart_threshold.as_ref()
    }

    /// Sets the value of RetryPeriodOnFailure
    pub fn set_retry_period_on_failure(&mut self, value: u32) {
        self.retry_period_on_failure = Some(value);
    }

    /// Gets the value of RetryPeriodOnFailure
    pub fn get_retry_period_on_failure(&self) -> Option<&u32> {
        self.retry_period_on_failure.as_ref()
    }

    /// Sets the value of SeparateMonitor
    pub fn set_separate_monitor(&mut self, value: bool) {
        self.separate_monitor = Some(value);
    }

    /// Gets the value of SeparateMonitor
    pub fn get_separate_monitor(&self) -> Option<&bool> {
        self.separate_monitor.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of StatusInformation
    pub fn set_status_information(&mut self, value: u64) {
        self.status_information = Some(value);
    }

    /// Gets the value of StatusInformation
    pub fn get_status_information(&self) -> Option<&u64> {
        self.status_information.as_ref()
    }

    /// Sets the value of Subclass
    pub fn set_subclass(&mut self, value: u32) {
        self.subclass = Some(value);
    }

    /// Gets the value of Subclass
    pub fn get_subclass(&self) -> Option<&u32> {
        self.subclass.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }

/// 

    /// * `group` -  (String)
    /// * `id` -  (String)
    /// * `resource_name` -  (String)
    /// * `resource_type` -  (String)
    /// * `separate_monitor` -  (bool)

    /// * `id` -  (String)
    pub fn create_resource(&self, group: &String, resource_name: &String, resource_type: &String, separate_monitor: bool, id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        args.push(MethodParameter { name: "ResourceName".to_string(), value: resource_name.into() });
        args.push(MethodParameter { name: "ResourceType".to_string(), value: resource_type.into() });
        args.push(MethodParameter { name: "SeparateMonitor".to_string(), value: separate_monitor.into() });

        let result = self.invoke_method("CreateResource", &args)?;
        let id = result.get_value("Id")?;
        Ok(result.return_value)

    }


/// 

    /// * `options` -  (u32)
    /// * `reason` -  (String)
    pub fn delete_resource(&self, options: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("DeleteResource", &args)

    }


/// 

    /// * `group` -  (String)
    /// * `reason` -  (String)
    pub fn move_to_new_group(&self, group: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Group".to_string(), value: group.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("MoveToNewGroup", &args)

    }


/// 

    /// * `resource` -  (String)
    pub fn add_dependency(&self, resource: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });
        self.invoke_method("AddDependency", &args)

    }


/// 

    /// * `resource` -  (String)
    pub fn remove_dependency(&self, resource: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Resource".to_string(), value: resource.into() });
        self.invoke_method("RemoveDependency", &args)

    }


/// 

    /// * `expression` -  (String)
    pub fn set_dependencies(&self, expression: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Expression".to_string(), value: expression.into() });
        self.invoke_method("SetDependencies", &args)

    }


/// 

    /// * `as_resource_ids` -  (bool)

    /// * `expression` -  (String)
    pub fn get_dependencies(&self, expression: &mut String, as_resource_ids: Option<bool>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = as_resource_ids {
            args.push(MethodParameter { name: "AsResourceIds".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetDependencies", &args)?;
        let expression = result.get_value("Expression")?;
        Ok(result.return_value)

    }


/// 

    /// * `reason` -  (String)
    /// * `time_out` -  (u32)
    pub fn bring_online(&self, time_out: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("BringOnline", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `parameters` -  (MSCluster_Property)
    /// * `reason` -  (String)
    /// * `time_out` -  (u32)
    pub fn take_offline(&self, time_out: u32, parameters: MSCluster_Property, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("TakeOffline", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `parameters` -  (u8[])
    /// * `reason` -  (String)
    /// * `time_out` -  (u32)
    pub fn take_offline_params(&self, time_out: u32, parameters: &Vec<u8>, flags: u32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TimeOut".to_string(), value: time_out.into() });
        args.push(MethodParameter { name: "Parameters".to_string(), value: parameters.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("TakeOfflineParams", &args)

    }


/// 

    /// * `new_name` -  (String)
    /// * `reason` -  (String)
    pub fn rename(&self, new_name: &String, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "newName".to_string(), value: new_name.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `reason` -  (String)
    pub fn fail_resource(&self, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }
        self.invoke_method("FailResource", &args)

    }


/// 

    /// * `checkpoint_name` -  (String)
    pub fn add_registry_checkpoint(&self, checkpoint_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CheckpointName".to_string(), value: checkpoint_name.into() });
        self.invoke_method("AddRegistryCheckpoint", &args)

    }


/// 

    /// * `checkpoint_name` -  (String)
    pub fn remove_registry_checkpoint(&self, checkpoint_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CheckpointName".to_string(), value: checkpoint_name.into() });
        self.invoke_method("RemoveRegistryCheckpoint", &args)

    }


/// 

    /// * `checkpoint_name` -  (String)
    pub fn add_crypto_checkpoint(&self, checkpoint_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CheckpointName".to_string(), value: checkpoint_name.into() });
        self.invoke_method("AddCryptoCheckpoint", &args)

    }


/// 

    /// * `checkpoint_name` -  (String)
    pub fn remove_crypto_checkpoint(&self, checkpoint_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CheckpointName".to_string(), value: checkpoint_name.into() });
        self.invoke_method("RemoveCryptoCheckpoint", &args)

    }


/// 
    pub fn renew_address(&self) -> Result<(), WmiError> {
        self.invoke_method("RenewAddress", &[])

    }


/// 
    pub fn release_address(&self) -> Result<(), WmiError> {
        self.invoke_method("ReleaseAddress", &[])

    }


/// 

    /// * `node_names` -  (String[])
    pub fn get_possible_owners(&self, node_names: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPossibleOwners", &[])?;
        let node_names = result.get_value("NodeNames")?;
        Ok(result.return_value)

    }


/// 

    /// * `node_name` -  (String)
    pub fn add_possible_owner(&self, node_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        self.invoke_method("AddPossibleOwner", &args)

    }


/// 

    /// * `node_name` -  (String)
    pub fn remove_possible_owner(&self, node_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NodeName".to_string(), value: node_name.into() });
        self.invoke_method("RemovePossibleOwner", &args)

    }


/// 
    pub fn update_virtual_machine(&self) -> Result<(), WmiError> {
        self.invoke_method("UpdateVirtualMachine", &[])

    }


/// 

    /// * `configuration_destination_path` -  (String)
    /// * `destination_paths` -  (String[])
    /// * `resource_destination_pools` -  (String[])
    /// * `snapshot_destination_path` -  (String)
    /// * `source_paths` -  (String[])
    /// * `swap_file_destination_path` -  (String)
    pub fn migrate_virtual_machine(&self, snapshot_destination_path: &String, configuration_destination_path: &String, swap_file_destination_path: &String, source_paths: &Vec<String>, destination_paths: &Vec<String>, resource_destination_pools: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SnapshotDestinationPath".to_string(), value: snapshot_destination_path.into() });
        args.push(MethodParameter { name: "ConfigurationDestinationPath".to_string(), value: configuration_destination_path.into() });
        args.push(MethodParameter { name: "SwapFileDestinationPath".to_string(), value: swap_file_destination_path.into() });
        args.push(MethodParameter { name: "SourcePaths".to_string(), value: source_paths.into() });
        args.push(MethodParameter { name: "DestinationPaths".to_string(), value: destination_paths.into() });
        args.push(MethodParameter { name: "ResourceDestinationPools".to_string(), value: resource_destination_pools.into() });
        self.invoke_method("MigrateVirtualMachine", &args)

    }


/// 

    /// * `storage_device` -  (MSCluster_AvailableDisk)
    pub fn attach_storage_device(&self, storage_device: MSCluster_AvailableDisk) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageDevice".to_string(), value: storage_device.into() });
        self.invoke_method("AttachStorageDevice", &args)

    }


/// 

    /// * `control_code` -  (i32)
    /// * `input_buffer` -  (u8[])
    /// * `reason` -  (String)

    /// * `output_buffer` -  (u8[])
    /// * `output_buffer_size` -  (i32)
    pub fn execute_resource_control(&self, control_code: i32, input_buffer: &Vec<u8>, output_buffer: &mut Vec<u8>, output_buffer_size: &mut i32, reason: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ControlCode".to_string(), value: control_code.into() });
        args.push(MethodParameter { name: "InputBuffer".to_string(), value: input_buffer.into() });
        if let Some(val) = reason {
            args.push(MethodParameter { name: "Reason".to_string(), value: val.into() });
        }

        let result = self.invoke_method("ExecuteResourceControl", &args)?;
        let output_buffer = result.get_value("OutputBuffer")?;
        let output_buffer_size = result.get_value("OutputBufferSize")?;
        Ok(result.return_value)

    }

}

