// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemSettingData {
    #[serde(flatten)]
    pub base: CIM_VirtualSystemSettingData,

/// 
    #[serde(rename = "AdditionalRecoveryInformation")]
    pub additional_recovery_information: Option<String>,

/// 
    #[serde(rename = "AllowFullSCSICommandSet")]
    pub allow_full_scsicommand_set: Option<bool>,

/// 
    #[serde(rename = "AllowReducedFcRedundancy")]
    pub allow_reduced_fc_redundancy: Option<bool>,

/// 
    #[serde(rename = "Architecture")]
    pub architecture: Option<VirtualSystemSettingData_Architecture>,

/// 
    #[serde(rename = "AutomaticCriticalErrorAction")]
    pub automatic_critical_error_action: Option<VirtualSystemSettingData_AutomaticCriticalErrorAction>,

/// 
    #[serde(rename = "AutomaticCriticalErrorActionTimeout")]
    pub automatic_critical_error_action_timeout: Option<String>,

/// 
    #[serde(rename = "AutomaticSnapshotsEnabled")]
    pub automatic_snapshots_enabled: Option<bool>,

/// 
    #[serde(rename = "BaseBoardSerialNumber")]
    pub base_board_serial_number: Option<String>,

/// 
    #[serde(rename = "BIOSGUID")]
    pub biosguid: Option<String>,

/// 
    #[serde(rename = "BIOSNumLock")]
    pub biosnum_lock: Option<bool>,

/// 
    #[serde(rename = "BIOSSerialNumber")]
    pub biosserial_number: Option<String>,

/// 
    #[serde(rename = "BootOrder")]
    pub boot_order: Vec<u16>,

/// 
    #[serde(rename = "BootPciExpress")]
    pub boot_pci_express: Option<bool>,

/// 
    #[serde(rename = "BootPciExpressInstanceFilter")]
    pub boot_pci_express_instance_filter: Option<String>,

/// 
    #[serde(rename = "BootSourceOrder")]
    pub boot_source_order: Vec<String>,

/// 
    #[serde(rename = "ChassisAssetTag")]
    pub chassis_asset_tag: Option<String>,

/// 
    #[serde(rename = "ChassisSerialNumber")]
    pub chassis_serial_number: Option<String>,

/// 
    #[serde(rename = "ClusterWideNodeCapabilitiesValidationMode")]
    pub cluster_wide_node_capabilities_validation_mode: Option<VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode>,

/// 
    #[serde(rename = "ConsoleMode")]
    pub console_mode: Option<VirtualSystemSettingData_ConsoleMode>,

/// 
    #[serde(rename = "DebugChannelId")]
    pub debug_channel_id: Option<u32>,

/// 
    #[serde(rename = "DebugPort")]
    pub debug_port: Option<u32>,

/// 
    #[serde(rename = "DebugPortEnabled")]
    pub debug_port_enabled: Option<VirtualSystemSettingData_DebugPortEnabled>,

/// 
    #[serde(rename = "EnableHibernation")]
    pub enable_hibernation: Option<bool>,

/// 
    #[serde(rename = "EnhancedSessionTransportType")]
    pub enhanced_session_transport_type: Option<u16>,

/// 
    #[serde(rename = "FirmwareFile")]
    pub firmware_file: Option<String>,

/// 
    #[serde(rename = "FirmwareParameters")]
    pub firmware_parameters: Vec<u8>,

/// 
    #[serde(rename = "GuestControlledCacheTypes")]
    pub guest_controlled_cache_types: Option<bool>,

/// 
    #[serde(rename = "GuestFeatureSet")]
    pub guest_feature_set: Option<u64>,

/// Filepath of a directory where information about the guest runtime state is stored.
    #[serde(rename = "GuestStateDataRoot")]
    pub guest_state_data_root: Option<String>,

/// Filepath of a file where information about the guest runtime state is stored. A relative path appends to the value of the GuestStateDataRoot property.
    #[serde(rename = "GuestStateFile")]
    pub guest_state_file: Option<String>,

/// 
    #[serde(rename = "GuestStateIsolationEnabled")]
    pub guest_state_isolation_enabled: Option<bool>,

/// 
    #[serde(rename = "GuestStateIsolationMode")]
    pub guest_state_isolation_mode: Option<u16>,

/// 
    #[serde(rename = "GuestStateIsolationType")]
    pub guest_state_isolation_type: Option<u16>,

/// 
    #[serde(rename = "HighMmioGapBase")]
    pub high_mmio_gap_base: Option<u64>,

/// 
    #[serde(rename = "HighMmioGapSize")]
    pub high_mmio_gap_size: Option<u64>,

/// 
    #[serde(rename = "IncrementalBackupEnabled")]
    pub incremental_backup_enabled: Option<bool>,

/// 
    #[serde(rename = "IsAutomaticSnapshot")]
    pub is_automatic_snapshot: Option<bool>,

/// 
    #[serde(rename = "IsSaved")]
    pub is_saved: Option<bool>,

/// 
    #[serde(rename = "LockOnDisconnect")]
    pub lock_on_disconnect: Option<bool>,

/// 
    #[serde(rename = "LowMmioGapSize")]
    pub low_mmio_gap_size: Option<u64>,

/// 
    #[serde(rename = "ManagementVtlUpdatePolicy")]
    pub management_vtl_update_policy: Option<u16>,

/// 
    #[serde(rename = "MemoryHostingJobObjectName")]
    pub memory_hosting_job_object_name: Option<String>,

/// 
    #[serde(rename = "NetworkBootPreferredProtocol")]
    pub network_boot_preferred_protocol: Option<VirtualSystemSettingData_NetworkBootPreferredProtocol>,

/// 
    #[serde(rename = "NumaNodeTopologyArray")]
    pub numa_node_topology_array: Vec<String>,

/// 
    #[serde(rename = "Parent")]
    pub parent: Option<String>,

/// 
    #[serde(rename = "PauseAfterBootFailure")]
    pub pause_after_boot_failure: Option<bool>,

/// 
    #[serde(rename = "SecureBootEnabled")]
    pub secure_boot_enabled: Option<bool>,

/// 
    #[serde(rename = "SecureBootTemplateId")]
    pub secure_boot_template_id: Option<String>,

/// Filepath to a source file that will be copied and used as the guest runtime state. 
    #[serde(rename = "SourceGuestStateFile")]
    pub source_guest_state_file: Option<String>,

/// 
    #[serde(rename = "TurnOffOnGuestRestart")]
    pub turn_off_on_guest_restart: Option<bool>,

/// 
    #[serde(rename = "UserSnapshotType")]
    pub user_snapshot_type: Option<u16>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VirtualNumaEnabled")]
    pub virtual_numa_enabled: Option<bool>,

/// 
    #[serde(rename = "VirtualSlitType")]
    pub virtual_slit_type: Option<VirtualSystemSettingData_VirtualSlitType>,

/// 
    #[serde(rename = "VirtualSystemSubType")]
    pub virtual_system_sub_type: Option<VirtualSystemSettingData_VirtualSystemSubType>,

/// 
    #[serde(rename = "VMBusMessageRedirection")]
    pub vmbus_message_redirection: Option<u16>,

/// 
    #[serde(rename = "Vtl2AddressRangeBase")]
    pub vtl2_address_range_base: Option<u64>,

/// 
    #[serde(rename = "Vtl2AddressRangeSize")]
    pub vtl2_address_range_size: Option<u64>,

/// 
    #[serde(rename = "Vtl2AddressSpaceConfigurationMode")]
    pub vtl2_address_space_configuration_mode: Option<VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode>,

/// 
    #[serde(rename = "Vtl2MmioAddressRangeSize")]
    pub vtl2_mmio_address_range_size: Option<u64>,

/// 
    #[serde(rename = "WatchdogEnabled")]
    pub watchdog_enabled: Option<bool>,

/// 
    #[serde(rename = "WorkerJobObjectName")]
    pub worker_job_object_name: Option<String>,
}

impl Msvm_VirtualSystemSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VirtualSystemSettingData::new(),
            additional_recovery_information: None,
            allow_full_scsicommand_set: None,
            allow_reduced_fc_redundancy: None,
            architecture: None,
            automatic_critical_error_action: None,
            automatic_critical_error_action_timeout: None,
            automatic_snapshots_enabled: None,
            base_board_serial_number: None,
            biosguid: None,
            biosnum_lock: None,
            biosserial_number: None,
            boot_order: Vec::new(),
            boot_pci_express: None,
            boot_pci_express_instance_filter: None,
            boot_source_order: Vec::new(),
            chassis_asset_tag: None,
            chassis_serial_number: None,
            cluster_wide_node_capabilities_validation_mode: None,
            console_mode: None,
            debug_channel_id: None,
            debug_port: None,
            debug_port_enabled: None,
            enable_hibernation: None,
            enhanced_session_transport_type: None,
            firmware_file: None,
            firmware_parameters: Vec::new(),
            guest_controlled_cache_types: None,
            guest_feature_set: None,
            guest_state_data_root: None,
            guest_state_file: None,
            guest_state_isolation_enabled: None,
            guest_state_isolation_mode: None,
            guest_state_isolation_type: None,
            high_mmio_gap_base: None,
            high_mmio_gap_size: None,
            incremental_backup_enabled: None,
            is_automatic_snapshot: None,
            is_saved: None,
            lock_on_disconnect: None,
            low_mmio_gap_size: None,
            management_vtl_update_policy: None,
            memory_hosting_job_object_name: None,
            network_boot_preferred_protocol: None,
            numa_node_topology_array: Vec::new(),
            parent: None,
            pause_after_boot_failure: None,
            secure_boot_enabled: None,
            secure_boot_template_id: None,
            source_guest_state_file: None,
            turn_off_on_guest_restart: None,
            user_snapshot_type: None,
            version: None,
            virtual_numa_enabled: None,
            virtual_slit_type: None,
            virtual_system_sub_type: None,
            vmbus_message_redirection: None,
            vtl2_address_range_base: None,
            vtl2_address_range_size: None,
            vtl2_address_space_configuration_mode: None,
            vtl2_mmio_address_range_size: None,
            watchdog_enabled: None,
            worker_job_object_name: None,
        }
    }


    /// Sets the value of AdditionalRecoveryInformation
    pub fn set_additional_recovery_information(&mut self, value: String) {
        self.additional_recovery_information = Some(value);
    }

    /// Gets the value of AdditionalRecoveryInformation
    pub fn get_additional_recovery_information(&self) -> Option<&String> {
        self.additional_recovery_information.as_ref()
    }

    /// Sets the value of AllowFullSCSICommandSet
    pub fn set_allow_full_scsicommand_set(&mut self, value: bool) {
        self.allow_full_scsicommand_set = Some(value);
    }

    /// Gets the value of AllowFullSCSICommandSet
    pub fn get_allow_full_scsicommand_set(&self) -> Option<&bool> {
        self.allow_full_scsicommand_set.as_ref()
    }

    /// Sets the value of AllowReducedFcRedundancy
    pub fn set_allow_reduced_fc_redundancy(&mut self, value: bool) {
        self.allow_reduced_fc_redundancy = Some(value);
    }

    /// Gets the value of AllowReducedFcRedundancy
    pub fn get_allow_reduced_fc_redundancy(&self) -> Option<&bool> {
        self.allow_reduced_fc_redundancy.as_ref()
    }

    /// Sets the value of Architecture
    pub fn set_architecture(&mut self, value: VirtualSystemSettingData_Architecture) {
        self.architecture = Some(value);
    }

    /// Gets the value of Architecture
    pub fn get_architecture(&self) -> Option<&VirtualSystemSettingData_Architecture> {
        self.architecture.as_ref()
    }

    /// Sets the value of AutomaticCriticalErrorAction
    pub fn set_automatic_critical_error_action(&mut self, value: VirtualSystemSettingData_AutomaticCriticalErrorAction) {
        self.automatic_critical_error_action = Some(value);
    }

    /// Gets the value of AutomaticCriticalErrorAction
    pub fn get_automatic_critical_error_action(&self) -> Option<&VirtualSystemSettingData_AutomaticCriticalErrorAction> {
        self.automatic_critical_error_action.as_ref()
    }

    /// Sets the value of AutomaticCriticalErrorActionTimeout
    pub fn set_automatic_critical_error_action_timeout(&mut self, value: String) {
        self.automatic_critical_error_action_timeout = Some(value);
    }

    /// Gets the value of AutomaticCriticalErrorActionTimeout
    pub fn get_automatic_critical_error_action_timeout(&self) -> Option<&String> {
        self.automatic_critical_error_action_timeout.as_ref()
    }

    /// Sets the value of AutomaticSnapshotsEnabled
    pub fn set_automatic_snapshots_enabled(&mut self, value: bool) {
        self.automatic_snapshots_enabled = Some(value);
    }

    /// Gets the value of AutomaticSnapshotsEnabled
    pub fn get_automatic_snapshots_enabled(&self) -> Option<&bool> {
        self.automatic_snapshots_enabled.as_ref()
    }

    /// Sets the value of BaseBoardSerialNumber
    pub fn set_base_board_serial_number(&mut self, value: String) {
        self.base_board_serial_number = Some(value);
    }

    /// Gets the value of BaseBoardSerialNumber
    pub fn get_base_board_serial_number(&self) -> Option<&String> {
        self.base_board_serial_number.as_ref()
    }

    /// Sets the value of BIOSGUID
    pub fn set_biosguid(&mut self, value: String) {
        self.biosguid = Some(value);
    }

    /// Gets the value of BIOSGUID
    pub fn get_biosguid(&self) -> Option<&String> {
        self.biosguid.as_ref()
    }

    /// Sets the value of BIOSNumLock
    pub fn set_biosnum_lock(&mut self, value: bool) {
        self.biosnum_lock = Some(value);
    }

    /// Gets the value of BIOSNumLock
    pub fn get_biosnum_lock(&self) -> Option<&bool> {
        self.biosnum_lock.as_ref()
    }

    /// Sets the value of BIOSSerialNumber
    pub fn set_biosserial_number(&mut self, value: String) {
        self.biosserial_number = Some(value);
    }

    /// Gets the value of BIOSSerialNumber
    pub fn get_biosserial_number(&self) -> Option<&String> {
        self.biosserial_number.as_ref()
    }

    /// Sets the value of BootOrder
    pub fn set_boot_order(&mut self, value: Vec<u16>) {
        self.boot_order = value;
    }

    /// Gets the value of BootOrder
    pub fn get_boot_order(&self) -> &Vec<u16> {
        &self.boot_order
    }

    /// Sets the value of BootPciExpress
    pub fn set_boot_pci_express(&mut self, value: bool) {
        self.boot_pci_express = Some(value);
    }

    /// Gets the value of BootPciExpress
    pub fn get_boot_pci_express(&self) -> Option<&bool> {
        self.boot_pci_express.as_ref()
    }

    /// Sets the value of BootPciExpressInstanceFilter
    pub fn set_boot_pci_express_instance_filter(&mut self, value: String) {
        self.boot_pci_express_instance_filter = Some(value);
    }

    /// Gets the value of BootPciExpressInstanceFilter
    pub fn get_boot_pci_express_instance_filter(&self) -> Option<&String> {
        self.boot_pci_express_instance_filter.as_ref()
    }

    /// Sets the value of BootSourceOrder
    pub fn set_boot_source_order(&mut self, value: Vec<String>) {
        self.boot_source_order = value;
    }

    /// Gets the value of BootSourceOrder
    pub fn get_boot_source_order(&self) -> &Vec<String> {
        &self.boot_source_order
    }

    /// Sets the value of ChassisAssetTag
    pub fn set_chassis_asset_tag(&mut self, value: String) {
        self.chassis_asset_tag = Some(value);
    }

    /// Gets the value of ChassisAssetTag
    pub fn get_chassis_asset_tag(&self) -> Option<&String> {
        self.chassis_asset_tag.as_ref()
    }

    /// Sets the value of ChassisSerialNumber
    pub fn set_chassis_serial_number(&mut self, value: String) {
        self.chassis_serial_number = Some(value);
    }

    /// Gets the value of ChassisSerialNumber
    pub fn get_chassis_serial_number(&self) -> Option<&String> {
        self.chassis_serial_number.as_ref()
    }

    /// Sets the value of ClusterWideNodeCapabilitiesValidationMode
    pub fn set_cluster_wide_node_capabilities_validation_mode(&mut self, value: VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode) {
        self.cluster_wide_node_capabilities_validation_mode = Some(value);
    }

    /// Gets the value of ClusterWideNodeCapabilitiesValidationMode
    pub fn get_cluster_wide_node_capabilities_validation_mode(&self) -> Option<&VirtualSystemSettingData_ClusterWideNodeCapabilitiesValidationMode> {
        self.cluster_wide_node_capabilities_validation_mode.as_ref()
    }

    /// Sets the value of ConsoleMode
    pub fn set_console_mode(&mut self, value: VirtualSystemSettingData_ConsoleMode) {
        self.console_mode = Some(value);
    }

    /// Gets the value of ConsoleMode
    pub fn get_console_mode(&self) -> Option<&VirtualSystemSettingData_ConsoleMode> {
        self.console_mode.as_ref()
    }

    /// Sets the value of DebugChannelId
    pub fn set_debug_channel_id(&mut self, value: u32) {
        self.debug_channel_id = Some(value);
    }

    /// Gets the value of DebugChannelId
    pub fn get_debug_channel_id(&self) -> Option<&u32> {
        self.debug_channel_id.as_ref()
    }

    /// Sets the value of DebugPort
    pub fn set_debug_port(&mut self, value: u32) {
        self.debug_port = Some(value);
    }

    /// Gets the value of DebugPort
    pub fn get_debug_port(&self) -> Option<&u32> {
        self.debug_port.as_ref()
    }

    /// Sets the value of DebugPortEnabled
    pub fn set_debug_port_enabled(&mut self, value: VirtualSystemSettingData_DebugPortEnabled) {
        self.debug_port_enabled = Some(value);
    }

    /// Gets the value of DebugPortEnabled
    pub fn get_debug_port_enabled(&self) -> Option<&VirtualSystemSettingData_DebugPortEnabled> {
        self.debug_port_enabled.as_ref()
    }

    /// Sets the value of EnableHibernation
    pub fn set_enable_hibernation(&mut self, value: bool) {
        self.enable_hibernation = Some(value);
    }

    /// Gets the value of EnableHibernation
    pub fn get_enable_hibernation(&self) -> Option<&bool> {
        self.enable_hibernation.as_ref()
    }

    /// Sets the value of EnhancedSessionTransportType
    pub fn set_enhanced_session_transport_type(&mut self, value: u16) {
        self.enhanced_session_transport_type = Some(value);
    }

    /// Gets the value of EnhancedSessionTransportType
    pub fn get_enhanced_session_transport_type(&self) -> Option<&u16> {
        self.enhanced_session_transport_type.as_ref()
    }

    /// Sets the value of FirmwareFile
    pub fn set_firmware_file(&mut self, value: String) {
        self.firmware_file = Some(value);
    }

    /// Gets the value of FirmwareFile
    pub fn get_firmware_file(&self) -> Option<&String> {
        self.firmware_file.as_ref()
    }

    /// Sets the value of FirmwareParameters
    pub fn set_firmware_parameters(&mut self, value: Vec<u8>) {
        self.firmware_parameters = value;
    }

    /// Gets the value of FirmwareParameters
    pub fn get_firmware_parameters(&self) -> &Vec<u8> {
        &self.firmware_parameters
    }

    /// Sets the value of GuestControlledCacheTypes
    pub fn set_guest_controlled_cache_types(&mut self, value: bool) {
        self.guest_controlled_cache_types = Some(value);
    }

    /// Gets the value of GuestControlledCacheTypes
    pub fn get_guest_controlled_cache_types(&self) -> Option<&bool> {
        self.guest_controlled_cache_types.as_ref()
    }

    /// Sets the value of GuestFeatureSet
    pub fn set_guest_feature_set(&mut self, value: u64) {
        self.guest_feature_set = Some(value);
    }

    /// Gets the value of GuestFeatureSet
    pub fn get_guest_feature_set(&self) -> Option<&u64> {
        self.guest_feature_set.as_ref()
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

    /// Sets the value of GuestStateIsolationEnabled
    pub fn set_guest_state_isolation_enabled(&mut self, value: bool) {
        self.guest_state_isolation_enabled = Some(value);
    }

    /// Gets the value of GuestStateIsolationEnabled
    pub fn get_guest_state_isolation_enabled(&self) -> Option<&bool> {
        self.guest_state_isolation_enabled.as_ref()
    }

    /// Sets the value of GuestStateIsolationMode
    pub fn set_guest_state_isolation_mode(&mut self, value: u16) {
        self.guest_state_isolation_mode = Some(value);
    }

    /// Gets the value of GuestStateIsolationMode
    pub fn get_guest_state_isolation_mode(&self) -> Option<&u16> {
        self.guest_state_isolation_mode.as_ref()
    }

    /// Sets the value of GuestStateIsolationType
    pub fn set_guest_state_isolation_type(&mut self, value: u16) {
        self.guest_state_isolation_type = Some(value);
    }

    /// Gets the value of GuestStateIsolationType
    pub fn get_guest_state_isolation_type(&self) -> Option<&u16> {
        self.guest_state_isolation_type.as_ref()
    }

    /// Sets the value of HighMmioGapBase
    pub fn set_high_mmio_gap_base(&mut self, value: u64) {
        self.high_mmio_gap_base = Some(value);
    }

    /// Gets the value of HighMmioGapBase
    pub fn get_high_mmio_gap_base(&self) -> Option<&u64> {
        self.high_mmio_gap_base.as_ref()
    }

    /// Sets the value of HighMmioGapSize
    pub fn set_high_mmio_gap_size(&mut self, value: u64) {
        self.high_mmio_gap_size = Some(value);
    }

    /// Gets the value of HighMmioGapSize
    pub fn get_high_mmio_gap_size(&self) -> Option<&u64> {
        self.high_mmio_gap_size.as_ref()
    }

    /// Sets the value of IncrementalBackupEnabled
    pub fn set_incremental_backup_enabled(&mut self, value: bool) {
        self.incremental_backup_enabled = Some(value);
    }

    /// Gets the value of IncrementalBackupEnabled
    pub fn get_incremental_backup_enabled(&self) -> Option<&bool> {
        self.incremental_backup_enabled.as_ref()
    }

    /// Sets the value of IsAutomaticSnapshot
    pub fn set_is_automatic_snapshot(&mut self, value: bool) {
        self.is_automatic_snapshot = Some(value);
    }

    /// Gets the value of IsAutomaticSnapshot
    pub fn get_is_automatic_snapshot(&self) -> Option<&bool> {
        self.is_automatic_snapshot.as_ref()
    }

    /// Sets the value of IsSaved
    pub fn set_is_saved(&mut self, value: bool) {
        self.is_saved = Some(value);
    }

    /// Gets the value of IsSaved
    pub fn get_is_saved(&self) -> Option<&bool> {
        self.is_saved.as_ref()
    }

    /// Sets the value of LockOnDisconnect
    pub fn set_lock_on_disconnect(&mut self, value: bool) {
        self.lock_on_disconnect = Some(value);
    }

    /// Gets the value of LockOnDisconnect
    pub fn get_lock_on_disconnect(&self) -> Option<&bool> {
        self.lock_on_disconnect.as_ref()
    }

    /// Sets the value of LowMmioGapSize
    pub fn set_low_mmio_gap_size(&mut self, value: u64) {
        self.low_mmio_gap_size = Some(value);
    }

    /// Gets the value of LowMmioGapSize
    pub fn get_low_mmio_gap_size(&self) -> Option<&u64> {
        self.low_mmio_gap_size.as_ref()
    }

    /// Sets the value of ManagementVtlUpdatePolicy
    pub fn set_management_vtl_update_policy(&mut self, value: u16) {
        self.management_vtl_update_policy = Some(value);
    }

    /// Gets the value of ManagementVtlUpdatePolicy
    pub fn get_management_vtl_update_policy(&self) -> Option<&u16> {
        self.management_vtl_update_policy.as_ref()
    }

    /// Sets the value of MemoryHostingJobObjectName
    pub fn set_memory_hosting_job_object_name(&mut self, value: String) {
        self.memory_hosting_job_object_name = Some(value);
    }

    /// Gets the value of MemoryHostingJobObjectName
    pub fn get_memory_hosting_job_object_name(&self) -> Option<&String> {
        self.memory_hosting_job_object_name.as_ref()
    }

    /// Sets the value of NetworkBootPreferredProtocol
    pub fn set_network_boot_preferred_protocol(&mut self, value: VirtualSystemSettingData_NetworkBootPreferredProtocol) {
        self.network_boot_preferred_protocol = Some(value);
    }

    /// Gets the value of NetworkBootPreferredProtocol
    pub fn get_network_boot_preferred_protocol(&self) -> Option<&VirtualSystemSettingData_NetworkBootPreferredProtocol> {
        self.network_boot_preferred_protocol.as_ref()
    }

    /// Sets the value of NumaNodeTopologyArray
    pub fn set_numa_node_topology_array(&mut self, value: Vec<String>) {
        self.numa_node_topology_array = value;
    }

    /// Gets the value of NumaNodeTopologyArray
    pub fn get_numa_node_topology_array(&self) -> &Vec<String> {
        &self.numa_node_topology_array
    }

    /// Sets the value of Parent
    pub fn set_parent(&mut self, value: String) {
        self.parent = Some(value);
    }

    /// Gets the value of Parent
    pub fn get_parent(&self) -> Option<&String> {
        self.parent.as_ref()
    }

    /// Sets the value of PauseAfterBootFailure
    pub fn set_pause_after_boot_failure(&mut self, value: bool) {
        self.pause_after_boot_failure = Some(value);
    }

    /// Gets the value of PauseAfterBootFailure
    pub fn get_pause_after_boot_failure(&self) -> Option<&bool> {
        self.pause_after_boot_failure.as_ref()
    }

    /// Sets the value of SecureBootEnabled
    pub fn set_secure_boot_enabled(&mut self, value: bool) {
        self.secure_boot_enabled = Some(value);
    }

    /// Gets the value of SecureBootEnabled
    pub fn get_secure_boot_enabled(&self) -> Option<&bool> {
        self.secure_boot_enabled.as_ref()
    }

    /// Sets the value of SecureBootTemplateId
    pub fn set_secure_boot_template_id(&mut self, value: String) {
        self.secure_boot_template_id = Some(value);
    }

    /// Gets the value of SecureBootTemplateId
    pub fn get_secure_boot_template_id(&self) -> Option<&String> {
        self.secure_boot_template_id.as_ref()
    }

    /// Sets the value of SourceGuestStateFile
    pub fn set_source_guest_state_file(&mut self, value: String) {
        self.source_guest_state_file = Some(value);
    }

    /// Gets the value of SourceGuestStateFile
    pub fn get_source_guest_state_file(&self) -> Option<&String> {
        self.source_guest_state_file.as_ref()
    }

    /// Sets the value of TurnOffOnGuestRestart
    pub fn set_turn_off_on_guest_restart(&mut self, value: bool) {
        self.turn_off_on_guest_restart = Some(value);
    }

    /// Gets the value of TurnOffOnGuestRestart
    pub fn get_turn_off_on_guest_restart(&self) -> Option<&bool> {
        self.turn_off_on_guest_restart.as_ref()
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

    /// Sets the value of VirtualNumaEnabled
    pub fn set_virtual_numa_enabled(&mut self, value: bool) {
        self.virtual_numa_enabled = Some(value);
    }

    /// Gets the value of VirtualNumaEnabled
    pub fn get_virtual_numa_enabled(&self) -> Option<&bool> {
        self.virtual_numa_enabled.as_ref()
    }

    /// Sets the value of VirtualSlitType
    pub fn set_virtual_slit_type(&mut self, value: VirtualSystemSettingData_VirtualSlitType) {
        self.virtual_slit_type = Some(value);
    }

    /// Gets the value of VirtualSlitType
    pub fn get_virtual_slit_type(&self) -> Option<&VirtualSystemSettingData_VirtualSlitType> {
        self.virtual_slit_type.as_ref()
    }

    /// Sets the value of VirtualSystemSubType
    pub fn set_virtual_system_sub_type(&mut self, value: VirtualSystemSettingData_VirtualSystemSubType) {
        self.virtual_system_sub_type = Some(value);
    }

    /// Gets the value of VirtualSystemSubType
    pub fn get_virtual_system_sub_type(&self) -> Option<&VirtualSystemSettingData_VirtualSystemSubType> {
        self.virtual_system_sub_type.as_ref()
    }

    /// Sets the value of VMBusMessageRedirection
    pub fn set_vmbus_message_redirection(&mut self, value: u16) {
        self.vmbus_message_redirection = Some(value);
    }

    /// Gets the value of VMBusMessageRedirection
    pub fn get_vmbus_message_redirection(&self) -> Option<&u16> {
        self.vmbus_message_redirection.as_ref()
    }

    /// Sets the value of Vtl2AddressRangeBase
    pub fn set_vtl2_address_range_base(&mut self, value: u64) {
        self.vtl2_address_range_base = Some(value);
    }

    /// Gets the value of Vtl2AddressRangeBase
    pub fn get_vtl2_address_range_base(&self) -> Option<&u64> {
        self.vtl2_address_range_base.as_ref()
    }

    /// Sets the value of Vtl2AddressRangeSize
    pub fn set_vtl2_address_range_size(&mut self, value: u64) {
        self.vtl2_address_range_size = Some(value);
    }

    /// Gets the value of Vtl2AddressRangeSize
    pub fn get_vtl2_address_range_size(&self) -> Option<&u64> {
        self.vtl2_address_range_size.as_ref()
    }

    /// Sets the value of Vtl2AddressSpaceConfigurationMode
    pub fn set_vtl2_address_space_configuration_mode(&mut self, value: VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode) {
        self.vtl2_address_space_configuration_mode = Some(value);
    }

    /// Gets the value of Vtl2AddressSpaceConfigurationMode
    pub fn get_vtl2_address_space_configuration_mode(&self) -> Option<&VirtualSystemSettingData_Vtl2AddressSpaceConfigurationMode> {
        self.vtl2_address_space_configuration_mode.as_ref()
    }

    /// Sets the value of Vtl2MmioAddressRangeSize
    pub fn set_vtl2_mmio_address_range_size(&mut self, value: u64) {
        self.vtl2_mmio_address_range_size = Some(value);
    }

    /// Gets the value of Vtl2MmioAddressRangeSize
    pub fn get_vtl2_mmio_address_range_size(&self) -> Option<&u64> {
        self.vtl2_mmio_address_range_size.as_ref()
    }

    /// Sets the value of WatchdogEnabled
    pub fn set_watchdog_enabled(&mut self, value: bool) {
        self.watchdog_enabled = Some(value);
    }

    /// Gets the value of WatchdogEnabled
    pub fn get_watchdog_enabled(&self) -> Option<&bool> {
        self.watchdog_enabled.as_ref()
    }

    /// Sets the value of WorkerJobObjectName
    pub fn set_worker_job_object_name(&mut self, value: String) {
        self.worker_job_object_name = Some(value);
    }

    /// Gets the value of WorkerJobObjectName
    pub fn get_worker_job_object_name(&self) -> Option<&String> {
        self.worker_job_object_name.as_ref()
    }
}

