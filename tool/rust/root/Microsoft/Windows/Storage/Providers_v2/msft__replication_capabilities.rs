// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ReplicationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ReplicationCapabilities {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// Default value for recovery point
    #[serde(rename = "DefaultRecoveryPointObjective")]
    pub default_recovery_point_objective: Option<u32>,

/// Enumeration indicating what operations will be executed as asynchronous jobs. If an operation is included in both this and SupportedSynchronousActions properties then the underlying implementation is indicating that it may or may not create a job.
/// Note: the following methods are not supported asynchronously, hence the gap between 11 and 19: 
/// 	 - CreateGroup 
/// 	 - DeleteGroup 
/// 	 - AddMembers 
/// 	 - RemoveMembers 
/// 	 - AddReplicationEntity 
/// 	 - AddServiceAccessPoint 
/// 	 - AddSharedSecret.
    #[serde(rename = "SupportedAsynchronousActions")]
    pub supported_asynchronous_actions: Vec<ReplicationCapabilities_SupportedAsynchronousActions>,

/// An array of supported features of partition objects for replication.
    #[serde(rename = "SupportedLogVolumeFeatures")]
    pub supported_log_volume_features: Vec<ReplicationCapabilities_SupportedLogVolumeFeatures>,

/// Maximum log size in bytes supported for replication.
    #[serde(rename = "SupportedMaximumLogSize")]
    pub supported_maximum_log_size: Option<u64>,

/// Minimum log size in bytes supported for replication.
    #[serde(rename = "SupportedMinimumLogSize")]
    pub supported_minimum_log_size: Option<u64>,

/// Enumeration indicating the supported object types associated with these replication capabilities.
    #[serde(rename = "SupportedObjectTypes")]
    pub supported_object_types: Vec<ReplicationCapabilities_SupportedObjectTypes>,

/// An array of supported features of partition objects for replication.
    #[serde(rename = "SupportedReplicatedPartitionFeatures")]
    pub supported_replicated_partition_features: Vec<ReplicationCapabilities_SupportedReplicatedPartitionFeatures>,

/// Enumeration indicating the supported SyncType/Mode/Local-or-Remote combinations.
    #[serde(rename = "SupportedReplicationTypes")]
    pub supported_replication_types: Vec<ReplicationCapabilities_SupportedReplicationTypes>,

/// Enumeration indicating what operations will be executed synchronously -- without the creation of a job. If an operation is included in both this property and SupportedAsynchronousActions then the underlying implementation is indicating that it may or may not create a job.
/// Note: the following methods are not supported asynchronously: 
/// 	 - CreateGroup 
/// 	 - DeleteGroup 
/// 	 - AddMembers 
/// 	 - RemoveMembers 
/// 	 - AddReplicationEntity 
/// 	 - AddServiceAccessPoint 
/// 	 - AddSharedSecret.
    #[serde(rename = "SupportedSynchronousActions")]
    pub supported_synchronous_actions: Vec<ReplicationCapabilities_SupportedSynchronousActions>,

/// Indicates if CreateReplicationShip operation is supported
    #[serde(rename = "SupportsCreateReplicationRelationshipMethod")]
    pub supports_create_replication_relationship_method: Option<bool>,

/// Indicates if empty Replicaiotn Groups are allowed
    #[serde(rename = "SupportsEmptyReplicationGroup")]
    pub supports_empty_replication_group: Option<bool>,

/// Indicates if this is a fully discovered model
    #[serde(rename = "SupportsFullDiscovery")]
    pub supports_full_discovery: Option<bool>,

/// Indicates if Replication Groups is supported
    #[serde(rename = "SupportsReplicationGroup")]
    pub supports_replication_group: Option<bool>,
}

impl MSFT_ReplicationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            default_recovery_point_objective: None,
            supported_asynchronous_actions: Vec::new(),
            supported_log_volume_features: Vec::new(),
            supported_maximum_log_size: None,
            supported_minimum_log_size: None,
            supported_object_types: Vec::new(),
            supported_replicated_partition_features: Vec::new(),
            supported_replication_types: Vec::new(),
            supported_synchronous_actions: Vec::new(),
            supports_create_replication_relationship_method: None,
            supports_empty_replication_group: None,
            supports_full_discovery: None,
            supports_replication_group: None,
        }
    }


    /// Sets the value of DefaultRecoveryPointObjective
    pub fn set_default_recovery_point_objective(&mut self, value: u32) {
        self.default_recovery_point_objective = Some(value);
    }

    /// Gets the value of DefaultRecoveryPointObjective
    pub fn get_default_recovery_point_objective(&self) -> Option<&u32> {
        self.default_recovery_point_objective.as_ref()
    }

    /// Sets the value of SupportedAsynchronousActions
    pub fn set_supported_asynchronous_actions(&mut self, value: Vec<ReplicationCapabilities_SupportedAsynchronousActions>) {
        self.supported_asynchronous_actions = value;
    }

    /// Gets the value of SupportedAsynchronousActions
    pub fn get_supported_asynchronous_actions(&self) -> &Vec<ReplicationCapabilities_SupportedAsynchronousActions> {
        &self.supported_asynchronous_actions
    }

    /// Sets the value of SupportedLogVolumeFeatures
    pub fn set_supported_log_volume_features(&mut self, value: Vec<ReplicationCapabilities_SupportedLogVolumeFeatures>) {
        self.supported_log_volume_features = value;
    }

    /// Gets the value of SupportedLogVolumeFeatures
    pub fn get_supported_log_volume_features(&self) -> &Vec<ReplicationCapabilities_SupportedLogVolumeFeatures> {
        &self.supported_log_volume_features
    }

    /// Sets the value of SupportedMaximumLogSize
    pub fn set_supported_maximum_log_size(&mut self, value: u64) {
        self.supported_maximum_log_size = Some(value);
    }

    /// Gets the value of SupportedMaximumLogSize
    pub fn get_supported_maximum_log_size(&self) -> Option<&u64> {
        self.supported_maximum_log_size.as_ref()
    }

    /// Sets the value of SupportedMinimumLogSize
    pub fn set_supported_minimum_log_size(&mut self, value: u64) {
        self.supported_minimum_log_size = Some(value);
    }

    /// Gets the value of SupportedMinimumLogSize
    pub fn get_supported_minimum_log_size(&self) -> Option<&u64> {
        self.supported_minimum_log_size.as_ref()
    }

    /// Sets the value of SupportedObjectTypes
    pub fn set_supported_object_types(&mut self, value: Vec<ReplicationCapabilities_SupportedObjectTypes>) {
        self.supported_object_types = value;
    }

    /// Gets the value of SupportedObjectTypes
    pub fn get_supported_object_types(&self) -> &Vec<ReplicationCapabilities_SupportedObjectTypes> {
        &self.supported_object_types
    }

    /// Sets the value of SupportedReplicatedPartitionFeatures
    pub fn set_supported_replicated_partition_features(&mut self, value: Vec<ReplicationCapabilities_SupportedReplicatedPartitionFeatures>) {
        self.supported_replicated_partition_features = value;
    }

    /// Gets the value of SupportedReplicatedPartitionFeatures
    pub fn get_supported_replicated_partition_features(&self) -> &Vec<ReplicationCapabilities_SupportedReplicatedPartitionFeatures> {
        &self.supported_replicated_partition_features
    }

    /// Sets the value of SupportedReplicationTypes
    pub fn set_supported_replication_types(&mut self, value: Vec<ReplicationCapabilities_SupportedReplicationTypes>) {
        self.supported_replication_types = value;
    }

    /// Gets the value of SupportedReplicationTypes
    pub fn get_supported_replication_types(&self) -> &Vec<ReplicationCapabilities_SupportedReplicationTypes> {
        &self.supported_replication_types
    }

    /// Sets the value of SupportedSynchronousActions
    pub fn set_supported_synchronous_actions(&mut self, value: Vec<ReplicationCapabilities_SupportedSynchronousActions>) {
        self.supported_synchronous_actions = value;
    }

    /// Gets the value of SupportedSynchronousActions
    pub fn get_supported_synchronous_actions(&self) -> &Vec<ReplicationCapabilities_SupportedSynchronousActions> {
        &self.supported_synchronous_actions
    }

    /// Sets the value of SupportsCreateReplicationRelationshipMethod
    pub fn set_supports_create_replication_relationship_method(&mut self, value: bool) {
        self.supports_create_replication_relationship_method = Some(value);
    }

    /// Gets the value of SupportsCreateReplicationRelationshipMethod
    pub fn get_supports_create_replication_relationship_method(&self) -> Option<&bool> {
        self.supports_create_replication_relationship_method.as_ref()
    }

    /// Sets the value of SupportsEmptyReplicationGroup
    pub fn set_supports_empty_replication_group(&mut self, value: bool) {
        self.supports_empty_replication_group = Some(value);
    }

    /// Gets the value of SupportsEmptyReplicationGroup
    pub fn get_supports_empty_replication_group(&self) -> Option<&bool> {
        self.supports_empty_replication_group.as_ref()
    }

    /// Sets the value of SupportsFullDiscovery
    pub fn set_supports_full_discovery(&mut self, value: bool) {
        self.supports_full_discovery = Some(value);
    }

    /// Gets the value of SupportsFullDiscovery
    pub fn get_supports_full_discovery(&self) -> Option<&bool> {
        self.supports_full_discovery.as_ref()
    }

    /// Sets the value of SupportsReplicationGroup
    pub fn set_supports_replication_group(&mut self, value: bool) {
        self.supports_replication_group = Some(value);
    }

    /// Gets the value of SupportsReplicationGroup
    pub fn get_supports_replication_group(&self) -> Option<&bool> {
        self.supports_replication_group.as_ref()
    }

/// 

    /// * `replication_type` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_operations` -  (u16[])
    pub fn get_supported_operations(&self, replication_type: u16, supported_operations: &mut Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetSupportedOperations", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_operations = result.get_value("SupportedOperations")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_type` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_group_operations` -  (u16[])
    pub fn get_supported_group_operations(&self, replication_type: u16, supported_group_operations: &mut Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetSupportedGroupOperations", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_group_operations = result.get_value("SupportedGroupOperations")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_type` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `features` -  (u16[])
    /// * `return_value` -  (u32)
    pub fn get_supported_features(&self, replication_type: u16, features: &mut Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetSupportedFeatures", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let features = result.get_value("Features")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_type` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `group_features` -  (u16[])
    /// * `return_value` -  (u32)
    pub fn get_supported_group_features(&self, replication_type: u16, group_features: &mut Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetSupportedGroupFeatures", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let group_features = result.get_value("GroupFeatures")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_type` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_copy_states` -  (u16[])
    pub fn get_supported_copy_states(&self, replication_type: u16, supported_copy_states: &mut Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetSupportedCopyStates", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_copy_states = result.get_value("SupportedCopyStates")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_type` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_copy_states` -  (u16[])
    pub fn get_supported_group_copy_states(&self, replication_type: u16, supported_copy_states: &mut Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetSupportedGroupCopyStates", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_copy_states = result.get_value("SupportedCopyStates")?;
        Ok(result.return_value)

    }


/// 

    /// * `replication_type` -  (u16)

    /// * `default_recovery_point` -  (u32)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `recovery_point_indicator` -  (u16)
    /// * `recovery_point_values` -  (u32[])
    /// * `return_value` -  (u32)
    pub fn get_recovery_point_data(&self, replication_type: u16, default_recovery_point: &mut u32, recovery_point_values: &mut Vec<u32>, recovery_point_indicator: &mut u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReplicationType".to_string(), value: replication_type.into() });

        let result = self.invoke_method("GetRecoveryPointData", &args)?;
        let default_recovery_point = result.get_value("DefaultRecoveryPoint")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let recovery_point_indicator = result.get_value("RecoveryPointIndicator")?;
        let recovery_point_values = result.get_value("RecoveryPointValues")?;
        Ok(result.return_value)

    }

}

