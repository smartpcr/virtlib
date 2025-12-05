// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SummaryInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SummaryInformation {
    #[serde(flatten)]
    pub base: Msvm_SummaryInformationBase,

/// 
    #[serde(rename = "AllocatedGPU")]
    pub allocated_gpu: Option<String>,

/// 
    #[serde(rename = "ApplicationHealth")]
    pub application_health: Option<u16>,

/// 
    #[serde(rename = "AsynchronousTasks")]
    pub asynchronous_tasks: Vec<CIM_ConcreteJob>,

/// 
    #[serde(rename = "AvailableMemoryBuffer")]
    pub available_memory_buffer: Option<i32>,

/// 
    #[serde(rename = "GuestOperatingSystem")]
    pub guest_operating_system: Option<String>,

/// 
    #[serde(rename = "Heartbeat")]
    pub heartbeat: Option<u16>,

/// 
    #[serde(rename = "HypervisorPartitionId")]
    pub hypervisor_partition_id: Option<u64>,

/// 
    #[serde(rename = "IntegrationServicesVersionState")]
    pub integration_services_version_state: Option<u16>,

/// 
    #[serde(rename = "MemoryAvailable")]
    pub memory_available: Option<i32>,

/// 
    #[serde(rename = "MemorySpansPhysicalNumaNodes")]
    pub memory_spans_physical_numa_nodes: Option<bool>,

/// 
    #[serde(rename = "MemoryUsage")]
    pub memory_usage: Option<u64>,

/// 
    #[serde(rename = "ProcessorLoad")]
    pub processor_load: Option<u16>,

/// 
    #[serde(rename = "ProcessorLoadHistory")]
    pub processor_load_history: Vec<u16>,

/// 
    #[serde(rename = "ReplicationHealth")]
    pub replication_health: Option<u16>,

/// 
    #[serde(rename = "ReplicationHealthEx")]
    pub replication_health_ex: Vec<u16>,

/// 
    #[serde(rename = "ReplicationMode")]
    pub replication_mode: Option<u16>,

/// 
    #[serde(rename = "ReplicationProviderId")]
    pub replication_provider_id: Vec<String>,

/// 
    #[serde(rename = "ReplicationState")]
    pub replication_state: Option<u16>,

/// 
    #[serde(rename = "ReplicationStateEx")]
    pub replication_state_ex: Vec<u16>,

/// 
    #[serde(rename = "Shielded")]
    pub shielded: Option<bool>,

/// 
    #[serde(rename = "Snapshots")]
    pub snapshots: Vec<CIM_VirtualSystemSettingData>,

/// 
    #[serde(rename = "SwapFilesInUse")]
    pub swap_files_in_use: Option<bool>,

/// 
    #[serde(rename = "TestReplicaSystem")]
    pub test_replica_system: Option<CIM_ComputerSystem>,

/// 
    #[serde(rename = "ThumbnailImage")]
    pub thumbnail_image: Vec<u8>,

/// 
    #[serde(rename = "ThumbnailImageHeight")]
    pub thumbnail_image_height: Option<u16>,

/// 
    #[serde(rename = "ThumbnailImageWidth")]
    pub thumbnail_image_width: Option<u16>,
}

impl Msvm_SummaryInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_SummaryInformationBase::new(),
            allocated_gpu: None,
            application_health: None,
            asynchronous_tasks: Vec::new(),
            available_memory_buffer: None,
            guest_operating_system: None,
            heartbeat: None,
            hypervisor_partition_id: None,
            integration_services_version_state: None,
            memory_available: None,
            memory_spans_physical_numa_nodes: None,
            memory_usage: None,
            processor_load: None,
            processor_load_history: Vec::new(),
            replication_health: None,
            replication_health_ex: Vec::new(),
            replication_mode: None,
            replication_provider_id: Vec::new(),
            replication_state: None,
            replication_state_ex: Vec::new(),
            shielded: None,
            snapshots: Vec::new(),
            swap_files_in_use: None,
            test_replica_system: None,
            thumbnail_image: Vec::new(),
            thumbnail_image_height: None,
            thumbnail_image_width: None,
        }
    }


    /// Sets the value of AllocatedGPU
    pub fn set_allocated_gpu(&mut self, value: String) {
        self.allocated_gpu = Some(value);
    }

    /// Gets the value of AllocatedGPU
    pub fn get_allocated_gpu(&self) -> Option<&String> {
        self.allocated_gpu.as_ref()
    }

    /// Sets the value of ApplicationHealth
    pub fn set_application_health(&mut self, value: u16) {
        self.application_health = Some(value);
    }

    /// Gets the value of ApplicationHealth
    pub fn get_application_health(&self) -> Option<&u16> {
        self.application_health.as_ref()
    }

    /// Sets the value of AsynchronousTasks
    pub fn set_asynchronous_tasks(&mut self, value: Vec<CIM_ConcreteJob>) {
        self.asynchronous_tasks = value;
    }

    /// Gets the value of AsynchronousTasks
    pub fn get_asynchronous_tasks(&self) -> &Vec<CIM_ConcreteJob> {
        &self.asynchronous_tasks
    }

    /// Sets the value of AvailableMemoryBuffer
    pub fn set_available_memory_buffer(&mut self, value: i32) {
        self.available_memory_buffer = Some(value);
    }

    /// Gets the value of AvailableMemoryBuffer
    pub fn get_available_memory_buffer(&self) -> Option<&i32> {
        self.available_memory_buffer.as_ref()
    }

    /// Sets the value of GuestOperatingSystem
    pub fn set_guest_operating_system(&mut self, value: String) {
        self.guest_operating_system = Some(value);
    }

    /// Gets the value of GuestOperatingSystem
    pub fn get_guest_operating_system(&self) -> Option<&String> {
        self.guest_operating_system.as_ref()
    }

    /// Sets the value of Heartbeat
    pub fn set_heartbeat(&mut self, value: u16) {
        self.heartbeat = Some(value);
    }

    /// Gets the value of Heartbeat
    pub fn get_heartbeat(&self) -> Option<&u16> {
        self.heartbeat.as_ref()
    }

    /// Sets the value of HypervisorPartitionId
    pub fn set_hypervisor_partition_id(&mut self, value: u64) {
        self.hypervisor_partition_id = Some(value);
    }

    /// Gets the value of HypervisorPartitionId
    pub fn get_hypervisor_partition_id(&self) -> Option<&u64> {
        self.hypervisor_partition_id.as_ref()
    }

    /// Sets the value of IntegrationServicesVersionState
    pub fn set_integration_services_version_state(&mut self, value: u16) {
        self.integration_services_version_state = Some(value);
    }

    /// Gets the value of IntegrationServicesVersionState
    pub fn get_integration_services_version_state(&self) -> Option<&u16> {
        self.integration_services_version_state.as_ref()
    }

    /// Sets the value of MemoryAvailable
    pub fn set_memory_available(&mut self, value: i32) {
        self.memory_available = Some(value);
    }

    /// Gets the value of MemoryAvailable
    pub fn get_memory_available(&self) -> Option<&i32> {
        self.memory_available.as_ref()
    }

    /// Sets the value of MemorySpansPhysicalNumaNodes
    pub fn set_memory_spans_physical_numa_nodes(&mut self, value: bool) {
        self.memory_spans_physical_numa_nodes = Some(value);
    }

    /// Gets the value of MemorySpansPhysicalNumaNodes
    pub fn get_memory_spans_physical_numa_nodes(&self) -> Option<&bool> {
        self.memory_spans_physical_numa_nodes.as_ref()
    }

    /// Sets the value of MemoryUsage
    pub fn set_memory_usage(&mut self, value: u64) {
        self.memory_usage = Some(value);
    }

    /// Gets the value of MemoryUsage
    pub fn get_memory_usage(&self) -> Option<&u64> {
        self.memory_usage.as_ref()
    }

    /// Sets the value of ProcessorLoad
    pub fn set_processor_load(&mut self, value: u16) {
        self.processor_load = Some(value);
    }

    /// Gets the value of ProcessorLoad
    pub fn get_processor_load(&self) -> Option<&u16> {
        self.processor_load.as_ref()
    }

    /// Sets the value of ProcessorLoadHistory
    pub fn set_processor_load_history(&mut self, value: Vec<u16>) {
        self.processor_load_history = value;
    }

    /// Gets the value of ProcessorLoadHistory
    pub fn get_processor_load_history(&self) -> &Vec<u16> {
        &self.processor_load_history
    }

    /// Sets the value of ReplicationHealth
    pub fn set_replication_health(&mut self, value: u16) {
        self.replication_health = Some(value);
    }

    /// Gets the value of ReplicationHealth
    pub fn get_replication_health(&self) -> Option<&u16> {
        self.replication_health.as_ref()
    }

    /// Sets the value of ReplicationHealthEx
    pub fn set_replication_health_ex(&mut self, value: Vec<u16>) {
        self.replication_health_ex = value;
    }

    /// Gets the value of ReplicationHealthEx
    pub fn get_replication_health_ex(&self) -> &Vec<u16> {
        &self.replication_health_ex
    }

    /// Sets the value of ReplicationMode
    pub fn set_replication_mode(&mut self, value: u16) {
        self.replication_mode = Some(value);
    }

    /// Gets the value of ReplicationMode
    pub fn get_replication_mode(&self) -> Option<&u16> {
        self.replication_mode.as_ref()
    }

    /// Sets the value of ReplicationProviderId
    pub fn set_replication_provider_id(&mut self, value: Vec<String>) {
        self.replication_provider_id = value;
    }

    /// Gets the value of ReplicationProviderId
    pub fn get_replication_provider_id(&self) -> &Vec<String> {
        &self.replication_provider_id
    }

    /// Sets the value of ReplicationState
    pub fn set_replication_state(&mut self, value: u16) {
        self.replication_state = Some(value);
    }

    /// Gets the value of ReplicationState
    pub fn get_replication_state(&self) -> Option<&u16> {
        self.replication_state.as_ref()
    }

    /// Sets the value of ReplicationStateEx
    pub fn set_replication_state_ex(&mut self, value: Vec<u16>) {
        self.replication_state_ex = value;
    }

    /// Gets the value of ReplicationStateEx
    pub fn get_replication_state_ex(&self) -> &Vec<u16> {
        &self.replication_state_ex
    }

    /// Sets the value of Shielded
    pub fn set_shielded(&mut self, value: bool) {
        self.shielded = Some(value);
    }

    /// Gets the value of Shielded
    pub fn get_shielded(&self) -> Option<&bool> {
        self.shielded.as_ref()
    }

    /// Sets the value of Snapshots
    pub fn set_snapshots(&mut self, value: Vec<CIM_VirtualSystemSettingData>) {
        self.snapshots = value;
    }

    /// Gets the value of Snapshots
    pub fn get_snapshots(&self) -> &Vec<CIM_VirtualSystemSettingData> {
        &self.snapshots
    }

    /// Sets the value of SwapFilesInUse
    pub fn set_swap_files_in_use(&mut self, value: bool) {
        self.swap_files_in_use = Some(value);
    }

    /// Gets the value of SwapFilesInUse
    pub fn get_swap_files_in_use(&self) -> Option<&bool> {
        self.swap_files_in_use.as_ref()
    }

    /// Sets the value of TestReplicaSystem
    pub fn set_test_replica_system(&mut self, value: CIM_ComputerSystem) {
        self.test_replica_system = Some(value);
    }

    /// Gets the value of TestReplicaSystem
    pub fn get_test_replica_system(&self) -> Option<&CIM_ComputerSystem> {
        self.test_replica_system.as_ref()
    }

    /// Sets the value of ThumbnailImage
    pub fn set_thumbnail_image(&mut self, value: Vec<u8>) {
        self.thumbnail_image = value;
    }

    /// Gets the value of ThumbnailImage
    pub fn get_thumbnail_image(&self) -> &Vec<u8> {
        &self.thumbnail_image
    }

    /// Sets the value of ThumbnailImageHeight
    pub fn set_thumbnail_image_height(&mut self, value: u16) {
        self.thumbnail_image_height = Some(value);
    }

    /// Gets the value of ThumbnailImageHeight
    pub fn get_thumbnail_image_height(&self) -> Option<&u16> {
        self.thumbnail_image_height.as_ref()
    }

    /// Sets the value of ThumbnailImageWidth
    pub fn set_thumbnail_image_width(&mut self, value: u16) {
        self.thumbnail_image_width = Some(value);
    }

    /// Gets the value of ThumbnailImageWidth
    pub fn get_thumbnail_image_width(&self) -> Option<&u16> {
        self.thumbnail_image_width.as_ref()
    }
}

