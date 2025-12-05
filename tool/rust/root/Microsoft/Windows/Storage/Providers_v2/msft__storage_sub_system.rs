// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSubSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSubSystem {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// Denotes whether this subsystem supports automatic object clustering.
    #[serde(rename = "AutomaticClusteringEnabled")]
    pub automatic_clustering_enabled: Option<bool>,

/// This field denotes the cache level that has been discovered. This corresponds to the storage provider's DiscoveryLevel parameter in the Discover method. 
/// 0 - 'Level 0': The storage provider and storage subsystem objects have been discovered. 
/// 1 - 'Level 1': Storage pools, resiliency settings, target ports, target portals, and initiator ids belonging to this subsystem have been discovered. 
/// 2 - 'Level 2': Virtual disks and masking sets belonging to this subsystem have been discovered. 
/// 3 - 'Level 3': Physical disks belonging to this subsystem have been discovered.
    #[serde(rename = "CurrentCacheLevel")]
    pub current_cache_level: Option<StorageSubSystem_CurrentCacheLevel>,

/// Denotes whether storage tiers are supported by the subsystem.
    #[serde(rename = "DataTieringType")]
    pub data_tiering_type: Option<StorageSubSystem_DataTieringType>,

/// A user settable description of the storage subsystem. This field can be used to store extra free-form information, such as notes or details about the subsystem's intended usage.
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// Determines the default allocation behavior for storage pools created in this subsystem. If the subsystem does not support storage pool creation, then it determines the default allocation behavior for virtual disks created in this subsystem.
    #[serde(rename = "FaultDomainAwarenessDefault")]
    pub fault_domain_awareness_default: Option<StorageSubSystem_FaultDomainAwarenessDefault>,

/// This field is a string representation of the subsystem's firmware version.
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// A user settable string representing the name of the storage subsystem. The storage provider or subsystem is expected to supply an initial value for this field.
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// Denotes the health of the subsystem. 
/// 0 - 'Healthy': Indicates that the subsystem is functioning normally. 
/// 1 - 'Warning': Indicates that the subsystem is still functioning, but has detected errors or issues that may require administrator intervention. 
/// 2 - 'Unhealthy': Indicates that the subsystem is not functioning due to errors or failures. The subsystem needs immediate attention from an administrator.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<StorageSubSystem_HealthStatus>,

/// Denotes the iSCSI Target Creation Scheme supported by the subsystem. 
/// 0 - 'Not Applicable' implies a non-iSCSI subsystem. 
/// 1 - 'Not Supported' implies the subsystem does not allow creation of a Target. 
/// 2 - 'Manual' implies the subsystem allows manual creation of the Target. 
/// 3 - 'Auto' implies the subsystem automatically creates a Target. 
/// 
    #[serde(rename = "iSCSITargetCreationScheme")]
    pub i_scsitarget_creation_scheme: Option<StorageSubSystem_iSCSITargetCreationScheme>,

/// This field is a string representation of the company responsible for creating the storage subsystem hardware.
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// If TRUE, the storage provider supports the use of the DeviceNumbers parameter of the CreateMaskingSet and AddVirtualDisk methods.
    #[serde(rename = "MaskingClientSelectableDeviceNumbers")]
    pub masking_client_selectable_device_numbers: Option<bool>,

/// Indicates the maximum number of masking sets that a particular virtual disk can be added to.
    #[serde(rename = "MaskingMapCountMax")]
    pub masking_map_count_max: Option<u16>,

/// If TRUE, the subsystem will only allow one initiator to be added to a masking set.
    #[serde(rename = "MaskingOneInitiatorIdPerView")]
    pub masking_one_initiator_id_per_view: Option<bool>,

/// If MaskingValidInitiatorIdTypes contains the value 1 - 'Other', this field is used to enumerate the other valid initiator id types for this storage subsystem.
    #[serde(rename = "MaskingOtherValidInitiatorIdTypes")]
    pub masking_other_valid_initiator_id_types: Vec<String>,

/// Indicates the number of target ports that can be used for masking a virtual disk. This applies to both masking sets and the virtual disk Show method.
    #[serde(rename = "MaskingPortsPerView")]
    pub masking_ports_per_view: Option<StorageSubSystem_MaskingPortsPerView>,

/// Indicates which address formats can be inferred by the storage provider and subsystem when working with initiator ids.
    #[serde(rename = "MaskingValidInitiatorIdTypes")]
    pub masking_valid_initiator_id_types: Vec<StorageSubSystem_MaskingValidInitiatorIdTypes>,

/// This field is a string representation of the model number of the subsystem array.
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// Name is a globally unique, human-readable string used to identify a storage subsystem.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// NameFormat describes the format of the Name identifier.
    #[serde(rename = "NameFormat")]
    pub name_format: Option<StorageSubSystem_NameFormat>,

/// Denotes the total number of physical disk slots in the subsystem or enclosure.
    #[serde(rename = "NumberOfSlots")]
    pub number_of_slots: Option<u32>,

/// Indicates the current statuses of the subsystem. Various operational statuses are defined. Many of the enumeration's values are self-explanatory. However, a few are not and are described here in more detail. 
/// 4 - 'Stressed': indicates that the subsystem is functioning, but needs attention. Examples of 'Stressed' states are overload, overheated, and so on. 
/// 5 - 'Predictive Failure': indicates that the subsystem is functioning nominally but predicting a failure in the near future. 
/// 11 - 'In Service': describes a subsystem being configured, maintained, cleaned, or otherwise administered. 
/// 12 - 'No Contact': indicates that the storage provider has knowledge of this subsystem, but has never been able to establish communications with it. 
/// 13 - 'Lost Communication': indicates that the subsystem is known to exist and has been contacted successfully in the past, but is currently unreachable. 
/// 10 - 'Stopped' and 14 - 'Aborted' are similar, although the former implies a clean and orderly stop, while the latter implies an abrupt stop where the state and configuration of the subsystem might need to be updated. 
/// 15 - 'Dormant': indicates that the subsystem is inactive. 
/// 16 - 'Supporting Entity in Error': indicates that this subsystem might be OK, but that another element, on which it is dependent, is in error. 
/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<StorageSubSystem_OperationalStatus>,

/// When the corresponding array entry in SupportedHostType[] is "Other", this entry provides a string describing the manufacturer and OS/Environment. When the corresponding SupportedHostType[] entry is not "Other", this entry allows variations or qualifications of ClientTypes - for example, different versions of Solaris.
    #[serde(rename = "OtherHostTypeDescription")]
    pub other_host_type_description: Vec<String>,

/// This field is an array of custom identifier for the subsystem. If this field is set, the OtherIdentifyingInfoDescription field must also be set.
    #[serde(rename = "OtherIdentifyingInfo")]
    pub other_identifying_info: Vec<String>,

/// An array of string description of the format used in the custom identifiers defined in the OtherIdentifyingInfo field. There must be a 1:1 mapping between this array and OtherIdentifyingInfo.
    #[serde(rename = "OtherIdentifyingInfoDescription")]
    pub other_identifying_info_description: Vec<String>,

/// A string representation of the vendor defined operational status. This field should only be set if the OperationalStatus array contains 1 - 'Other'.
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// Denotes the minimum number of physical disks required for creating a storage pool on this subsystem.
    #[serde(rename = "PhysicalDisksPerStoragePoolMin")]
    pub physical_disks_per_storage_pool_min: Option<u16>,

/// This field is reserved for future releases.
    #[serde(rename = "ReplicasPerSourceCloneMax")]
    pub replicas_per_source_clone_max: Option<u16>,

/// This field is reserved for future releases.
    #[serde(rename = "ReplicasPerSourceMirrorMax")]
    pub replicas_per_source_mirror_max: Option<u16>,

/// This field is reserved for future releases.
    #[serde(rename = "ReplicasPerSourceSnapshotMax")]
    pub replicas_per_source_snapshot_max: Option<u16>,

/// This field is a string representation of the serial number of the subsystem array.
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// The storage transport on this subsystem.
    #[serde(rename = "StorageConnectionType")]
    pub storage_connection_type: Option<u16>,

/// Denotes the file system types supported for Deduplication in this subsystem.
    #[serde(rename = "SupportedDeduplicationFileSystemTypes")]
    pub supported_deduplication_file_system_types: Vec<StorageSubSystem_SupportedDeduplicationFileSystemTypes>,

/// Denotes the object types supported for Deduplication in this subsystem.
    #[serde(rename = "SupportedDeduplicationObjectTypes")]
    pub supported_deduplication_object_types: Vec<StorageSubSystem_SupportedDeduplicationObjectTypes>,

/// This field describes the protocols supported by file servers on this subsystem.
    #[serde(rename = "SupportedFileServerProtocols")]
    pub supported_file_server_protocols: Vec<StorageSubSystem_SupportedFileServerProtocols>,

/// File systems supported on this subsystem.
    #[serde(rename = "SupportedFileSystems")]
    pub supported_file_systems: Vec<StorageSubSystem_SupportedFileSystems>,

/// An array representing the supported host types.
    #[serde(rename = "SupportedHostType")]
    pub supported_host_type: Vec<StorageSubSystem_SupportedHostType>,

/// If TRUE, the CreateVirtualDisk method on the storage subsystem is supported.
    #[serde(rename = "SupportsAutomaticStoragePoolSelection")]
    pub supports_automatic_storage_pool_selection: Option<bool>,

/// Denotes whether this subsystem supports local cloning. This field must be true if the VirtualDisk::CreateClone method is implemented.
    #[serde(rename = "SupportsCloneLocal")]
    pub supports_clone_local: Option<bool>,

/// Denotes whether this subsystem supports remote cloning.
    #[serde(rename = "SupportsCloneRemote")]
    pub supports_clone_remote: Option<bool>,

/// Denotes whether this subsystem supports continuously available (CA) file servers.
    #[serde(rename = "SupportsContinuouslyAvailableFileServer")]
    pub supports_continuously_available_file_server: Option<bool>,

/// Denotes whether this subsystem supports a file server.
    #[serde(rename = "SupportsFileServer")]
    pub supports_file_server: Option<bool>,

/// Denotes whether this subsystem supports creation of a file server.
    #[serde(rename = "SupportsFileServerCreation")]
    pub supports_file_server_creation: Option<bool>,

/// If TRUE, the storage subsystem supports showing and hiding (masking) a virtual disk to a host initiator through the Show/Hide methods of the virtual disk and by the use of masking sets.
    #[serde(rename = "SupportsMaskingVirtualDiskToHosts")]
    pub supports_masking_virtual_disk_to_hosts: Option<bool>,

/// Denotes whether this subsystem supports local mirror replication.
    #[serde(rename = "SupportsMirrorLocal")]
    pub supports_mirror_local: Option<bool>,

/// Denotes whether this subsystem supports remote mirror replication.
    #[serde(rename = "SupportsMirrorRemote")]
    pub supports_mirror_remote: Option<bool>,

/// If TRUE, all resiliency settings will be copied from the primordial pool and added to a concrete pool upon its creation. If FALSE, the storage pool should copy the setting specified in the ResiliencySettingNameDefault parameter of CreateStoragePool. If no name was given, the resiliency setting specified by the primordial pool's ResiliencySettingNameDefault property should be used.
    #[serde(rename = "SupportsMultipleResiliencySettingsPerStoragePool")]
    pub supports_multiple_resiliency_settings_per_storage_pool: Option<bool>,

/// Denotes whether this subsystem supports local snapshotting. This field must be true if the VirtualDisk::CreateSnapshot method is implemented.
    #[serde(rename = "SupportsSnapshotLocal")]
    pub supports_snapshot_local: Option<bool>,

/// Denotes whether this subsystem supports remote snapshotting.
    #[serde(rename = "SupportsSnapshotRemote")]
    pub supports_snapshot_remote: Option<bool>,

/// If TRUE, storage pools on this subsystem support capacity expansion through adding more physical disks.
    #[serde(rename = "SupportsStoragePoolAddPhysicalDisk")]
    pub supports_storage_pool_add_physical_disk: Option<bool>,

/// If TRUE, this subsystem supports the ability to create new concrete storage pools from one or more physical disks. If FALSE, either the subsystem uses pre-created storage pools, or it does not support storage pools.
    #[serde(rename = "SupportsStoragePoolCreation")]
    pub supports_storage_pool_creation: Option<bool>,

/// If TRUE, this subsystem supports the deletion of its storage pools.
    #[serde(rename = "SupportsStoragePoolDeletion")]
    pub supports_storage_pool_deletion: Option<bool>,

/// 
    #[serde(rename = "SupportsStoragePoolFriendlyNameModification")]
    pub supports_storage_pool_friendly_name_modification: Option<bool>,

/// If TRUE, storage pools on this subsystem support the replacement or removal of physical disks by use of the RemovePhysicalDisk method on the storage pool instance.
    #[serde(rename = "SupportsStoragePoolRemovePhysicalDisk")]
    pub supports_storage_pool_remove_physical_disk: Option<bool>,

/// If TRUE, this subsystem supports the ability to create new storage tiers. If FALSE, either the subsystem uses pre-created storage tiers, or it does not support storage tiers.
    #[serde(rename = "SupportsStorageTierCreation")]
    pub supports_storage_tier_creation: Option<bool>,

/// If TRUE, this subsystem supports the deletion of storage tiers.
    #[serde(rename = "SupportsStorageTierDeletion")]
    pub supports_storage_tier_deletion: Option<bool>,

/// If TRUE, this subsystem supports the creation of tiered virtual disks.
    #[serde(rename = "SupportsStorageTieredVirtualDiskCreation")]
    pub supports_storage_tiered_virtual_disk_creation: Option<bool>,

/// If TRUE, this subsystem supports the modification of the storage tier friendly name.
    #[serde(rename = "SupportsStorageTierFriendlyNameModification")]
    pub supports_storage_tier_friendly_name_modification: Option<bool>,

/// If TRUE, this subsystem supports the resizing of storage tiers.
    #[serde(rename = "SupportsStorageTierResize")]
    pub supports_storage_tier_resize: Option<bool>,

/// Indicates if the subsystem allows a virtual disk to be grown in size (using the Resize method of the virtual disk instance).
    #[serde(rename = "SupportsVirtualDiskCapacityExpansion")]
    pub supports_virtual_disk_capacity_expansion: Option<bool>,

/// Indicates if the subsystem allows a virtual disk to be reduced in size (using the Resize method of the virtual disk instance).
    #[serde(rename = "SupportsVirtualDiskCapacityReduction")]
    pub supports_virtual_disk_capacity_reduction: Option<bool>,

/// Denotes whether a user can create a virtual disk by using the CreateVirtualDisk method on either the storage subsystem or storage pool objects.
    #[serde(rename = "SupportsVirtualDiskCreation")]
    pub supports_virtual_disk_creation: Option<bool>,

/// Denotes whether a user can delete a virtual disk through the use of the DeleteObject extrinsic method on the virtual disk instance.
    #[serde(rename = "SupportsVirtualDiskDeletion")]
    pub supports_virtual_disk_deletion: Option<bool>,

/// Denotes whether a user can modify attributes or other properties on a virtual disk by using the various Set* extrinsic methods. (For example: SetFriendlyname ).
    #[serde(rename = "SupportsVirtualDiskModification")]
    pub supports_virtual_disk_modification: Option<bool>,

/// Indicates if the subsystem supports explicit repairing of a virtual disk through the Repair method of the virtual disk instance.
    #[serde(rename = "SupportsVirtualDiskRepair")]
    pub supports_virtual_disk_repair: Option<bool>,

/// Denotes whether this subsystem supports direct creation of volumes on a storage pool.
    #[serde(rename = "SupportsVolumeCreation")]
    pub supports_volume_creation: Option<bool>,

/// Tag is an identifier for the subsystem that is independent from any location-based information. Examples of a tag could be the subsystem's serial number or asset tag.
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

/// Denotes whether virtual disk repair is enabled on this subsystem.
    #[serde(rename = "VirtualDiskRepairEnabled")]
    pub virtual_disk_repair_enabled: Option<bool>,

/// Denotes the virtual disk repair queue depth policy in this subsystem.
    #[serde(rename = "VirtualDiskRepairQueueDepth")]
    pub virtual_disk_repair_queue_depth: Option<u32>,
}

impl MSFT_StorageSubSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            automatic_clustering_enabled: None,
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

    /// Sets the value of CurrentCacheLevel
    pub fn set_current_cache_level(&mut self, value: StorageSubSystem_CurrentCacheLevel) {
        self.current_cache_level = Some(value);
    }

    /// Gets the value of CurrentCacheLevel
    pub fn get_current_cache_level(&self) -> Option<&StorageSubSystem_CurrentCacheLevel> {
        self.current_cache_level.as_ref()
    }

    /// Sets the value of DataTieringType
    pub fn set_data_tiering_type(&mut self, value: StorageSubSystem_DataTieringType) {
        self.data_tiering_type = Some(value);
    }

    /// Gets the value of DataTieringType
    pub fn get_data_tiering_type(&self) -> Option<&StorageSubSystem_DataTieringType> {
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
    pub fn set_fault_domain_awareness_default(&mut self, value: StorageSubSystem_FaultDomainAwarenessDefault) {
        self.fault_domain_awareness_default = Some(value);
    }

    /// Gets the value of FaultDomainAwarenessDefault
    pub fn get_fault_domain_awareness_default(&self) -> Option<&StorageSubSystem_FaultDomainAwarenessDefault> {
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
    pub fn set_health_status(&mut self, value: StorageSubSystem_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&StorageSubSystem_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of iSCSITargetCreationScheme
    pub fn set_i_scsitarget_creation_scheme(&mut self, value: StorageSubSystem_iSCSITargetCreationScheme) {
        self.i_scsitarget_creation_scheme = Some(value);
    }

    /// Gets the value of iSCSITargetCreationScheme
    pub fn get_i_scsitarget_creation_scheme(&self) -> Option<&StorageSubSystem_iSCSITargetCreationScheme> {
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
    pub fn set_masking_ports_per_view(&mut self, value: StorageSubSystem_MaskingPortsPerView) {
        self.masking_ports_per_view = Some(value);
    }

    /// Gets the value of MaskingPortsPerView
    pub fn get_masking_ports_per_view(&self) -> Option<&StorageSubSystem_MaskingPortsPerView> {
        self.masking_ports_per_view.as_ref()
    }

    /// Sets the value of MaskingValidInitiatorIdTypes
    pub fn set_masking_valid_initiator_id_types(&mut self, value: Vec<StorageSubSystem_MaskingValidInitiatorIdTypes>) {
        self.masking_valid_initiator_id_types = value;
    }

    /// Gets the value of MaskingValidInitiatorIdTypes
    pub fn get_masking_valid_initiator_id_types(&self) -> &Vec<StorageSubSystem_MaskingValidInitiatorIdTypes> {
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
    pub fn set_name_format(&mut self, value: StorageSubSystem_NameFormat) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&StorageSubSystem_NameFormat> {
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
    pub fn set_operational_status(&mut self, value: Vec<StorageSubSystem_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<StorageSubSystem_OperationalStatus> {
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
    pub fn set_supported_deduplication_file_system_types(&mut self, value: Vec<StorageSubSystem_SupportedDeduplicationFileSystemTypes>) {
        self.supported_deduplication_file_system_types = value;
    }

    /// Gets the value of SupportedDeduplicationFileSystemTypes
    pub fn get_supported_deduplication_file_system_types(&self) -> &Vec<StorageSubSystem_SupportedDeduplicationFileSystemTypes> {
        &self.supported_deduplication_file_system_types
    }

    /// Sets the value of SupportedDeduplicationObjectTypes
    pub fn set_supported_deduplication_object_types(&mut self, value: Vec<StorageSubSystem_SupportedDeduplicationObjectTypes>) {
        self.supported_deduplication_object_types = value;
    }

    /// Gets the value of SupportedDeduplicationObjectTypes
    pub fn get_supported_deduplication_object_types(&self) -> &Vec<StorageSubSystem_SupportedDeduplicationObjectTypes> {
        &self.supported_deduplication_object_types
    }

    /// Sets the value of SupportedFileServerProtocols
    pub fn set_supported_file_server_protocols(&mut self, value: Vec<StorageSubSystem_SupportedFileServerProtocols>) {
        self.supported_file_server_protocols = value;
    }

    /// Gets the value of SupportedFileServerProtocols
    pub fn get_supported_file_server_protocols(&self) -> &Vec<StorageSubSystem_SupportedFileServerProtocols> {
        &self.supported_file_server_protocols
    }

    /// Sets the value of SupportedFileSystems
    pub fn set_supported_file_systems(&mut self, value: Vec<StorageSubSystem_SupportedFileSystems>) {
        self.supported_file_systems = value;
    }

    /// Gets the value of SupportedFileSystems
    pub fn get_supported_file_systems(&self) -> &Vec<StorageSubSystem_SupportedFileSystems> {
        &self.supported_file_systems
    }

    /// Sets the value of SupportedHostType
    pub fn set_supported_host_type(&mut self, value: Vec<StorageSubSystem_SupportedHostType>) {
        self.supported_host_type = value;
    }

    /// Gets the value of SupportedHostType
    pub fn get_supported_host_type(&self) -> &Vec<StorageSubSystem_SupportedHostType> {
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

/// This method creates a storage pool from available physical disks contained within a common primordial pool. A physical disk is available for storage pool creation if its CanPool property is set to TRUE. Storage pool creation is only available when the SupportsStoragePoolCreation field of the storage subsystem is TRUE.

    /// * `auto_write_cache_size` - Indicates if provider should pick up the auto write cache size or not (bool)
    /// * `enclosure_aware_default` - This parameter indicates the default allocation policy for virtual disks created in an enclosure aware storage pool. For example, an enclosure aware subsystem could balance each data copy of the virtual disk across multiple physical enclosures such that each enclosure contains a full data copy of the virtual disk. (bool)
    /// * `friendly_name` - This parameter allows the user to specify the FriendlyName at the time of the storage pool creation. FriendlyNames are expected to be descriptive, however they are not required to be unique. Note that some storage subsystems do not allow setting a friendly name during pool creation. If a subsystem doesn't support this, storage pool creation should still succeed, however the pool may have a different name assigned to it. (String)
    /// * `logical_sector_size_default` - This parameter indicates the default logical sector size for the storage pool. This is useful when a storage pool may contain a mix of 512 emulated and either 4K native or 512 native physical disks. (u64)
    /// * `other_usage_description` - Allows a user to set a custom usage type for the new storage pool object. This parameter can only be specified if the Usage parameter is set to 1 - 'Other'.  (String)
    /// * `physical_disks` - This parameter is used to specify an array of physical disk objects that will be used as the backing data storage for the created storage pool. The physical disks must come from a primordial pool on the subsystem on which you are creating this pool. Only the disks from a single primordial pool may be used. (MSFT_PhysicalDisk[])
    /// * `provisioning_type_default` - This parameter indicates the provisioning type to be used by default when creating a new virtual disk on this storage pool. If no default is specified, the default is inherited from the primordial pool. (StorageSubSystem_ProvisioningTypeDefault)
    /// * `resiliency_setting_name_default` - This parameter indicates the resiliency setting to be used by default when creating a new virtual disk on this storage pool. If the subsystem's SupportsMultipleResiliencySettingsPerStoragePool property is set to FALSE, this parameter also acts as a hint to the Storage Management Provider on which resiliency setting should be inherited by this storage pool. If no value is given, it is up to the Storage Management Provider to pick the most appropriate resiliency setting. (String)
    /// * `usage` - Denotes the intended usage of the storage pool. (StorageSubSystem_Usage)
    /// * `write_cache_size_default` - Default size of write cache for virtual disk creation (u64)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. When the operation has completed, an association should exist between the storage job and the created objects. (MSFT_StorageJob)
    /// * `created_storage_pool` -  (MSFT_StoragePool)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_pool(&self, friendly_name: &String, usage: StorageSubSystem_Usage, other_usage_description: &String, physical_disks: &Vec<MSFT_PhysicalDisk>, resiliency_setting_name_default: &String, provisioning_type_default: StorageSubSystem_ProvisioningTypeDefault, logical_sector_size_default: u64, enclosure_aware_default: bool, write_cache_size_default: u64, auto_write_cache_size: bool, created_storage_pool: &mut MSFT_StoragePool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "ResiliencySettingNameDefault".to_string(), value: resiliency_setting_name_default.into() });
        args.push(MethodParameter { name: "ProvisioningTypeDefault".to_string(), value: provisioning_type_default.into() });
        args.push(MethodParameter { name: "LogicalSectorSizeDefault".to_string(), value: logical_sector_size_default.into() });
        args.push(MethodParameter { name: "EnclosureAwareDefault".to_string(), value: enclosure_aware_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });

        let result = self.invoke_method("CreateStoragePool", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_pool = result.get_value("CreatedStoragePool")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `auto_write_cache_size` -  (bool)
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
    /// * `usage` -  (u16)
    /// * `version` -  (u16)
    /// * `write_cache_size_default` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_storage_pool` -  (MSFT_StoragePool)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_pool2(&self, friendly_name: &String, usage: u16, other_usage_description: &String, physical_disks: &Vec<MSFT_PhysicalDisk>, resiliency_setting_name_default: &String, provisioning_type_default: u16, media_type_default: u16, logical_sector_size_default: u64, metadata_length: u64, minimum_allocation_size: u64, fault_domain_awareness_default: u16, write_cache_size_default: u64, auto_write_cache_size: bool, version: u16, created_storage_pool: &mut MSFT_StoragePool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });
        args.push(MethodParameter { name: "Version".to_string(), value: version.into() });

        let result = self.invoke_method("CreateStoragePool2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_pool = result.get_value("CreatedStoragePool")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows for the creation of virtual disks on a storage subsystem. This method is typically used when either a) the subsystem's storage pools do not allow virtual disk creation directly, or b) the subsystem does not support storage pools. Storage Management Providers may also choose to implement this method to 'intelligently' pick a storage pool for the user. If this method is supported, the subsystem's SupportsAutomaticStoragePoolSelection property should be set to TRUE.

    /// * `friendly_name` - This parameter allows the user to specify the desired FriendlyName at the time of the virtual disk creation. FriendlyNames are expected to be descriptive, however they are not required to be unique. Note that some storage subsystems do not allow setting a friendly name during virtual disk creation. If a subsystem doesn't support this, virtual disk creation should still succeed, however the disk may have a different name assigned to it. (String)
    /// * `interleave` - Specifies the number of bytes used to form a strip in common striping-based resiliency settings. The strip is defined as the size of the portion of a stripe that lies on one physical disk. Thus Interleave * NumberOfColumns will yield the total size of one stripe. (u64)
    /// * `is_enclosure_aware` - Determines the allocation behavior for this virtual disk. Enclosure aware virtual disks will intelligently pick the physical disks to use for their redundancy. If TRUE, the virtual disk will attempt to use physical disks from different enclosures to balance the fault tolerance between two (or more) physical enclosures. (bool)
    /// * `number_of_columns` - Specifies the number of underlying physical disks across which data should be striped. (u16)
    /// * `number_of_data_copies` - Specifies the number of complete data copies to maintain for this virtual disk. (u16)
    /// * `other_usage_description` - Allows a user to set a custom usage type for the new virtual disk object. This parameter can only be specified if the Usage parameter is set to 1 - 'Other'.  (String)
    /// * `parity_layout` - This field specifies whether a parity-based resiliency setting is using a rotated or non-rotated parity layout. If the resiliency setting is not parity based, this field must be set to NULL (StorageSubSystem_ParityLayout)
    /// * `physical_disk_redundancy` - Specifies how many physical disk failures the virtual disk should be able to withstand before data loss occurs. (u16)
    /// * `provisioning_type` - Denotes the provisioning type of the virtual disk. A value of 1 - 'Thin' means that the storage for the disk is allocated on-demand. A value of 2 - 'Fixed' means that the storage is allocated up front. (StorageSubSystem_ProvisioningType)
    /// * `request_no_single_point_of_failure` -  (bool)
    /// * `size` - Indicates the desired size for the virtual disk. Note that some storage subsystems will round the size up or down to a multiple of its allocation unit size. If this parameter is specified, UseMaximumSize must be NULL or FALSE. (u64)
    /// * `usage` - Denotes the intended usage of the virtual disk (StorageSubSystem_Usage)
    /// * `use_maximum_size` - Create a virtual disk using the largest supported size. This parameter cannot be used with the Size parameter. (bool)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. When the operation has completed, an association should exist between the storage job and the created objects. (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `size` - Indicates the desired size for the virtual disk. Note that some storage subsystems will round the size up or down to a multiple of its allocation unit size. If this parameter is specified, UseMaximumSize must be NULL or FALSE. (u64)
    pub fn create_virtual_disk(&self, friendly_name: &String, usage: StorageSubSystem_Usage, other_usage_description: &String, size: &mut u64, use_maximum_size: bool, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, interleave: u64, parity_layout: StorageSubSystem_ParityLayout, request_no_single_point_of_failure: bool, is_enclosure_aware: bool, provisioning_type: StorageSubSystem_ProvisioningType, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });

        let result = self.invoke_method("CreateVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let size = result.get_value("Size")?;
        Ok(result.return_value)

    }


/// 

    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `other_usage_description` -  (String)
    /// * `parity_layout` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `request_no_single_point_of_failure` -  (bool)
    /// * `size` -  (u64)
    /// * `usage` -  (u16)
    /// * `use_maximum_size` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_virtual_disk2(&self, friendly_name: &String, usage: u16, other_usage_description: &String, size: u64, use_maximum_size: bool, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, interleave: u64, parity_layout: u16, request_no_single_point_of_failure: bool, fault_domain_awareness: u16, provisioning_type: u16, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "UseMaximumSize".to_string(), value: use_maximum_size.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "ParityLayout".to_string(), value: parity_layout.into() });
        args.push(MethodParameter { name: "RequestNoSinglePointOfFailure".to_string(), value: request_no_single_point_of_failure.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });

        let result = self.invoke_method("CreateVirtualDisk2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// Creates logical grouping of virtual disks, target ports, and initiators for the purpose of showing virtual disks to host systems.

    /// * `device_accesses` - This parameter specifies the level of access the initiator should have to each virtual disk specified by VirtualDiskNames. This parameter has a 1:1 mapping with the VirtualDiskNames parameter (the arrays must be the same length and have the same order). (StorageSubSystem_DeviceAccesses[])
    /// * `device_numbers` - Specifies the order in which the virtual disks should be exposed to the initiator. This capability is only available if the storage subsystem's MaskingClientSelectableDeviceNumbers property is set to TRUE. If specified, this parameter must have a 1:1 mapping with the VirtualDiskNames parameter. (String[])
    /// * `friendly_name` - This parameter allows the user to specify the desired FriendlyName for the masking set at the time of its creation. FriendlyNames are expected to be descriptive, however they are not requried to be unique. (String)
    /// * `host_type` - Designates the host operating system or other host environment factors that may influence the behavior the storage subsystem should take when showing a virtual disk to an initiator. (StorageSubSystem_HostType)
    /// * `initiator_addresses` - This parameter specifies the initiators for which the virtual disks should be shown. If the subsystem's MaskingOneInitiatorIdPerView property is TRUE, only one initiator can be specified for this masking set. The list of valid initiator address formats can be determined through the subsystem's MaskingValidInitiatorIdTypes property. (String[])
    /// * `target_port_addresses` - This parameter specifies the target ports which should be used when showing the virtual disks to the initiators. The number of target ports that can be specified depends on the subsystem's MaskingPortsPerView property. If MaskingPortsPerView is set to 4 - 'All target ports share the same view', this parameter is essentially ignored as all target ports on the system will be associated with this masking set. (String[])
    /// * `virtual_disk_names` - This parameter specifies the list of virtual disks to show to the initiators in the masking set. The identifier used by this parameter is the virtual disk Name property. This parameter has a 1:1 mapping with the DeviceAccesses parameter (the arrays must be the same length and have the same order). (String[])

    /// * `created_masking_set` -  (MSFT_MaskingSet)
    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. When the operation has completed, an association should exist between the storage job and the created objects. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_masking_set(&self, friendly_name: &String, virtual_disk_names: &Vec<String>, device_accesses: &Vec<StorageSubSystem_DeviceAccesses>, device_numbers: &Vec<String>, target_port_addresses: &Vec<String>, initiator_addresses: &Vec<String>, host_type: StorageSubSystem_HostType, created_storage_job: &mut MSFT_StorageJob, created_masking_set: &mut MSFT_MaskingSet, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "VirtualDiskNames".to_string(), value: virtual_disk_names.into() });
        args.push(MethodParameter { name: "DeviceAccesses".to_string(), value: device_accesses.into() });
        args.push(MethodParameter { name: "DeviceNumbers".to_string(), value: device_numbers.into() });
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "InitiatorAddresses".to_string(), value: initiator_addresses.into() });
        args.push(MethodParameter { name: "HostType".to_string(), value: host_type.into() });

        let result = self.invoke_method("CreateMaskingSet", &args)?;
        let created_masking_set = result.get_value("CreatedMaskingSet")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method returns the security descriptor that controls access to this specific object instance.

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `security_descriptor` - A Security Descriptor Definition Language (SDDL) formed string describing the access control list of the object. (String)
    pub fn get_security_descriptor(&self, security_descriptor: &mut String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let security_descriptor = result.get_value("SecurityDescriptor")?;
        Ok(result.return_value)

    }


/// This method allows a user with sufficient privileges to set the security descriptor that control access to this specific object instance. If the call is not made in the context of a user specified in the security descriptor's access control list, this method will fail with 40001 - 'Access Denied'. If an empty security descriptor is passed to this function, the behavior is left to the specific implementation so long as there is some user context (typically domain administrators) that can access and administer the object.

    /// * `security_descriptor` - A Security Descriptor Definition Language (SDDL) formed string describing the desired access control list for this object. (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, security_descriptor: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecurityDescriptor".to_string(), value: security_descriptor.into() });

        let result = self.invoke_method("SetSecurityDescriptor", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows a user to set the description field of the storage subsystem.

    /// * `description` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_description(&self, description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("SetDescription", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows a user to set the SupportsAutomaticObjectClustering field of the storage subsystem.

    /// * `automatic_clustering_enabled` -  (bool)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, automatic_clustering_enabled: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AutomaticClusteringEnabled".to_string(), value: automatic_clustering_enabled.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
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
    pub fn set_attributes2(&self, automatic_clustering_enabled: bool, virtual_disk_repair_enabled: bool, virtual_disk_repair_queue_depth: u32, fault_domain_awareness_default: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AutomaticClusteringEnabled".to_string(), value: automatic_clustering_enabled.into() });
        args.push(MethodParameter { name: "VirtualDiskRepairEnabled".to_string(), value: virtual_disk_repair_enabled.into() });
        args.push(MethodParameter { name: "VirtualDiskRepairQueueDepth".to_string(), value: virtual_disk_repair_queue_depth.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });

        let result = self.invoke_method("SetAttributes2", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `recovery_point_objective` -  (u32)
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
    pub fn create_replication_relationship(&self, friendly_name: &String, sync_type: u16, target_storage_subsystem: MSFT_ReplicaPeer, source_replication_group_friendly_name: &String, source_replication_group_description: &String, source_storage_elements: &Vec<MSFT_StorageObject>, source_group_settings: MSFT_ReplicationSettings, target_replication_group_friendly_name: &String, target_replication_group_description: &String, target_storage_elements: &Vec<MSFT_StorageObject>, target_storage_pool: MSFT_StoragePool, target_storage_pools: &Vec<MSFT_StoragePool>, target_group_settings: MSFT_ReplicationSettings, recovery_point_objective: u32, source_group: &mut MSFT_ReplicationGroup, target_group: MSFT_ReplicationGroup, created_replica_peer: &mut MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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
        args.push(MethodParameter { name: "TargetGroup".to_string(), value: target_group.into() });

        let result = self.invoke_method("CreateReplicationRelationship", &args)?;
        let created_replica_peer = result.get_value("CreatedReplicaPeer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let source_group = result.get_value("SourceGroup")?;
        Ok(result.return_value)

    }


/// 

    /// * `source_replication_group` -  (MSFT_ReplicationGroup)
    /// * `target_group_replica_peer` -  (MSFT_ReplicaPeer)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_replication_relationship(&self, source_replication_group: MSFT_ReplicationGroup, target_group_replica_peer: MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SourceReplicationGroup".to_string(), value: source_replication_group.into() });
        args.push(MethodParameter { name: "TargetGroupReplicaPeer".to_string(), value: target_group_replica_peer.into() });

        let result = self.invoke_method("DeleteReplicationRelationship", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `friendly_name` -  (String)
    /// * `replication_settings` -  (MSFT_ReplicationSettings)
    /// * `storage_elements` -  (MSFT_StorageObject[])

    /// * `created_replication_group` -  (MSFT_ReplicationGroup)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_replication_group(&self, friendly_name: &String, description: &String, storage_elements: &Vec<MSFT_StorageObject>, replication_settings: MSFT_ReplicationSettings, created_storage_job: &mut MSFT_StorageJob, created_replication_group: &mut MSFT_ReplicationGroup, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "StorageElements".to_string(), value: storage_elements.into() });
        args.push(MethodParameter { name: "ReplicationSettings".to_string(), value: replication_settings.into() });

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

    /// * `created_file_server` -  (MSFT_FileServer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_file_server(&self, friendly_name: &String, file_sharing_protocols: &Vec<u16>, host_names: &Vec<String>, created_file_server: &mut MSFT_FileServer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "FileSharingProtocols".to_string(), value: file_sharing_protocols.into() });
        args.push(MethodParameter { name: "HostNames".to_string(), value: host_names.into() });

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

