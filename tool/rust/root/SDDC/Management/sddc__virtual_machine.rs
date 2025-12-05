// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_VirtualMachine struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_VirtualMachine {

/// 
    #[serde(rename = "Alerts")]
    pub alerts: Vec<SDDC_Alert>,

/// 
    #[serde(rename = "BootOrder")]
    pub boot_order: Vec<u16>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "CpuUsage")]
    pub cpu_usage: Option<f64>,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "EncryptStateAndVmMigrationTraffic")]
    pub encrypt_state_and_vm_migration_traffic: Option<bool>,

/// 
    #[serde(rename = "Generation")]
    pub generation: Option<u16>,

/// 
    #[serde(rename = "GuestStateDataRoot")]
    pub guest_state_data_root: Option<String>,

/// 
    #[serde(rename = "GuestStateFile")]
    pub guest_state_file: Option<String>,

/// 
    #[serde(rename = "GuestStateIsolationType")]
    pub guest_state_isolation_type: Option<u16>,

/// 
    #[serde(rename = "HeartBeat")]
    pub heart_beat: Option<u16>,

/// 
    #[serde(rename = "Host")]
    pub host: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IntegratedServiceVersion")]
    pub integrated_service_version: Option<String>,

/// 
    #[serde(rename = "IsClustered")]
    pub is_clustered: Option<bool>,

/// 
    #[serde(rename = "IsDynamicMemoryEnabled")]
    pub is_dynamic_memory_enabled: Option<bool>,

/// 
    #[serde(rename = "IsGuestStateIsolationEnabled")]
    pub is_guest_state_isolation_enabled: Option<bool>,

/// 
    #[serde(rename = "LatestSnapshot")]
    pub latest_snapshot: Option<String>,

/// 
    #[serde(rename = "MemoryAssigned")]
    pub memory_assigned: Option<u64>,

/// 
    #[serde(rename = "MemoryDemand")]
    pub memory_demand: Option<u64>,

/// 
    #[serde(rename = "MemoryStartUp")]
    pub memory_start_up: Option<u64>,

/// 
    #[serde(rename = "MemoryStartUpUnits")]
    pub memory_start_up_units: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Os")]
    pub os: Option<String>,

/// 
    #[serde(rename = "OsVersion")]
    pub os_version: Option<String>,

/// 
    #[serde(rename = "ParentCheckpointId")]
    pub parent_checkpoint_id: Option<String>,

/// 
    #[serde(rename = "ProcessorCount")]
    pub processor_count: Option<u16>,

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
    #[serde(rename = "SecureBootEnabled")]
    pub secure_boot_enabled: Option<bool>,

/// 
    #[serde(rename = "ShieldingRequested")]
    pub shielding_requested: Option<bool>,

/// 
    #[serde(rename = "SizeOfSystemFiles")]
    pub size_of_system_files: Option<u64>,

/// 
    #[serde(rename = "SnapshotDataRoot")]
    pub snapshot_data_root: Option<String>,

/// 
    #[serde(rename = "Snapshots")]
    pub snapshots: Vec<SDDC_VmSnapshot>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u16>,

/// 
    #[serde(rename = "Status")]
    pub status: Vec<u16>,

/// 
    #[serde(rename = "TotalIops")]
    pub total_iops: Option<f64>,

/// 
    #[serde(rename = "TotalNetworkUsage")]
    pub total_network_usage: Option<f64>,

/// 
    #[serde(rename = "TotalThroughput")]
    pub total_throughput: Option<f64>,

/// 
    #[serde(rename = "TpmEnabled")]
    pub tpm_enabled: Option<bool>,

/// 
    #[serde(rename = "Uptime")]
    pub uptime: Option<String>,

/// 
    #[serde(rename = "UserSnapshotType")]
    pub user_snapshot_type: Option<u16>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "Vhds")]
    pub vhds: Vec<SDDC_Vhd>,

/// 
    #[serde(rename = "VirtualSystemType")]
    pub virtual_system_type: Option<String>,

/// 
    #[serde(rename = "VmIntegrationServices")]
    pub vm_integration_services: Vec<SDDC_VmIntegrationService>,

/// 
    #[serde(rename = "VNics")]
    pub vnics: Vec<SDDC_VmNetAdapter>,
}

impl SDDC_VirtualMachine {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
            boot_order: Vec::new(),
            computer_name: None,
            cpu_usage: None,
            creation_time: None,
            encrypt_state_and_vm_migration_traffic: None,
            generation: None,
            guest_state_data_root: None,
            guest_state_file: None,
            guest_state_isolation_type: None,
            heart_beat: None,
            host: None,
            id: None,
            integrated_service_version: None,
            is_clustered: None,
            is_dynamic_memory_enabled: None,
            is_guest_state_isolation_enabled: None,
            latest_snapshot: None,
            memory_assigned: None,
            memory_demand: None,
            memory_start_up: None,
            memory_start_up_units: None,
            name: None,
            os: None,
            os_version: None,
            parent_checkpoint_id: None,
            processor_count: None,
            replication_health: None,
            replication_mode: None,
            replication_state: None,
            secure_boot_enabled: None,
            shielding_requested: None,
            size_of_system_files: None,
            snapshot_data_root: None,
            snapshots: Vec::new(),
            state: None,
            status: Vec::new(),
            total_iops: None,
            total_network_usage: None,
            total_throughput: None,
            tpm_enabled: None,
            uptime: None,
            user_snapshot_type: None,
            version: None,
            vhds: Vec::new(),
            virtual_system_type: None,
            vm_integration_services: Vec::new(),
            vnics: Vec::new(),
        }
    }


    /// Sets the value of Alerts
    pub fn set_alerts(&mut self, value: Vec<SDDC_Alert>) {
        self.alerts = value;
    }

    /// Gets the value of Alerts
    pub fn get_alerts(&self) -> &Vec<SDDC_Alert> {
        &self.alerts
    }

    /// Sets the value of BootOrder
    pub fn set_boot_order(&mut self, value: Vec<u16>) {
        self.boot_order = value;
    }

    /// Gets the value of BootOrder
    pub fn get_boot_order(&self) -> &Vec<u16> {
        &self.boot_order
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of CpuUsage
    pub fn set_cpu_usage(&mut self, value: f64) {
        self.cpu_usage = Some(value);
    }

    /// Gets the value of CpuUsage
    pub fn get_cpu_usage(&self) -> Option<&f64> {
        self.cpu_usage.as_ref()
    }

    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of EncryptStateAndVmMigrationTraffic
    pub fn set_encrypt_state_and_vm_migration_traffic(&mut self, value: bool) {
        self.encrypt_state_and_vm_migration_traffic = Some(value);
    }

    /// Gets the value of EncryptStateAndVmMigrationTraffic
    pub fn get_encrypt_state_and_vm_migration_traffic(&self) -> Option<&bool> {
        self.encrypt_state_and_vm_migration_traffic.as_ref()
    }

    /// Sets the value of Generation
    pub fn set_generation(&mut self, value: u16) {
        self.generation = Some(value);
    }

    /// Gets the value of Generation
    pub fn get_generation(&self) -> Option<&u16> {
        self.generation.as_ref()
    }

    /// Sets the value of GuestStateDataRoot
    pub fn set_guest_state_data_root(&mut self, value: String) {
        self.guest_state_data_root = Some(value);
    }

    /// Gets the value of GuestStateDataRoot
    pub fn get_guest_state_data_root(&self) -> Option<&String> {
        self.guest_state_data_root.as_ref()
    }

    /// Sets the value of GuestStateFile
    pub fn set_guest_state_file(&mut self, value: String) {
        self.guest_state_file = Some(value);
    }

    /// Gets the value of GuestStateFile
    pub fn get_guest_state_file(&self) -> Option<&String> {
        self.guest_state_file.as_ref()
    }

    /// Sets the value of GuestStateIsolationType
    pub fn set_guest_state_isolation_type(&mut self, value: u16) {
        self.guest_state_isolation_type = Some(value);
    }

    /// Gets the value of GuestStateIsolationType
    pub fn get_guest_state_isolation_type(&self) -> Option<&u16> {
        self.guest_state_isolation_type.as_ref()
    }

    /// Sets the value of HeartBeat
    pub fn set_heart_beat(&mut self, value: u16) {
        self.heart_beat = Some(value);
    }

    /// Gets the value of HeartBeat
    pub fn get_heart_beat(&self) -> Option<&u16> {
        self.heart_beat.as_ref()
    }

    /// Sets the value of Host
    pub fn set_host(&mut self, value: String) {
        self.host = Some(value);
    }

    /// Gets the value of Host
    pub fn get_host(&self) -> Option<&String> {
        self.host.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IntegratedServiceVersion
    pub fn set_integrated_service_version(&mut self, value: String) {
        self.integrated_service_version = Some(value);
    }

    /// Gets the value of IntegratedServiceVersion
    pub fn get_integrated_service_version(&self) -> Option<&String> {
        self.integrated_service_version.as_ref()
    }

    /// Sets the value of IsClustered
    pub fn set_is_clustered(&mut self, value: bool) {
        self.is_clustered = Some(value);
    }

    /// Gets the value of IsClustered
    pub fn get_is_clustered(&self) -> Option<&bool> {
        self.is_clustered.as_ref()
    }

    /// Sets the value of IsDynamicMemoryEnabled
    pub fn set_is_dynamic_memory_enabled(&mut self, value: bool) {
        self.is_dynamic_memory_enabled = Some(value);
    }

    /// Gets the value of IsDynamicMemoryEnabled
    pub fn get_is_dynamic_memory_enabled(&self) -> Option<&bool> {
        self.is_dynamic_memory_enabled.as_ref()
    }

    /// Sets the value of IsGuestStateIsolationEnabled
    pub fn set_is_guest_state_isolation_enabled(&mut self, value: bool) {
        self.is_guest_state_isolation_enabled = Some(value);
    }

    /// Gets the value of IsGuestStateIsolationEnabled
    pub fn get_is_guest_state_isolation_enabled(&self) -> Option<&bool> {
        self.is_guest_state_isolation_enabled.as_ref()
    }

    /// Sets the value of LatestSnapshot
    pub fn set_latest_snapshot(&mut self, value: String) {
        self.latest_snapshot = Some(value);
    }

    /// Gets the value of LatestSnapshot
    pub fn get_latest_snapshot(&self) -> Option<&String> {
        self.latest_snapshot.as_ref()
    }

    /// Sets the value of MemoryAssigned
    pub fn set_memory_assigned(&mut self, value: u64) {
        self.memory_assigned = Some(value);
    }

    /// Gets the value of MemoryAssigned
    pub fn get_memory_assigned(&self) -> Option<&u64> {
        self.memory_assigned.as_ref()
    }

    /// Sets the value of MemoryDemand
    pub fn set_memory_demand(&mut self, value: u64) {
        self.memory_demand = Some(value);
    }

    /// Gets the value of MemoryDemand
    pub fn get_memory_demand(&self) -> Option<&u64> {
        self.memory_demand.as_ref()
    }

    /// Sets the value of MemoryStartUp
    pub fn set_memory_start_up(&mut self, value: u64) {
        self.memory_start_up = Some(value);
    }

    /// Gets the value of MemoryStartUp
    pub fn get_memory_start_up(&self) -> Option<&u64> {
        self.memory_start_up.as_ref()
    }

    /// Sets the value of MemoryStartUpUnits
    pub fn set_memory_start_up_units(&mut self, value: String) {
        self.memory_start_up_units = Some(value);
    }

    /// Gets the value of MemoryStartUpUnits
    pub fn get_memory_start_up_units(&self) -> Option<&String> {
        self.memory_start_up_units.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Os
    pub fn set_os(&mut self, value: String) {
        self.os = Some(value);
    }

    /// Gets the value of Os
    pub fn get_os(&self) -> Option<&String> {
        self.os.as_ref()
    }

    /// Sets the value of OsVersion
    pub fn set_os_version(&mut self, value: String) {
        self.os_version = Some(value);
    }

    /// Gets the value of OsVersion
    pub fn get_os_version(&self) -> Option<&String> {
        self.os_version.as_ref()
    }

    /// Sets the value of ParentCheckpointId
    pub fn set_parent_checkpoint_id(&mut self, value: String) {
        self.parent_checkpoint_id = Some(value);
    }

    /// Gets the value of ParentCheckpointId
    pub fn get_parent_checkpoint_id(&self) -> Option<&String> {
        self.parent_checkpoint_id.as_ref()
    }

    /// Sets the value of ProcessorCount
    pub fn set_processor_count(&mut self, value: u16) {
        self.processor_count = Some(value);
    }

    /// Gets the value of ProcessorCount
    pub fn get_processor_count(&self) -> Option<&u16> {
        self.processor_count.as_ref()
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

    /// Sets the value of SecureBootEnabled
    pub fn set_secure_boot_enabled(&mut self, value: bool) {
        self.secure_boot_enabled = Some(value);
    }

    /// Gets the value of SecureBootEnabled
    pub fn get_secure_boot_enabled(&self) -> Option<&bool> {
        self.secure_boot_enabled.as_ref()
    }

    /// Sets the value of ShieldingRequested
    pub fn set_shielding_requested(&mut self, value: bool) {
        self.shielding_requested = Some(value);
    }

    /// Gets the value of ShieldingRequested
    pub fn get_shielding_requested(&self) -> Option<&bool> {
        self.shielding_requested.as_ref()
    }

    /// Sets the value of SizeOfSystemFiles
    pub fn set_size_of_system_files(&mut self, value: u64) {
        self.size_of_system_files = Some(value);
    }

    /// Gets the value of SizeOfSystemFiles
    pub fn get_size_of_system_files(&self) -> Option<&u64> {
        self.size_of_system_files.as_ref()
    }

    /// Sets the value of SnapshotDataRoot
    pub fn set_snapshot_data_root(&mut self, value: String) {
        self.snapshot_data_root = Some(value);
    }

    /// Gets the value of SnapshotDataRoot
    pub fn get_snapshot_data_root(&self) -> Option<&String> {
        self.snapshot_data_root.as_ref()
    }

    /// Sets the value of Snapshots
    pub fn set_snapshots(&mut self, value: Vec<SDDC_VmSnapshot>) {
        self.snapshots = value;
    }

    /// Gets the value of Snapshots
    pub fn get_snapshots(&self) -> &Vec<SDDC_VmSnapshot> {
        &self.snapshots
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u16) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u16> {
        self.state.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: Vec<u16>) {
        self.status = value;
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> &Vec<u16> {
        &self.status
    }

    /// Sets the value of TotalIops
    pub fn set_total_iops(&mut self, value: f64) {
        self.total_iops = Some(value);
    }

    /// Gets the value of TotalIops
    pub fn get_total_iops(&self) -> Option<&f64> {
        self.total_iops.as_ref()
    }

    /// Sets the value of TotalNetworkUsage
    pub fn set_total_network_usage(&mut self, value: f64) {
        self.total_network_usage = Some(value);
    }

    /// Gets the value of TotalNetworkUsage
    pub fn get_total_network_usage(&self) -> Option<&f64> {
        self.total_network_usage.as_ref()
    }

    /// Sets the value of TotalThroughput
    pub fn set_total_throughput(&mut self, value: f64) {
        self.total_throughput = Some(value);
    }

    /// Gets the value of TotalThroughput
    pub fn get_total_throughput(&self) -> Option<&f64> {
        self.total_throughput.as_ref()
    }

    /// Sets the value of TpmEnabled
    pub fn set_tpm_enabled(&mut self, value: bool) {
        self.tpm_enabled = Some(value);
    }

    /// Gets the value of TpmEnabled
    pub fn get_tpm_enabled(&self) -> Option<&bool> {
        self.tpm_enabled.as_ref()
    }

    /// Sets the value of Uptime
    pub fn set_uptime(&mut self, value: String) {
        self.uptime = Some(value);
    }

    /// Gets the value of Uptime
    pub fn get_uptime(&self) -> Option<&String> {
        self.uptime.as_ref()
    }

    /// Sets the value of UserSnapshotType
    pub fn set_user_snapshot_type(&mut self, value: u16) {
        self.user_snapshot_type = Some(value);
    }

    /// Gets the value of UserSnapshotType
    pub fn get_user_snapshot_type(&self) -> Option<&u16> {
        self.user_snapshot_type.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of Vhds
    pub fn set_vhds(&mut self, value: Vec<SDDC_Vhd>) {
        self.vhds = value;
    }

    /// Gets the value of Vhds
    pub fn get_vhds(&self) -> &Vec<SDDC_Vhd> {
        &self.vhds
    }

    /// Sets the value of VirtualSystemType
    pub fn set_virtual_system_type(&mut self, value: String) {
        self.virtual_system_type = Some(value);
    }

    /// Gets the value of VirtualSystemType
    pub fn get_virtual_system_type(&self) -> Option<&String> {
        self.virtual_system_type.as_ref()
    }

    /// Sets the value of VmIntegrationServices
    pub fn set_vm_integration_services(&mut self, value: Vec<SDDC_VmIntegrationService>) {
        self.vm_integration_services = value;
    }

    /// Gets the value of VmIntegrationServices
    pub fn get_vm_integration_services(&self) -> &Vec<SDDC_VmIntegrationService> {
        &self.vm_integration_services
    }

    /// Sets the value of VNics
    pub fn set_vnics(&mut self, value: Vec<SDDC_VmNetAdapter>) {
        self.vnics = value;
    }

    /// Gets the value of VNics
    pub fn get_vnics(&self) -> &Vec<SDDC_VmNetAdapter> {
        &self.vnics
    }

/// 

    /// * `series_name` -  (String)
    /// * `time_frame` -  (u16)

    /// * `metric` -  (SDDC_Metric)
    /// * `return_value` -  (u32)
    pub fn get_metrics(&self, series_name: &String, time_frame: u16, metric: &mut SDDC_Metric) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SeriesName".to_string(), value: series_name.into() });
        args.push(MethodParameter { name: "TimeFrame".to_string(), value: time_frame.into() });

        let result = self.invoke_method("GetMetrics", &args)?;
        let metric = result.get_value("Metric")?;
        Ok(result.return_value)

    }


/// 

    /// * `refresh_type` -  (u16)

    /// * `return_value` -  (u32)
    pub fn refresh(&self, refresh_type: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RefreshType".to_string(), value: refresh_type.into() });
        self.invoke_method("Refresh", &args)

    }

}

