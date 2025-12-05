// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystem {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "AutomaticClusteringEnabled")]
    pub automatic_clustering_enabled: Option<bool>,

/// 
    #[serde(rename = "CimServerName")]
    pub cim_server_name: Option<String>,

/// 
    #[serde(rename = "CurrentCacheLevel")]
    pub current_cache_level: Option<u16>,

/// 
    #[serde(rename = "DataTieringType")]
    pub data_tiering_type: Option<u16>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FaultDomainAwarenessDefault")]
    pub fault_domain_awareness_default: Option<u16>,

/// 
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "iSCSITargetCreationScheme")]
    pub i_scsitarget_creation_scheme: Option<u16>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "MaskingClientSelectableDeviceNumbers")]
    pub masking_client_selectable_device_numbers: Option<bool>,

/// 
    #[serde(rename = "MaskingMapCountMax")]
    pub masking_map_count_max: Option<u16>,

/// 
    #[serde(rename = "MaskingOneInitiatorIdPerView")]
    pub masking_one_initiator_id_per_view: Option<bool>,

/// 
    #[serde(rename = "MaskingOtherValidInitiatorIdTypes")]
    pub masking_other_valid_initiator_id_types: Vec<String>,

/// 
    #[serde(rename = "MaskingPortsPerView")]
    pub masking_ports_per_view: Option<u16>,

/// 
    #[serde(rename = "MaskingValidInitiatorIdTypes")]
    pub masking_valid_initiator_id_types: Vec<u16>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NameFormat")]
    pub name_format: Option<u16>,

/// 
    #[serde(rename = "NumberOfSlots")]
    pub number_of_slots: Option<u32>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherHostTypeDescription")]
    pub other_host_type_description: Vec<String>,

/// 
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// 
    #[serde(rename = "OtherIdentifyingInfoDescription")]
    pub other_identifying_info_description: Vec<String>,

/// 
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// 
    #[serde(rename = "PhysicalDisksPerStoragePoolMin")]
    pub physical_disks_per_storage_pool_min: Option<u16>,

/// 
    #[serde(rename = "ReplicasPerSourceCloneMax")]
    pub replicas_per_source_clone_max: Option<u16>,

/// 
    #[serde(rename = "ReplicasPerSourceMirrorMax")]
    pub replicas_per_source_mirror_max: Option<u16>,

/// 
    #[serde(rename = "ReplicasPerSourceSnapshotMax")]
    pub replicas_per_source_snapshot_max: Option<u16>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "StorageConnectionType")]
    pub storage_connection_type: Option<u16>,

/// 
    #[serde(rename = "SupportedDeduplicationFileSystemTypes")]
    pub supported_deduplication_file_system_types: Vec<u16>,

/// 
    #[serde(rename = "SupportedDeduplicationObjectTypes")]
    pub supported_deduplication_object_types: Vec<u16>,

/// 
    #[serde(rename = "SupportedFileServerProtocols")]
    pub supported_file_server_protocols: Vec<u16>,

/// 
    #[serde(rename = "SupportedFileSystems")]
    pub supported_file_systems: Vec<u16>,

/// 
    #[serde(rename = "SupportedHostType")]
    pub supported_host_type: Vec<u16>,

/// 
    #[serde(rename = "SupportsAutomaticStoragePoolSelection")]
    pub supports_automatic_storage_pool_selection: Option<bool>,

/// 
    #[serde(rename = "SupportsCloneLocal")]
    pub supports_clone_local: Option<bool>,

/// 
    #[serde(rename = "SupportsCloneRemote")]
    pub supports_clone_remote: Option<bool>,

/// 
    #[serde(rename = "SupportsContinuouslyAvailableFileServer")]
    pub supports_continuously_available_file_server: Option<bool>,

/// 
    #[serde(rename = "SupportsFileServer")]
    pub supports_file_server: Option<bool>,

/// 
    #[serde(rename = "SupportsFileServerCreation")]
    pub supports_file_server_creation: Option<bool>,

/// 
    #[serde(rename = "SupportsMaskingVirtualDiskToHosts")]
    pub supports_masking_virtual_disk_to_hosts: Option<bool>,

/// 
    #[serde(rename = "SupportsMirrorLocal")]
    pub supports_mirror_local: Option<bool>,

/// 
    #[serde(rename = "SupportsMirrorRemote")]
    pub supports_mirror_remote: Option<bool>,

/// 
    #[serde(rename = "SupportsMultipleResiliencySettingsPerStoragePool")]
    pub supports_multiple_resiliency_settings_per_storage_pool: Option<bool>,

/// 
    #[serde(rename = "SupportsSnapshotLocal")]
    pub supports_snapshot_local: Option<bool>,

/// 
    #[serde(rename = "SupportsSnapshotRemote")]
    pub supports_snapshot_remote: Option<bool>,

/// 
    #[serde(rename = "SupportsStoragePoolAddPhysicalDisk")]
    pub supports_storage_pool_add_physical_disk: Option<bool>,

/// 
    #[serde(rename = "SupportsStoragePoolCreation")]
    pub supports_storage_pool_creation: Option<bool>,

/// 
    #[serde(rename = "SupportsStoragePoolDeletion")]
    pub supports_storage_pool_deletion: Option<bool>,

/// 
    #[serde(rename = "SupportsStoragePoolFriendlyNameModification")]
    pub supports_storage_pool_friendly_name_modification: Option<bool>,

/// 
    #[serde(rename = "SupportsStoragePoolRemovePhysicalDisk")]
    pub supports_storage_pool_remove_physical_disk: Option<bool>,

/// 
    #[serde(rename = "SupportsStorageTierCreation")]
    pub supports_storage_tier_creation: Option<bool>,

/// 
    #[serde(rename = "SupportsStorageTierDeletion")]
    pub supports_storage_tier_deletion: Option<bool>,

/// 
    #[serde(rename = "SupportsStorageTieredVirtualDiskCreation")]
    pub supports_storage_tiered_virtual_disk_creation: Option<bool>,

/// 
    #[serde(rename = "SupportsStorageTierFriendlyNameModification")]
    pub supports_storage_tier_friendly_name_modification: Option<bool>,

/// 
    #[serde(rename = "SupportsStorageTierResize")]
    pub supports_storage_tier_resize: Option<bool>,

/// 
    #[serde(rename = "SupportsVirtualDiskCapacityExpansion")]
    pub supports_virtual_disk_capacity_expansion: Option<bool>,

/// 
    #[serde(rename = "SupportsVirtualDiskCapacityReduction")]
    pub supports_virtual_disk_capacity_reduction: Option<bool>,

/// 
    #[serde(rename = "SupportsVirtualDiskCreation")]
    pub supports_virtual_disk_creation: Option<bool>,

/// 
    #[serde(rename = "SupportsVirtualDiskDeletion")]
    pub supports_virtual_disk_deletion: Option<bool>,

/// 
    #[serde(rename = "SupportsVirtualDiskModification")]
    pub supports_virtual_disk_modification: Option<bool>,

/// 
    #[serde(rename = "SupportsVirtualDiskRepair")]
    pub supports_virtual_disk_repair: Option<bool>,

/// 
    #[serde(rename = "SupportsVolumeCreation")]
    pub supports_volume_creation: Option<bool>,

/// 
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

/// 
    #[serde(rename = "VirtualDiskRepairEnabled")]
    pub virtual_disk_repair_enabled: Option<bool>,

/// 
    #[serde(rename = "VirtualDiskRepairQueueDepth")]
    pub virtual_disk_repair_queue_depth: Option<u32>,
}

impl MSFT_StorageSubSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            automatic_clustering_enabled: None,
            cim_server_name: None,
            current_cache_level: None,
            data_tiering_type: None,
            description: None,
            fault_domain_awareness_default: None,
            firmware_version: None,
            friendly_name: None,
            health_status: None,
            i_scsitarget_creation_scheme: None,
            manufacturer: None,
            masking_client_selectable_device_numbers: None,
            masking_map_count_max: None,
            masking_one_initiator_id_per_view: None,
            masking_other_valid_initiator_id_types: Vec::new(),
            masking_ports_per_view: None,
            masking_valid_initiator_id_types: Vec::new(),
            model: None,
            name: None,
            name_format: None,
            number_of_slots: None,
            operational_status: Vec::new(),
            other_host_type_description: Vec::new(),
            other_identifying_info: Vec::new(),
            other_identifying_info_description: Vec::new(),
            other_operational_status_description: None,
            physical_disks_per_storage_pool_min: None,
            replicas_per_source_clone_max: None,
            replicas_per_source_mirror_max: None,
            replicas_per_source_snapshot_max: None,
            serial_number: None,
            storage_connection_type: None,
            supported_deduplication_file_system_types: Vec::new(),
            supported_deduplication_object_types: Vec::new(),
            supported_file_server_protocols: Vec::new(),
            supported_file_systems: Vec::new(),
            supported_host_type: Vec::new(),
            supports_automatic_storage_pool_selection: None,
            supports_clone_local: None,
            supports_clone_remote: None,
            supports_continuously_available_file_server: None,
            supports_file_server: None,
            supports_file_server_creation: None,
            supports_masking_virtual_disk_to_hosts: None,
            supports_mirror_local: None,
            supports_mirror_remote: None,
            supports_multiple_resiliency_settings_per_storage_pool: None,
            supports_snapshot_local: None,
            supports_snapshot_remote: None,
            supports_storage_pool_add_physical_disk: None,
            supports_storage_pool_creation: None,
            supports_storage_pool_deletion: None,
            supports_storage_pool_friendly_name_modification: None,
            supports_storage_pool_remove_physical_disk: None,
            supports_storage_tier_creation: None,
            supports_storage_tier_deletion: None,
            supports_storage_tiered_virtual_disk_creation: None,
            supports_storage_tier_friendly_name_modification: None,
            supports_storage_tier_resize: None,
            supports_virtual_disk_capacity_expansion: None,
            supports_virtual_disk_capacity_reduction: None,
            supports_virtual_disk_creation: None,
            supports_virtual_disk_deletion: None,
            supports_virtual_disk_modification: None,
            supports_virtual_disk_repair: None,
            supports_volume_creation: None,
            tag: None,
            virtual_disk_repair_enabled: None,
            virtual_disk_repair_queue_depth: None,
        }
    }


    /// Sets the value of AutomaticClusteringEnabled
    pub fn set_automatic_clustering_enabled(&mut self, value: bool) {
        self.automatic_clustering_enabled = Some(value);
    }

    /// Gets the value of AutomaticClusteringEnabled
    pub fn get_automatic_clustering_enabled(&self) -> Option<&bool> {
        self.automatic_clustering_enabled.as_ref()
    }

    /// Sets the value of CimServerName
    pub fn set_cim_server_name(&mut self, value: String) {
        self.cim_server_name = Some(value);
    }

    /// Gets the value of CimServerName
    pub fn get_cim_server_name(&self) -> Option<&String> {
        self.cim_server_name.as_ref()
    }

    /// Sets the value of CurrentCacheLevel
    pub fn set_current_cache_level(&mut self, value: u16) {
        self.current_cache_level = Some(value);
    }

    /// Gets the value of CurrentCacheLevel
    pub fn get_current_cache_level(&self) -> Option<&u16> {
        self.current_cache_level.as_ref()
    }

    /// Sets the value of DataTieringType
    pub fn set_data_tiering_type(&mut self, value: u16) {
        self.data_tiering_type = Some(value);
    }

    /// Gets the value of DataTieringType
    pub fn get_data_tiering_type(&self) -> Option<&u16> {
        self.data_tiering_type.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of FaultDomainAwarenessDefault
    pub fn set_fault_domain_awareness_default(&mut self, value: u16) {
        self.fault_domain_awareness_default = Some(value);
    }

    /// Gets the value of FaultDomainAwarenessDefault
    pub fn get_fault_domain_awareness_default(&self) -> Option<&u16> {
        self.fault_domain_awareness_default.as_ref()
    }

    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
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
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of iSCSITargetCreationScheme
    pub fn set_i_scsitarget_creation_scheme(&mut self, value: u16) {
        self.i_scsitarget_creation_scheme = Some(value);
    }

    /// Gets the value of iSCSITargetCreationScheme
    pub fn get_i_scsitarget_creation_scheme(&self) -> Option<&u16> {
        self.i_scsitarget_creation_scheme.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of MaskingClientSelectableDeviceNumbers
    pub fn set_masking_client_selectable_device_numbers(&mut self, value: bool) {
        self.masking_client_selectable_device_numbers = Some(value);
    }

    /// Gets the value of MaskingClientSelectableDeviceNumbers
    pub fn get_masking_client_selectable_device_numbers(&self) -> Option<&bool> {
        self.masking_client_selectable_device_numbers.as_ref()
    }

    /// Sets the value of MaskingMapCountMax
    pub fn set_masking_map_count_max(&mut self, value: u16) {
        self.masking_map_count_max = Some(value);
    }

    /// Gets the value of MaskingMapCountMax
    pub fn get_masking_map_count_max(&self) -> Option<&u16> {
        self.masking_map_count_max.as_ref()
    }

    /// Sets the value of MaskingOneInitiatorIdPerView
    pub fn set_masking_one_initiator_id_per_view(&mut self, value: bool) {
        self.masking_one_initiator_id_per_view = Some(value);
    }

    /// Gets the value of MaskingOneInitiatorIdPerView
    pub fn get_masking_one_initiator_id_per_view(&self) -> Option<&bool> {
        self.masking_one_initiator_id_per_view.as_ref()
    }

    /// Sets the value of MaskingOtherValidInitiatorIdTypes
    pub fn set_masking_other_valid_initiator_id_types(&mut self, value: Vec<String>) {
        self.masking_other_valid_initiator_id_types = value;
    }

    /// Gets the value of MaskingOtherValidInitiatorIdTypes
    pub fn get_masking_other_valid_initiator_id_types(&self) -> &Vec<String> {
        &self.masking_other_valid_initiator_id_types
    }

    /// Sets the value of MaskingPortsPerView
    pub fn set_masking_ports_per_view(&mut self, value: u16) {
        self.masking_ports_per_view = Some(value);
    }

    /// Gets the value of MaskingPortsPerView
    pub fn get_masking_ports_per_view(&self) -> Option<&u16> {
        self.masking_ports_per_view.as_ref()
    }

    /// Sets the value of MaskingValidInitiatorIdTypes
    pub fn set_masking_valid_initiator_id_types(&mut self, value: Vec<u16>) {
        self.masking_valid_initiator_id_types = value;
    }

    /// Gets the value of MaskingValidInitiatorIdTypes
    pub fn get_masking_valid_initiator_id_types(&self) -> &Vec<u16> {
        &self.masking_valid_initiator_id_types
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NameFormat
    pub fn set_name_format(&mut self, value: u16) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&u16> {
        self.name_format.as_ref()
    }

    /// Sets the value of NumberOfSlots
    pub fn set_number_of_slots(&mut self, value: u32) {
        self.number_of_slots = Some(value);
    }

    /// Gets the value of NumberOfSlots
    pub fn get_number_of_slots(&self) -> Option<&u32> {
        self.number_of_slots.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of OtherHostTypeDescription
    pub fn set_other_host_type_description(&mut self, value: Vec<String>) {
        self.other_host_type_description = value;
    }

    /// Gets the value of OtherHostTypeDescription
    pub fn get_other_host_type_description(&self) -> &Vec<String> {
        &self.other_host_type_description
    }

    /// Sets the value of OtherIdentifyingInfo
    pub fn set_other_identifying_info(&mut self, value: Vec<String>) {
        self.other_identifying_info = value;
    }

    /// Gets the value of OtherIdentifyingInfo
    pub fn get_other_identifying_info(&self) -> &Vec<String> {
        &self.other_identifying_info
    }

    /// Sets the value of OtherIdentifyingInfoDescription
    pub fn set_other_identifying_info_description(&mut self, value: Vec<String>) {
        self.other_identifying_info_description = value;
    }

    /// Gets the value of OtherIdentifyingInfoDescription
    pub fn get_other_identifying_info_description(&self) -> &Vec<String> {
        &self.other_identifying_info_description
    }

    /// Sets the value of OtherOperationalStatusDescription
    pub fn set_other_operational_status_description(&mut self, value: String) {
        self.other_operational_status_description = Some(value);
    }

    /// Gets the value of OtherOperationalStatusDescription
    pub fn get_other_operational_status_description(&self) -> Option<&String> {
        self.other_operational_status_description.as_ref()
    }

    /// Sets the value of PhysicalDisksPerStoragePoolMin
    pub fn set_physical_disks_per_storage_pool_min(&mut self, value: u16) {
        self.physical_disks_per_storage_pool_min = Some(value);
    }

    /// Gets the value of PhysicalDisksPerStoragePoolMin
    pub fn get_physical_disks_per_storage_pool_min(&self) -> Option<&u16> {
        self.physical_disks_per_storage_pool_min.as_ref()
    }

    /// Sets the value of ReplicasPerSourceCloneMax
    pub fn set_replicas_per_source_clone_max(&mut self, value: u16) {
        self.replicas_per_source_clone_max = Some(value);
    }

    /// Gets the value of ReplicasPerSourceCloneMax
    pub fn get_replicas_per_source_clone_max(&self) -> Option<&u16> {
        self.replicas_per_source_clone_max.as_ref()
    }

    /// Sets the value of ReplicasPerSourceMirrorMax
    pub fn set_replicas_per_source_mirror_max(&mut self, value: u16) {
        self.replicas_per_source_mirror_max = Some(value);
    }

    /// Gets the value of ReplicasPerSourceMirrorMax
    pub fn get_replicas_per_source_mirror_max(&self) -> Option<&u16> {
        self.replicas_per_source_mirror_max.as_ref()
    }

    /// Sets the value of ReplicasPerSourceSnapshotMax
    pub fn set_replicas_per_source_snapshot_max(&mut self, value: u16) {
        self.replicas_per_source_snapshot_max = Some(value);
    }

    /// Gets the value of ReplicasPerSourceSnapshotMax
    pub fn get_replicas_per_source_snapshot_max(&self) -> Option<&u16> {
        self.replicas_per_source_snapshot_max.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of StorageConnectionType
    pub fn set_storage_connection_type(&mut self, value: u16) {
        self.storage_connection_type = Some(value);
    }

    /// Gets the value of StorageConnectionType
    pub fn get_storage_connection_type(&self) -> Option<&u16> {
        self.storage_connection_type.as_ref()
    }

    /// Sets the value of SupportedDeduplicationFileSystemTypes
    pub fn set_supported_deduplication_file_system_types(&mut self, value: Vec<u16>) {
        self.supported_deduplication_file_system_types = value;
    }

    /// Gets the value of SupportedDeduplicationFileSystemTypes
    pub fn get_supported_deduplication_file_system_types(&self) -> &Vec<u16> {
        &self.supported_deduplication_file_system_types
    }

    /// Sets the value of SupportedDeduplicationObjectTypes
    pub fn set_supported_deduplication_object_types(&mut self, value: Vec<u16>) {
        self.supported_deduplication_object_types = value;
    }

    /// Gets the value of SupportedDeduplicationObjectTypes
    pub fn get_supported_deduplication_object_types(&self) -> &Vec<u16> {
        &self.supported_deduplication_object_types
    }

    /// Sets the value of SupportedFileServerProtocols
    pub fn set_supported_file_server_protocols(&mut self, value: Vec<u16>) {
        self.supported_file_server_protocols = value;
    }

    /// Gets the value of SupportedFileServerProtocols
    pub fn get_supported_file_server_protocols(&self) -> &Vec<u16> {
        &self.supported_file_server_protocols
    }

    /// Sets the value of SupportedFileSystems
    pub fn set_supported_file_systems(&mut self, value: Vec<u16>) {
        self.supported_file_systems = value;
    }

    /// Gets the value of SupportedFileSystems
    pub fn get_supported_file_systems(&self) -> &Vec<u16> {
        &self.supported_file_systems
    }

    /// Sets the value of SupportedHostType
    pub fn set_supported_host_type(&mut self, value: Vec<u16>) {
        self.supported_host_type = value;
    }

    /// Gets the value of SupportedHostType
    pub fn get_supported_host_type(&self) -> &Vec<u16> {
        &self.supported_host_type
    }

    /// Sets the value of SupportsAutomaticStoragePoolSelection
    pub fn set_supports_automatic_storage_pool_selection(&mut self, value: bool) {
        self.supports_automatic_storage_pool_selection = Some(value);
    }

    /// Gets the value of SupportsAutomaticStoragePoolSelection
    pub fn get_supports_automatic_storage_pool_selection(&self) -> Option<&bool> {
        self.supports_automatic_storage_pool_selection.as_ref()
    }

    /// Sets the value of SupportsCloneLocal
    pub fn set_supports_clone_local(&mut self, value: bool) {
        self.supports_clone_local = Some(value);
    }

    /// Gets the value of SupportsCloneLocal
    pub fn get_supports_clone_local(&self) -> Option<&bool> {
        self.supports_clone_local.as_ref()
    }

    /// Sets the value of SupportsCloneRemote
    pub fn set_supports_clone_remote(&mut self, value: bool) {
        self.supports_clone_remote = Some(value);
    }

    /// Gets the value of SupportsCloneRemote
    pub fn get_supports_clone_remote(&self) -> Option<&bool> {
        self.supports_clone_remote.as_ref()
    }

    /// Sets the value of SupportsContinuouslyAvailableFileServer
    pub fn set_supports_continuously_available_file_server(&mut self, value: bool) {
        self.supports_continuously_available_file_server = Some(value);
    }

    /// Gets the value of SupportsContinuouslyAvailableFileServer
    pub fn get_supports_continuously_available_file_server(&self) -> Option<&bool> {
        self.supports_continuously_available_file_server.as_ref()
    }

    /// Sets the value of SupportsFileServer
    pub fn set_supports_file_server(&mut self, value: bool) {
        self.supports_file_server = Some(value);
    }

    /// Gets the value of SupportsFileServer
    pub fn get_supports_file_server(&self) -> Option<&bool> {
        self.supports_file_server.as_ref()
    }

    /// Sets the value of SupportsFileServerCreation
    pub fn set_supports_file_server_creation(&mut self, value: bool) {
        self.supports_file_server_creation = Some(value);
    }

    /// Gets the value of SupportsFileServerCreation
    pub fn get_supports_file_server_creation(&self) -> Option<&bool> {
        self.supports_file_server_creation.as_ref()
    }

    /// Sets the value of SupportsMaskingVirtualDiskToHosts
    pub fn set_supports_masking_virtual_disk_to_hosts(&mut self, value: bool) {
        self.supports_masking_virtual_disk_to_hosts = Some(value);
    }

    /// Gets the value of SupportsMaskingVirtualDiskToHosts
    pub fn get_supports_masking_virtual_disk_to_hosts(&self) -> Option<&bool> {
        self.supports_masking_virtual_disk_to_hosts.as_ref()
    }

    /// Sets the value of SupportsMirrorLocal
    pub fn set_supports_mirror_local(&mut self, value: bool) {
        self.supports_mirror_local = Some(value);
    }

    /// Gets the value of SupportsMirrorLocal
    pub fn get_supports_mirror_local(&self) -> Option<&bool> {
        self.supports_mirror_local.as_ref()
    }

    /// Sets the value of SupportsMirrorRemote
    pub fn set_supports_mirror_remote(&mut self, value: bool) {
        self.supports_mirror_remote = Some(value);
    }

    /// Gets the value of SupportsMirrorRemote
    pub fn get_supports_mirror_remote(&self) -> Option<&bool> {
        self.supports_mirror_remote.as_ref()
    }

    /// Sets the value of SupportsMultipleResiliencySettingsPerStoragePool
    pub fn set_supports_multiple_resiliency_settings_per_storage_pool(&mut self, value: bool) {
        self.supports_multiple_resiliency_settings_per_storage_pool = Some(value);
    }

    /// Gets the value of SupportsMultipleResiliencySettingsPerStoragePool
    pub fn get_supports_multiple_resiliency_settings_per_storage_pool(&self) -> Option<&bool> {
        self.supports_multiple_resiliency_settings_per_storage_pool.as_ref()
    }

    /// Sets the value of SupportsSnapshotLocal
    pub fn set_supports_snapshot_local(&mut self, value: bool) {
        self.supports_snapshot_local = Some(value);
    }

    /// Gets the value of SupportsSnapshotLocal
    pub fn get_supports_snapshot_local(&self) -> Option<&bool> {
        self.supports_snapshot_local.as_ref()
    }

    /// Sets the value of SupportsSnapshotRemote
    pub fn set_supports_snapshot_remote(&mut self, value: bool) {
        self.supports_snapshot_remote = Some(value);
    }

    /// Gets the value of SupportsSnapshotRemote
    pub fn get_supports_snapshot_remote(&self) -> Option<&bool> {
        self.supports_snapshot_remote.as_ref()
    }

    /// Sets the value of SupportsStoragePoolAddPhysicalDisk
    pub fn set_supports_storage_pool_add_physical_disk(&mut self, value: bool) {
        self.supports_storage_pool_add_physical_disk = Some(value);
    }

    /// Gets the value of SupportsStoragePoolAddPhysicalDisk
    pub fn get_supports_storage_pool_add_physical_disk(&self) -> Option<&bool> {
        self.supports_storage_pool_add_physical_disk.as_ref()
    }

    /// Sets the value of SupportsStoragePoolCreation
    pub fn set_supports_storage_pool_creation(&mut self, value: bool) {
        self.supports_storage_pool_creation = Some(value);
    }

    /// Gets the value of SupportsStoragePoolCreation
    pub fn get_supports_storage_pool_creation(&self) -> Option<&bool> {
        self.supports_storage_pool_creation.as_ref()
    }

    /// Sets the value of SupportsStoragePoolDeletion
    pub fn set_supports_storage_pool_deletion(&mut self, value: bool) {
        self.supports_storage_pool_deletion = Some(value);
    }

    /// Gets the value of SupportsStoragePoolDeletion
    pub fn get_supports_storage_pool_deletion(&self) -> Option<&bool> {
        self.supports_storage_pool_deletion.as_ref()
    }

    /// Sets the value of SupportsStoragePoolFriendlyNameModification
    pub fn set_supports_storage_pool_friendly_name_modification(&mut self, value: bool) {
        self.supports_storage_pool_friendly_name_modification = Some(value);
    }

    /// Gets the value of SupportsStoragePoolFriendlyNameModification
    pub fn get_supports_storage_pool_friendly_name_modification(&self) -> Option<&bool> {
        self.supports_storage_pool_friendly_name_modification.as_ref()
    }

    /// Sets the value of SupportsStoragePoolRemovePhysicalDisk
    pub fn set_supports_storage_pool_remove_physical_disk(&mut self, value: bool) {
        self.supports_storage_pool_remove_physical_disk = Some(value);
    }

    /// Gets the value of SupportsStoragePoolRemovePhysicalDisk
    pub fn get_supports_storage_pool_remove_physical_disk(&self) -> Option<&bool> {
        self.supports_storage_pool_remove_physical_disk.as_ref()
    }

    /// Sets the value of SupportsStorageTierCreation
    pub fn set_supports_storage_tier_creation(&mut self, value: bool) {
        self.supports_storage_tier_creation = Some(value);
    }

    /// Gets the value of SupportsStorageTierCreation
    pub fn get_supports_storage_tier_creation(&self) -> Option<&bool> {
        self.supports_storage_tier_creation.as_ref()
    }

    /// Sets the value of SupportsStorageTierDeletion
    pub fn set_supports_storage_tier_deletion(&mut self, value: bool) {
        self.supports_storage_tier_deletion = Some(value);
    }

    /// Gets the value of SupportsStorageTierDeletion
    pub fn get_supports_storage_tier_deletion(&self) -> Option<&bool> {
        self.supports_storage_tier_deletion.as_ref()
    }

    /// Sets the value of SupportsStorageTieredVirtualDiskCreation
    pub fn set_supports_storage_tiered_virtual_disk_creation(&mut self, value: bool) {
        self.supports_storage_tiered_virtual_disk_creation = Some(value);
    }

    /// Gets the value of SupportsStorageTieredVirtualDiskCreation
    pub fn get_supports_storage_tiered_virtual_disk_creation(&self) -> Option<&bool> {
        self.supports_storage_tiered_virtual_disk_creation.as_ref()
    }

    /// Sets the value of SupportsStorageTierFriendlyNameModification
    pub fn set_supports_storage_tier_friendly_name_modification(&mut self, value: bool) {
        self.supports_storage_tier_friendly_name_modification = Some(value);
    }

    /// Gets the value of SupportsStorageTierFriendlyNameModification
    pub fn get_supports_storage_tier_friendly_name_modification(&self) -> Option<&bool> {
        self.supports_storage_tier_friendly_name_modification.as_ref()
    }

    /// Sets the value of SupportsStorageTierResize
    pub fn set_supports_storage_tier_resize(&mut self, value: bool) {
        self.supports_storage_tier_resize = Some(value);
    }

    /// Gets the value of SupportsStorageTierResize
    pub fn get_supports_storage_tier_resize(&self) -> Option<&bool> {
        self.supports_storage_tier_resize.as_ref()
    }

    /// Sets the value of SupportsVirtualDiskCapacityExpansion
    pub fn set_supports_virtual_disk_capacity_expansion(&mut self, value: bool) {
        self.supports_virtual_disk_capacity_expansion = Some(value);
    }

    /// Gets the value of SupportsVirtualDiskCapacityExpansion
    pub fn get_supports_virtual_disk_capacity_expansion(&self) -> Option<&bool> {
        self.supports_virtual_disk_capacity_expansion.as_ref()
    }

    /// Sets the value of SupportsVirtualDiskCapacityReduction
    pub fn set_supports_virtual_disk_capacity_reduction(&mut self, value: bool) {
        self.supports_virtual_disk_capacity_reduction = Some(value);
    }

    /// Gets the value of SupportsVirtualDiskCapacityReduction
    pub fn get_supports_virtual_disk_capacity_reduction(&self) -> Option<&bool> {
        self.supports_virtual_disk_capacity_reduction.as_ref()
    }

    /// Sets the value of SupportsVirtualDiskCreation
    pub fn set_supports_virtual_disk_creation(&mut self, value: bool) {
        self.supports_virtual_disk_creation = Some(value);
    }

    /// Gets the value of SupportsVirtualDiskCreation
    pub fn get_supports_virtual_disk_creation(&self) -> Option<&bool> {
        self.supports_virtual_disk_creation.as_ref()
    }

    /// Sets the value of SupportsVirtualDiskDeletion
    pub fn set_supports_virtual_disk_deletion(&mut self, value: bool) {
        self.supports_virtual_disk_deletion = Some(value);
    }

    /// Gets the value of SupportsVirtualDiskDeletion
    pub fn get_supports_virtual_disk_deletion(&self) -> Option<&bool> {
        self.supports_virtual_disk_deletion.as_ref()
    }

    /// Sets the value of SupportsVirtualDiskModification
    pub fn set_supports_virtual_disk_modification(&mut self, value: bool) {
        self.supports_virtual_disk_modification = Some(value);
    }

    /// Gets the value of SupportsVirtualDiskModification
    pub fn get_supports_virtual_disk_modification(&self) -> Option<&bool> {
        self.supports_virtual_disk_modification.as_ref()
    }

    /// Sets the value of SupportsVirtualDiskRepair
    pub fn set_supports_virtual_disk_repair(&mut self, value: bool) {
        self.supports_virtual_disk_repair = Some(value);
    }

    /// Gets the value of SupportsVirtualDiskRepair
    pub fn get_supports_virtual_disk_repair(&self) -> Option<&bool> {
        self.supports_virtual_disk_repair.as_ref()
    }

    /// Sets the value of SupportsVolumeCreation
    pub fn set_supports_volume_creation(&mut self, value: bool) {
        self.supports_volume_creation = Some(value);
    }

    /// Gets the value of SupportsVolumeCreation
    pub fn get_supports_volume_creation(&self) -> Option<&bool> {
        self.supports_volume_creation.as_ref()
    }

    /// Sets the value of Tag
    pub fn set_tag(&mut self, value: String) {
        self.tag = Some(value);
    }

    /// Gets the value of Tag
    pub fn get_tag(&self) -> Option<&String> {
        self.tag.as_ref()
    }

    /// Sets the value of VirtualDiskRepairEnabled
    pub fn set_virtual_disk_repair_enabled(&mut self, value: bool) {
        self.virtual_disk_repair_enabled = Some(value);
    }

    /// Gets the value of VirtualDiskRepairEnabled
    pub fn get_virtual_disk_repair_enabled(&self) -> Option<&bool> {
        self.virtual_disk_repair_enabled.as_ref()
    }

    /// Sets the value of VirtualDiskRepairQueueDepth
    pub fn set_virtual_disk_repair_queue_depth(&mut self, value: u32) {
        self.virtual_disk_repair_queue_depth = Some(value);
    }

    /// Gets the value of VirtualDiskRepairQueueDepth
    pub fn get_virtual_disk_repair_queue_depth(&self) -> Option<&u32> {
        self.virtual_disk_repair_queue_depth.as_ref()
    }

/// 

    /// * `auto_write_cache_size` -  (bool)
    /// * `enclosure_aware_default` -  (bool)
    /// * `fault_domain_awareness_default` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `logical_sector_size_default` -  (u64)
    /// * `media_type_default` -  (u16)
    /// * `metadata_length` -  (u64)
    /// * `minimum_allocation_size` -  (u64)
    /// * `other_usage_description` -  (String)
    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `provisioning_type_default` -  (u16)
    /// * `resiliency_setting_name_default` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `usage` -  (u16)
    /// * `version` -  (u16)
    /// * `write_cache_size_default` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_storage_pool` -  (MSFT_StoragePool)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_pool(&self, friendly_name: &String, usage: u16, other_usage_description: &String, physical_disks: &Vec<MSFT_PhysicalDisk>, resiliency_setting_name_default: &String, provisioning_type_default: u16, media_type_default: u16, logical_sector_size_default: u64, metadata_length: u64, minimum_allocation_size: u64, enclosure_aware_default: bool, fault_domain_awareness_default: u16, write_cache_size_default: u64, auto_write_cache_size: bool, version: u16, run_as_job: bool, created_storage_pool: &mut MSFT_StoragePool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "ResiliencySettingNameDefault".to_string(), value: resiliency_setting_name_default.into() });
        args.push(MethodParameter { name: "ProvisioningTypeDefault".to_string(), value: provisioning_type_default.into() });
        args.push(MethodParameter { name: "MediaTypeDefault".to_string(), value: media_type_default.into() });
        args.push(MethodParameter { name: "LogicalSectorSizeDefault".to_string(), value: logical_sector_size_default.into() });
        args.push(MethodParameter { name: "MetadataLength".to_string(), value: metadata_length.into() });
        args.push(MethodParameter { name: "MinimumAllocationSize".to_string(), value: minimum_allocation_size.into() });
        args.push(MethodParameter { name: "EnclosureAwareDefault".to_string(), value: enclosure_aware_default.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });
        args.push(MethodParameter { name: "Version".to_string(), value: version.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateStoragePool", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_pool = result.get_value("CreatedStoragePool")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `is_enclosure_aware` -  (bool)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `other_usage_description` -  (String)
    /// * `parity_layout` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `request_no_single_point_of_failure` -  (bool)
    /// * `run_as_job` -  (bool)
    /// * `size` -  (u64)
    /// * `usage` -  (u16)
    /// * `use_maximum_size` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `size` -  (u64)
    pub fn create_virtual_disk(&self, friendly_name: &String, usage: u16, other_usage_description: &String, size: &mut u64, use_maximum_size: bool, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, interleave: u64, parity_layout: u16, request_no_single_point_of_failure: bool, is_enclosure_aware: bool, fault_domain_awareness: u16, provisioning_type: u16, run_as_job: bool, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "UseMaximumSize".to_string(), value: use_maximum_size.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "ParityLayout".to_string(), value: parity_layout.into() });
        args.push(MethodParameter { name: "RequestNoSinglePointOfFailure".to_string(), value: request_no_single_point_of_failure.into() });
        args.push(MethodParameter { name: "IsEnclosureAware".to_string(), value: is_enclosure_aware.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let size = result.get_value("Size")?;
        Ok(result.return_value)

    }


/// 

    /// * `device_accesses` -  (u16[])
    /// * `device_numbers` -  (String[])
    /// * `friendly_name` -  (String)
    /// * `host_type` -  (u16)
    /// * `initiator_addresses` -  (String[])
    /// * `run_as_job` -  (bool)
    /// * `target_port_addresses` -  (String[])
    /// * `virtual_disk_names` -  (String[])

    /// * `created_masking_set` -  (MSFT_MaskingSet)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_masking_set(&self, friendly_name: &String, virtual_disk_names: &Vec<String>, device_accesses: &Vec<u16>, device_numbers: &Vec<String>, target_port_addresses: &Vec<String>, initiator_addresses: &Vec<String>, host_type: u16, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, created_masking_set: &mut MSFT_MaskingSet, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "VirtualDiskNames".to_string(), value: virtual_disk_names.into() });
        args.push(MethodParameter { name: "DeviceAccesses".to_string(), value: device_accesses.into() });
        args.push(MethodParameter { name: "DeviceNumbers".to_string(), value: device_numbers.into() });
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "InitiatorAddresses".to_string(), value: initiator_addresses.into() });
        args.push(MethodParameter { name: "HostType".to_string(), value: host_type.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateMaskingSet", &args)?;
        let created_masking_set = result.get_value("CreatedMaskingSet")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `security_descriptor` -  (String)
    pub fn get_security_descriptor(&self, security_descriptor: &mut String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let security_descriptor = result.get_value("SecurityDescriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `security_descriptor` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, security_descriptor: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });

        let result = self.invoke_method("SetSecurityDescriptor", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_description(&self, description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("SetDescription", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `automatic_clustering_enabled` -  (bool)
    /// * `fault_domain_awareness_default` -  (u16)
    /// * `virtual_disk_repair_enabled` -  (bool)
    /// * `virtual_disk_repair_queue_depth` -  (u32)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, automatic_clustering_enabled: bool, virtual_disk_repair_enabled: bool, virtual_disk_repair_queue_depth: u32, fault_domain_awareness_default: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AutomaticClusteringEnabled".to_string(), value: automatic_clustering_enabled.into() });
        args.push(MethodParameter { name: "VirtualDiskRepairEnabled".to_string(), value: virtual_disk_repair_enabled.into() });
        args.push(MethodParameter { name: "VirtualDiskRepairQueueDepth".to_string(), value: virtual_disk_repair_queue_depth.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `recovery_point_objective` -  (u32)
    /// * `run_as_job` -  (bool)
    /// * `source_group` -  (MSFT_ReplicationGroup)
    /// * `source_group_settings` -  (MSFT_ReplicationSettings)
    /// * `source_replication_group_description` -  (String)
    /// * `source_replication_group_friendly_name` -  (String)
    /// * `source_storage_elements` -  (MSFT_StorageObject[])
    /// * `sync_type` -  (u16)
    /// * `target_group` -  (MSFT_ReplicationGroup)
    /// * `target_group_settings` -  (MSFT_ReplicationSettings)
    /// * `target_replication_group_description` -  (String)
    /// * `target_replication_group_friendly_name` -  (String)
    /// * `target_storage_elements` -  (MSFT_StorageObject[])
    /// * `target_storage_pool` -  (MSFT_StoragePool)
    /// * `target_storage_pools` -  (MSFT_StoragePool[])
    /// * `target_storage_subsystem` -  (MSFT_ReplicaPeer)

    /// * `created_replica_peer` -  (MSFT_ReplicaPeer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `source_group` -  (MSFT_ReplicationGroup)
    pub fn create_replication_relationship(&self, friendly_name: &String, sync_type: u16, target_storage_subsystem: MSFT_ReplicaPeer, source_replication_group_friendly_name: &String, source_replication_group_description: &String, source_storage_elements: &Vec<MSFT_StorageObject>, source_group_settings: MSFT_ReplicationSettings, target_replication_group_friendly_name: &String, target_replication_group_description: &String, target_storage_elements: &Vec<MSFT_StorageObject>, target_storage_pool: MSFT_StoragePool, target_storage_pools: &Vec<MSFT_StoragePool>, target_group_settings: MSFT_ReplicationSettings, recovery_point_objective: u32, run_as_job: bool, source_group: &mut MSFT_ReplicationGroup, target_group: MSFT_ReplicationGroup, created_replica_peer: &mut MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "SyncType".to_string(), value: sync_type.into() });
        args.push(MethodParameter { name: "TargetStorageSubsystem".to_string(), value: target_storage_subsystem.into() });
        args.push(MethodParameter { name: "SourceReplicationGroupFriendlyName".to_string(), value: source_replication_group_friendly_name.into() });
        args.push(MethodParameter { name: "SourceReplicationGroupDescription".to_string(), value: source_replication_group_description.into() });
        args.push(MethodParameter { name: "SourceStorageElements".to_string(), value: source_storage_elements.into() });
        args.push(MethodParameter { name: "SourceGroupSettings".to_string(), value: source_group_settings.into() });
        args.push(MethodParameter { name: "TargetReplicationGroupFriendlyName".to_string(), value: target_replication_group_friendly_name.into() });
        args.push(MethodParameter { name: "TargetReplicationGroupDescription".to_string(), value: target_replication_group_description.into() });
        args.push(MethodParameter { name: "TargetStorageElements".to_string(), value: target_storage_elements.into() });
        args.push(MethodParameter { name: "TargetStoragePool".to_string(), value: target_storage_pool.into() });
        args.push(MethodParameter { name: "TargetStoragePools".to_string(), value: target_storage_pools.into() });
        args.push(MethodParameter { name: "TargetGroupSettings".to_string(), value: target_group_settings.into() });
        args.push(MethodParameter { name: "RecoveryPointObjective".to_string(), value: recovery_point_objective.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });
        args.push(MethodParameter { name: "TargetGroup".to_string(), value: target_group.into() });

        let result = self.invoke_method("CreateReplicationRelationship", &args)?;
        let created_replica_peer = result.get_value("CreatedReplicaPeer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let source_group = result.get_value("SourceGroup")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `source_replication_group` -  (MSFT_ReplicationGroup)
    /// * `target_group_replica_peer` -  (MSFT_ReplicaPeer)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_replication_relationship(&self, source_replication_group: MSFT_ReplicationGroup, target_group_replica_peer: MSFT_ReplicaPeer, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceReplicationGroup".to_string(), value: source_replication_group.into() });
        args.push(MethodParameter { name: "TargetGroupReplicaPeer".to_string(), value: target_group_replica_peer.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("DeleteReplicationRelationship", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `friendly_name` -  (String)
    /// * `replication_settings` -  (MSFT_ReplicationSettings)
    /// * `run_as_job` -  (bool)
    /// * `storage_elements` -  (MSFT_StorageObject[])

    /// * `created_replication_group` -  (MSFT_ReplicationGroup)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_replication_group(&self, friendly_name: &String, description: &String, storage_elements: &Vec<MSFT_StorageObject>, replication_settings: MSFT_ReplicationSettings, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, created_replication_group: &mut MSFT_ReplicationGroup, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "StorageElements".to_string(), value: storage_elements.into() });
        args.push(MethodParameter { name: "ReplicationSettings".to_string(), value: replication_settings.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateReplicationGroup", &args)?;
        let created_replication_group = result.get_value("CreatedReplicationGroup")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `file_sharing_protocols` -  (u16[])
    /// * `friendly_name` -  (String)
    /// * `host_names` -  (String[])
    /// * `run_as_job` -  (bool)

    /// * `created_file_server` -  (MSFT_FileServer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_file_server(&self, friendly_name: &String, file_sharing_protocols: &Vec<u16>, host_names: &Vec<String>, run_as_job: bool, created_file_server: &mut MSFT_FileServer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "FileSharingProtocols".to_string(), value: file_sharing_protocols.into() });
        args.push(MethodParameter { name: "HostNames".to_string(), value: host_names.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateFileServer", &args)?;
        let created_file_server = result.get_value("CreatedFileServer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `activity_id` -  (String)
    /// * `copy_existing_info_only` -  (bool)
    /// * `destination_path` -  (String)
    /// * `exclude_diagnostic_log` -  (bool)
    /// * `exclude_operational_log` -  (bool)
    /// * `include_live_dump` -  (bool)
    /// * `time_span` -  (u32)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_diagnostic_info(&self, destination_path: &String, time_span: u32, activity_id: &String, exclude_operational_log: bool, exclude_diagnostic_log: bool, include_live_dump: bool, copy_existing_info_only: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DestinationPath".to_string(), value: destination_path.into() });
        args.push(MethodParameter { name: "TimeSpan".to_string(), value: time_span.into() });
        args.push(MethodParameter { name: "ActivityId".to_string(), value: activity_id.into() });
        args.push(MethodParameter { name: "ExcludeOperationalLog".to_string(), value: exclude_operational_log.into() });
        args.push(MethodParameter { name: "ExcludeDiagnosticLog".to_string(), value: exclude_diagnostic_log.into() });
        args.push(MethodParameter { name: "IncludeLiveDump".to_string(), value: include_live_dump.into() });
        args.push(MethodParameter { name: "CopyExistingInfoOnly".to_string(), value: copy_existing_info_only.into() });

        let result = self.invoke_method("GetDiagnosticInfo", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn clear_diagnostic_info(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("ClearDiagnosticInfo", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `level` -  (u16)
    /// * `max_log_size` -  (u64)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn start_diagnostic_log(&self, level: u16, max_log_size: u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Level".to_string(), value: level.into() });
        args.push(MethodParameter { name: "MaxLogSize".to_string(), value: max_log_size.into() });

        let result = self.invoke_method("StartDiagnosticLog", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn stop_diagnostic_log(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("StopDiagnosticLog", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `diagnose_results` -  (MSFT_StorageDiagnoseResult[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn diagnose(&self, diagnose_results: &mut Vec<MSFT_StorageDiagnoseResult>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Diagnose", &[])?;
        let diagnose_results = result.get_value("DiagnoseResults")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `action_results` -  (MSFT_HealthAction[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_actions(&self, action_results: &mut Vec<MSFT_HealthAction>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetActions", &[])?;
        let action_results = result.get_value("ActionResults")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

