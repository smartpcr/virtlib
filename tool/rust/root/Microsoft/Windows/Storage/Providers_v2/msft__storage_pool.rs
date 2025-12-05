// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StoragePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StoragePool {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// Indicates the total sum of all the capacity used by this storage pool. If the pool is primordial, this will be the sum of all capacity currently allocated to concrete storage pools. If the pool is concrete, this value should be the sum of all capacity currently allocated to virtual disks and other pool metadata.
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// If TRUE, the storage pool should clear (zero out) physical disks that are removed from the pool.
    #[serde(rename = "ClearOnDeallocate")]
    pub clear_on_deallocate: Option<bool>,

/// Determines the default allocation behavior for virtual disks created in this pool. Enclosure aware virtual disks will intelligently pick the physical disks to use for their redundancy. If TRUE, the storage subsystem will use physical disks from different enclosures to balance the fault tolerance between two (or more) physical enclosures.
    #[serde(rename = "EnclosureAwareDefault")]
    pub enclosure_aware_default: Option<bool>,

/// Determines the default allocation behavior for virtual disks created in this pool. Fault domain aware virtual disks will intelligently pick the physical disks to use for their redundancy to balance the fault tolerance between two (or more) fault domain units of the specified type.
    #[serde(rename = "FaultDomainAwarenessDefault")]
    pub fault_domain_awareness_default: Option<StoragePool_FaultDomainAwarenessDefault>,

/// A user-friendly string representing the name of the storage pool. Friendly name can be set using the SetFriendlyName method.
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// Denotes the current health status of the storage pool. Health of a storage pool is derived from the health of the backing physical disks, and whether or not the storage pool can maintain the required levels of resiliency.
///  0 - 'Healthy': All physical disks are present and in a healthy state. 
/// 1 - 'Warning': The majority of physical disks are healthy, but one or more may be failing I/O requests. 
/// 2 - 'Unhealthy': The majority of physical disks are unhealthy or in a failed state, and the pool no longer has data integrity.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<StoragePool_HealthStatus>,

/// Indicates whether or not the storage pool is used in a clustered environment.
    #[serde(rename = "IsClustered")]
    pub is_clustered: Option<bool>,

/// This property indicates whether the disks comprising this pool are able to tolerate power loss without data loss, e.g. automatically flush volatile buffers to non-volatile media after external power is disconnected.
    #[serde(rename = "IsPowerProtected")]
    pub is_power_protected: Option<bool>,

/// If this field is set to TRUE, the storage pool is primordial. A primordial pool, also known as the 'available storage' pool is where storage capacity is drawn and returned in the creation and deletion of concrete storage pools. Primordial pools cannot be created or deleted. 
/// If this field is set to FALSE, the storage pool is a concrete pool. These pools are subject to all of the management operations defined on the storage pool class. This includes creation, deletion, creation of virtual disks, etc.
    #[serde(rename = "IsPrimordial")]
    pub is_primordial: Option<bool>,

/// Indicates whether or not the storage pool's configuration is read-only. If TRUE, the storage pool will not allow configuration changes to itself or any of its virtual and physical disks. Note that the data on the virtual disk may still be writable.
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// This field indicates the logical sector size of the storage pool, in bytes. This value is derived from the backing physical disks, as well as the preference specified at the time this storage pool was created.
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u64>,

/// 
    #[serde(rename = "MediaTypeDefault")]
    pub media_type_default: Option<u16>,

/// The size of the storage pool metadata in bytes.
    #[serde(rename = "MetadataLength")]
    pub metadata_length: Option<u64>,

/// Indicates the smallest unit of allocation for this storage pool.
    #[serde(rename = "MinimumAllocationSize")]
    pub minimum_allocation_size: Option<u64>,

/// Name is a semi-unique (scoped to the owning storage subsystem), human-readable string used to identify a storage pool.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// Indicates the current operating conditions of the storage pool. Unlike HealthStatus, this field indicates the status of hardware, software, and infrastructure issues related to this storage pool, and can contain multiple values. Various operational statuses are defined. Many of the enumeration's values are self-explanatory. However, a few are not and are described here in more detail. 
/// 4 - 'Stressed': indicates that the storage pool is functioning, but needs attention. Examples of 'Stressed' states are overload, overheated, and so on. 
/// 5 - 'Predictive Failure': indicates that the storage pool is functioning nominally but predicting a failure in the near future. 
/// 11 - 'In Service': describes a storage pool being configured, maintained, or otherwise administered. 
/// 12 - 'No Contact': indicates that the storage provider has knowledge of this storage pool, but has never been able to establish communications with it. 
/// 13 - 'Lost Communication': indicates that the storage pool is known to exist and has been contacted successfully in the past, but is currently unreachable. 
/// 10 - 'Stopped' and 14 - 'Aborted' are similar, although the former implies a clean and orderly stop, while the latter implies an abrupt stop where the state and configuration of the storage pool might need to be updated. 
/// 15 - 'Dormant': indicates that the storage pool is inactive. 
/// 16 - 'Supporting Entity in Error': indicates that this storage pool might be OK, but that another element, on which it is dependent, is in error. 
/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<StoragePool_OperationalStatus>,

/// A string representation of the vendor defined operational status. This field should only be set if the OperationalStatus array contains 1 - 'Other'.
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// If Usage is set to 1 - 'Other', this field contains the string representation of the vendor defined usage for the storage pool. This property must be NULL if Usage is not set to 1 - 'Other'.
    #[serde(rename = "OtherUsageDescription")]
    pub other_usage_description: Option<String>,

/// This field indicates the physical sector size of the storage pool, in bytes. This value is derived from the backing physical disks for this storage pool.
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u64>,

/// Indicates the provisioning scheme to use when creating new virtual disks on this storage pool. 
/// 0 - 'Unknown': May mean that this information is unavailable, or the storage pool uses a proprietary method of allocation.
/// 1 - 'Thin': Storage for the virtual disk is allocated on-demand. 
/// 2 - 'Fixed': Storage for the virtual disk is allocated at the time of virtual disk creation.
    #[serde(rename = "ProvisioningTypeDefault")]
    pub provisioning_type_default: Option<StoragePool_ProvisioningTypeDefault>,

/// Denotes the reason why the storage pool is read-only. 
/// 1 - 'None': The pool is not read-only. 
/// 2 - 'By Policy': The administrator has either requested the pool to be read-only or has enacted a policy on the system that requires the pool to be read-only. 
/// 3 - 'Majority Disks Unhealthy': The majority of the supporting physical disks are in an unhealthy state that has forced the storage pool into a read-only state.
    #[serde(rename = "ReadOnlyReason")]
    pub read_only_reason: Option<StoragePool_ReadOnlyReason>,

/// This property indicates how the operating system will proceed with repairing of virtual disks for this storage pool.
/// 2 - 'Sequential': repair will process one allocation slab at a time. This will result in longer repair times, but small impact on the I/O load.
/// 3 - 'Parallel': repair will process as many allocation slabs as it can in parallel. This will result in the shortest repair time, but will have significant impact on I/O load.
/// 
    #[serde(rename = "RepairPolicy")]
    pub repair_policy: Option<StoragePool_RepairPolicy>,

/// Indicates the default resiliency setting used for virtual disk creation. This default can be overridden at the time of virtual disk creation. This property's value should correspond to the resiliency setting's Name field.
    #[serde(rename = "ResiliencySettingNameDefault")]
    pub resiliency_setting_name_default: Option<String>,

/// If TRUE, the storage subsystem will automatically retire missing physical disks in this storage pool and replace them with hot-spares or other available physical disks (in the storage pool).
    #[serde(rename = "RetireMissingPhysicalDisks")]
    pub retire_missing_physical_disks: Option<StoragePool_RetireMissingPhysicalDisks>,

/// Indicates the capacity of the storage pool. If the pool is primordial, this is the sum of all the healthy physical disk sizes. If the pool is concrete, this is the sum of all associated physical disks (except hot-spares, and including failed drives).
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// Denotes the provisioning schemes that this storage pool supports.
    #[serde(rename = "SupportedProvisioningTypes")]
    pub supported_provisioning_types: Vec<StoragePool_SupportedProvisioningTypes>,

/// If TRUE, this storage pool supports data deduplication.
    #[serde(rename = "SupportsDeduplication")]
    pub supports_deduplication: Option<bool>,

/// Percentages at which an alert should be generated
    #[serde(rename = "ThinProvisioningAlertThresholds")]
    pub thin_provisioning_alert_thresholds: Vec<u16>,

/// Denotes the intended usage of the storage pool.
    #[serde(rename = "Usage")]
    pub usage: Option<StoragePool_Usage>,

/// Denotes the version of this storage pool.
    #[serde(rename = "Version")]
    pub version: Option<StoragePool_Version>,

/// Default size of write cache for virtual disk creation
    #[serde(rename = "WriteCacheSizeDefault")]
    pub write_cache_size_default: Option<u64>,

/// Maximum size of write cache for virtual disk creation
    #[serde(rename = "WriteCacheSizeMax")]
    pub write_cache_size_max: Option<u64>,

/// Minimum size of write cache for virtual disk creation
    #[serde(rename = "WriteCacheSizeMin")]
    pub write_cache_size_min: Option<u64>,
}

impl MSFT_StoragePool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            allocated_size: None,
            clear_on_deallocate: None,
            enclosure_aware_default: None,
            fault_domain_awareness_default: None,
            friendly_name: None,
            health_status: None,
            is_clustered: None,
            is_power_protected: None,
            is_primordial: None,
            is_read_only: None,
            logical_sector_size: None,
            media_type_default: None,
            metadata_length: None,
            minimum_allocation_size: None,
            name: None,
            operational_status: Vec::new(),
            other_operational_status_description: None,
            other_usage_description: None,
            physical_sector_size: None,
            provisioning_type_default: None,
            read_only_reason: None,
            repair_policy: None,
            resiliency_setting_name_default: None,
            retire_missing_physical_disks: None,
            size: None,
            supported_provisioning_types: Vec::new(),
            supports_deduplication: None,
            thin_provisioning_alert_thresholds: Vec::new(),
            usage: None,
            version: None,
            write_cache_size_default: None,
            write_cache_size_max: None,
            write_cache_size_min: None,
        }
    }


    /// Sets the value of AllocatedSize
    pub fn set_allocated_size(&mut self, value: u64) {
        self.allocated_size = Some(value);
    }

    /// Gets the value of AllocatedSize
    pub fn get_allocated_size(&self) -> Option<&u64> {
        self.allocated_size.as_ref()
    }

    /// Sets the value of ClearOnDeallocate
    pub fn set_clear_on_deallocate(&mut self, value: bool) {
        self.clear_on_deallocate = Some(value);
    }

    /// Gets the value of ClearOnDeallocate
    pub fn get_clear_on_deallocate(&self) -> Option<&bool> {
        self.clear_on_deallocate.as_ref()
    }

    /// Sets the value of EnclosureAwareDefault
    pub fn set_enclosure_aware_default(&mut self, value: bool) {
        self.enclosure_aware_default = Some(value);
    }

    /// Gets the value of EnclosureAwareDefault
    pub fn get_enclosure_aware_default(&self) -> Option<&bool> {
        self.enclosure_aware_default.as_ref()
    }

    /// Sets the value of FaultDomainAwarenessDefault
    pub fn set_fault_domain_awareness_default(&mut self, value: StoragePool_FaultDomainAwarenessDefault) {
        self.fault_domain_awareness_default = Some(value);
    }

    /// Gets the value of FaultDomainAwarenessDefault
    pub fn get_fault_domain_awareness_default(&self) -> Option<&StoragePool_FaultDomainAwarenessDefault> {
        self.fault_domain_awareness_default.as_ref()
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
    pub fn set_health_status(&mut self, value: StoragePool_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&StoragePool_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of IsClustered
    pub fn set_is_clustered(&mut self, value: bool) {
        self.is_clustered = Some(value);
    }

    /// Gets the value of IsClustered
    pub fn get_is_clustered(&self) -> Option<&bool> {
        self.is_clustered.as_ref()
    }

    /// Sets the value of IsPowerProtected
    pub fn set_is_power_protected(&mut self, value: bool) {
        self.is_power_protected = Some(value);
    }

    /// Gets the value of IsPowerProtected
    pub fn get_is_power_protected(&self) -> Option<&bool> {
        self.is_power_protected.as_ref()
    }

    /// Sets the value of IsPrimordial
    pub fn set_is_primordial(&mut self, value: bool) {
        self.is_primordial = Some(value);
    }

    /// Gets the value of IsPrimordial
    pub fn get_is_primordial(&self) -> Option<&bool> {
        self.is_primordial.as_ref()
    }

    /// Sets the value of IsReadOnly
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = Some(value);
    }

    /// Gets the value of IsReadOnly
    pub fn get_is_read_only(&self) -> Option<&bool> {
        self.is_read_only.as_ref()
    }

    /// Sets the value of LogicalSectorSize
    pub fn set_logical_sector_size(&mut self, value: u64) {
        self.logical_sector_size = Some(value);
    }

    /// Gets the value of LogicalSectorSize
    pub fn get_logical_sector_size(&self) -> Option<&u64> {
        self.logical_sector_size.as_ref()
    }

    /// Sets the value of MediaTypeDefault
    pub fn set_media_type_default(&mut self, value: u16) {
        self.media_type_default = Some(value);
    }

    /// Gets the value of MediaTypeDefault
    pub fn get_media_type_default(&self) -> Option<&u16> {
        self.media_type_default.as_ref()
    }

    /// Sets the value of MetadataLength
    pub fn set_metadata_length(&mut self, value: u64) {
        self.metadata_length = Some(value);
    }

    /// Gets the value of MetadataLength
    pub fn get_metadata_length(&self) -> Option<&u64> {
        self.metadata_length.as_ref()
    }

    /// Sets the value of MinimumAllocationSize
    pub fn set_minimum_allocation_size(&mut self, value: u64) {
        self.minimum_allocation_size = Some(value);
    }

    /// Gets the value of MinimumAllocationSize
    pub fn get_minimum_allocation_size(&self) -> Option<&u64> {
        self.minimum_allocation_size.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<StoragePool_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<StoragePool_OperationalStatus> {
        &self.operational_status
    }

    /// Sets the value of OtherOperationalStatusDescription
    pub fn set_other_operational_status_description(&mut self, value: String) {
        self.other_operational_status_description = Some(value);
    }

    /// Gets the value of OtherOperationalStatusDescription
    pub fn get_other_operational_status_description(&self) -> Option<&String> {
        self.other_operational_status_description.as_ref()
    }

    /// Sets the value of OtherUsageDescription
    pub fn set_other_usage_description(&mut self, value: String) {
        self.other_usage_description = Some(value);
    }

    /// Gets the value of OtherUsageDescription
    pub fn get_other_usage_description(&self) -> Option<&String> {
        self.other_usage_description.as_ref()
    }

    /// Sets the value of PhysicalSectorSize
    pub fn set_physical_sector_size(&mut self, value: u64) {
        self.physical_sector_size = Some(value);
    }

    /// Gets the value of PhysicalSectorSize
    pub fn get_physical_sector_size(&self) -> Option<&u64> {
        self.physical_sector_size.as_ref()
    }

    /// Sets the value of ProvisioningTypeDefault
    pub fn set_provisioning_type_default(&mut self, value: StoragePool_ProvisioningTypeDefault) {
        self.provisioning_type_default = Some(value);
    }

    /// Gets the value of ProvisioningTypeDefault
    pub fn get_provisioning_type_default(&self) -> Option<&StoragePool_ProvisioningTypeDefault> {
        self.provisioning_type_default.as_ref()
    }

    /// Sets the value of ReadOnlyReason
    pub fn set_read_only_reason(&mut self, value: StoragePool_ReadOnlyReason) {
        self.read_only_reason = Some(value);
    }

    /// Gets the value of ReadOnlyReason
    pub fn get_read_only_reason(&self) -> Option<&StoragePool_ReadOnlyReason> {
        self.read_only_reason.as_ref()
    }

    /// Sets the value of RepairPolicy
    pub fn set_repair_policy(&mut self, value: StoragePool_RepairPolicy) {
        self.repair_policy = Some(value);
    }

    /// Gets the value of RepairPolicy
    pub fn get_repair_policy(&self) -> Option<&StoragePool_RepairPolicy> {
        self.repair_policy.as_ref()
    }

    /// Sets the value of ResiliencySettingNameDefault
    pub fn set_resiliency_setting_name_default(&mut self, value: String) {
        self.resiliency_setting_name_default = Some(value);
    }

    /// Gets the value of ResiliencySettingNameDefault
    pub fn get_resiliency_setting_name_default(&self) -> Option<&String> {
        self.resiliency_setting_name_default.as_ref()
    }

    /// Sets the value of RetireMissingPhysicalDisks
    pub fn set_retire_missing_physical_disks(&mut self, value: StoragePool_RetireMissingPhysicalDisks) {
        self.retire_missing_physical_disks = Some(value);
    }

    /// Gets the value of RetireMissingPhysicalDisks
    pub fn get_retire_missing_physical_disks(&self) -> Option<&StoragePool_RetireMissingPhysicalDisks> {
        self.retire_missing_physical_disks.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of SupportedProvisioningTypes
    pub fn set_supported_provisioning_types(&mut self, value: Vec<StoragePool_SupportedProvisioningTypes>) {
        self.supported_provisioning_types = value;
    }

    /// Gets the value of SupportedProvisioningTypes
    pub fn get_supported_provisioning_types(&self) -> &Vec<StoragePool_SupportedProvisioningTypes> {
        &self.supported_provisioning_types
    }

    /// Sets the value of SupportsDeduplication
    pub fn set_supports_deduplication(&mut self, value: bool) {
        self.supports_deduplication = Some(value);
    }

    /// Gets the value of SupportsDeduplication
    pub fn get_supports_deduplication(&self) -> Option<&bool> {
        self.supports_deduplication.as_ref()
    }

    /// Sets the value of ThinProvisioningAlertThresholds
    pub fn set_thin_provisioning_alert_thresholds(&mut self, value: Vec<u16>) {
        self.thin_provisioning_alert_thresholds = value;
    }

    /// Gets the value of ThinProvisioningAlertThresholds
    pub fn get_thin_provisioning_alert_thresholds(&self) -> &Vec<u16> {
        &self.thin_provisioning_alert_thresholds
    }

    /// Sets the value of Usage
    pub fn set_usage(&mut self, value: StoragePool_Usage) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&StoragePool_Usage> {
        self.usage.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: StoragePool_Version) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&StoragePool_Version> {
        self.version.as_ref()
    }

    /// Sets the value of WriteCacheSizeDefault
    pub fn set_write_cache_size_default(&mut self, value: u64) {
        self.write_cache_size_default = Some(value);
    }

    /// Gets the value of WriteCacheSizeDefault
    pub fn get_write_cache_size_default(&self) -> Option<&u64> {
        self.write_cache_size_default.as_ref()
    }

    /// Sets the value of WriteCacheSizeMax
    pub fn set_write_cache_size_max(&mut self, value: u64) {
        self.write_cache_size_max = Some(value);
    }

    /// Gets the value of WriteCacheSizeMax
    pub fn get_write_cache_size_max(&self) -> Option<&u64> {
        self.write_cache_size_max.as_ref()
    }

    /// Sets the value of WriteCacheSizeMin
    pub fn set_write_cache_size_min(&mut self, value: u64) {
        self.write_cache_size_min = Some(value);
    }

    /// Gets the value of WriteCacheSizeMin
    pub fn get_write_cache_size_min(&self) -> Option<&u64> {
        self.write_cache_size_min.as_ref()
    }

/// This method creates a virtual disk using the resources of the storage pool. This method is available only when the SupportsVirtualDiskCreation property on the storage subsystem is set to TRUE. If it is set to FALSE, this method will fail with MI_RESULT_NOT_SUPPORTED. This method is also not supported for primordial pools. 
/// Creating tiered virtual disks is available only when the SupportsStorageTieredVirtualDiskCreation property on the storage subsystem is set to TRUE. If it is set to FALSE, this method will fail with MI_RESULT_NOT_SUPPORTED. 
/// CreateVirtualDisk requires only FriendlyName and Size to be specified. Sizes can be specified explicitly through the Size parameter, or you can use the maximum available space from the storage pool by specifying the UseMaximumSize parameter. Both FriendlyName and Size are treated as goals rather than hard requirements. For example, not all SMI-S based arrays support custom friendly names; however, the virtual disk creation will still succeed. If the size specified is not achieved, the actual size used for the virtual disk will be returned in the out parameter structure. 
/// The usage of this virtual disk can be set using the Usage and OtherUsageDescription parameters. If a value for OtherUsageDescription is given, Usage must be set to 1 - 'Other', otherwise an error will be returned. 
/// By default, the resiliency setting applied to this virtual disk will be whatever is specified in the storage pool's ResiliencySettingNameDefault property. This can be overridden using the ResiliencySettingName parameter. Note that the name given here must correspond to a resiliency setting associated with this storage pool. Any other value will result in an error. 
/// Individual settings of the resiliency setting can be overridden using the NumberOfDataCopies, PhysicalDiskRedundancy, NumberOfColumns, and Interleave parameters. If these parameters are not used, the defaults from the resiliency setting will be used. These overrides will not persist back to the particular resiliency setting instance; however some storage providers may choose to create a new resiliency setting instance to capture this new configuration. If any of the goals specified in the override parameters are out of range, or are not supported by the storage pool, an error will be returned. 
/// The provisioning policy for the virtual disk is determined in a similar way to the resiliency setting. If no preference is specified in the ProvisioningType parameter, the policy is determined by the storage pool's ProvisioningTypeDefault property. If the ProvisioningType parameter is specified, the default is ignored and the value specified will be used instead. 
/// Allocation can be further controlled by the PhysicalDisksToUse parameter. There may be certain scenarios where a storage administrator wants to manually choose which physical disks should back the virtual disk. When this parameter is specified, data for the virtual disk will only be stored on the physical disks in this array and not on any others.

    /// * `auto_number_of_columns` - If TRUE, this field instructs the storage provider (or subsystem) to automatically pick what it determines to be the best number of columns for the virtual disk. If this field is TRUE, then the NumberOfColumns parameter must be NULL. (bool)
    /// * `auto_write_cache_size` - Indicates whether the provider should pick up the auto write cache size (bool)
    /// * `friendly_name` - This parameter allows the user to specify the FriendlyName at the time of the virtual disk creation. FriendlyNames are expected to be descriptive, however they are not required to be unique. Note that some storage subsystems do not allow setting a friendly name during virtual disk creation. If a subsystem doesn't support this, virtual disk creation should still succeed, however the disk may have a different name assigned to it. (String)
    /// * `interleave` - Specifies the number of bytes that should be used for a strip in the common striping-based resiliency settings. The strip is defined as the size of the portion of a stripe that lies on one physical disk. Thus Interleave * NumberOfColumns will yield the size of one stripe of user data. If this parameter is specified, this value will override the InterleaveDefault which would have been inherited from the resiliency setting specified by ResiliencySettingName. (u64)
    /// * `is_enclosure_aware` - Determines the allocation behavior for this virtual disk. Enclosure aware virtual disks will intelligently pick the physical disks to use for their redundancy. If TRUE, the virtual disk will attempt to use physical disks from different enclosures to balance the fault tolerance between two (or more) physical enclosures. (bool)
    /// * `number_of_columns` - Specifies the number of underlying physical disks across which data should be striped. If specified, this value will override the NumberOfColumnsDefault value that would have been inherited from the resiliency setting specified by ResiliencySettingName. (u16)
    /// * `number_of_data_copies` - Specifies the number of complete data copies to maintain for this virtual disk. If specified, this value will override the NumberOfDataCopiesDefault value that would have been inherited from the resiliency setting specified by ResiliencySettingName. (u16)
    /// * `other_usage_description` - Allows a user to set a vendor specific usage for the new virtual disk object. This parameter can only be specified if the Usage parameter is set to 1 - 'Other'. (String)
    /// * `physical_disk_redundancy` - Specifies how many physical disk failures the virtual disk should be able to withstand before data loss occurs. If specified, this value will override the PhysicalDiskRedundancyDefault value that would have been inherited from the resiliency setting specified by ResiliencySettingName. (u16)
    /// * `physical_disks_to_use` - If specified, allocation of this virtual disk's storage is limited to the physical disks in the list. These physical disks must already be added to this storage pool. (MSFT_PhysicalDisk[])
    /// * `provisioning_type` - Denotes the provisioning type of the virtual disk.  1 - 'Thin': The storage for the virtual disk is allocated on-demand.  2 - 'Fixed': The storage for the virtual disk is allocated up front. (StoragePool_ProvisioningType)
    /// * `resiliency_setting_name` - This parameter specifies the resiliency setting to use as a template for this virtual disk. This property's value should correspond with the particular resiliency setting instance's Name property. Only resiliency settings associated with this storage pool may be used. (String)
    /// * `size` - Indicates the size for the virtual disk. Note that some storage subsystems will round the size up or down to a multiple of its allocation unit size. This parameter cannot be used if UseMaximumSize is set to TRUE. (u64)
    /// * `storage_tiers` - Storage tiers on this virtual disk (MSFT_StorageTier[])
    /// * `storage_tier_sizes` - Sizes of each tier (u64[])
    /// * `usage` - Denotes the intended usage of the virtual disk (StoragePool_Usage)
    /// * `use_maximum_size` - UseMaximumSize instructs the storage array to create the largest possible virtual disk given the available resources of this storage pool. This parameter cannot be used if the Size parameter is set. (bool)
    /// * `write_cache_size` - Size of write cache on the virtual disk (u64)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. When the operation has completed, an association should exist between the storage job and the created objects. (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_virtual_disk(&self, friendly_name: &String, size: u64, use_maximum_size: bool, provisioning_type: StoragePool_ProvisioningType, resiliency_setting_name: &String, usage: StoragePool_Usage, other_usage_description: &String, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, auto_number_of_columns: bool, interleave: u64, is_enclosure_aware: bool, physical_disks_to_use: &Vec<MSFT_PhysicalDisk>, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, write_cache_size: u64, auto_write_cache_size: bool, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "UseMaximumSize".to_string(), value: use_maximum_size.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "AutoNumberOfColumns".to_string(), value: auto_number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "IsEnclosureAware".to_string(), value: is_enclosure_aware.into() });
        args.push(MethodParameter { name: "PhysicalDisksToUse".to_string(), value: physical_disks_to_use.into() });
        args.push(MethodParameter { name: "StorageTiers".to_string(), value: storage_tiers.into() });
        args.push(MethodParameter { name: "StorageTierSizes".to_string(), value: storage_tier_sizes.into() });
        args.push(MethodParameter { name: "WriteCacheSize".to_string(), value: write_cache_size.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });

        let result = self.invoke_method("CreateVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `allocation_unit_size` -  (u64)
    /// * `auto_number_of_columns` -  (bool)
    /// * `auto_write_cache_size` -  (bool)
    /// * `column_isolation` -  (u16)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `media_type` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `other_usage_description` -  (String)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `physical_disks_to_use` -  (MSFT_PhysicalDisk[])
    /// * `provisioning_type` -  (u16)
    /// * `read_cache_size` -  (u64)
    /// * `resiliency_setting_name` -  (String)
    /// * `size` -  (u64)
    /// * `storage_tiers` -  (MSFT_StorageTier[])
    /// * `storage_tier_sizes` -  (u64[])
    /// * `usage` -  (u16)
    /// * `use_maximum_size` -  (bool)
    /// * `write_cache_size` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_virtual_disk2(&self, friendly_name: &String, size: u64, use_maximum_size: bool, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, resiliency_setting_name: &String, usage: u16, other_usage_description: &String, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, auto_number_of_columns: bool, interleave: u64, number_of_groups: u16, fault_domain_awareness: u16, column_isolation: u16, physical_disks_to_use: &Vec<MSFT_PhysicalDisk>, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, write_cache_size: u64, auto_write_cache_size: bool, read_cache_size: u64, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "UseMaximumSize".to_string(), value: use_maximum_size.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "AutoNumberOfColumns".to_string(), value: auto_number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "PhysicalDisksToUse".to_string(), value: physical_disks_to_use.into() });
        args.push(MethodParameter { name: "StorageTiers".to_string(), value: storage_tiers.into() });
        args.push(MethodParameter { name: "StorageTierSizes".to_string(), value: storage_tier_sizes.into() });
        args.push(MethodParameter { name: "WriteCacheSize".to_string(), value: write_cache_size.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });
        args.push(MethodParameter { name: "ReadCacheSize".to_string(), value: read_cache_size.into() });

        let result = self.invoke_method("CreateVirtualDisk2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `add_to_cluster` -  (bool)
    /// * `allocation_unit_size` -  (u64)
    /// * `auto_number_of_columns` -  (bool)
    /// * `auto_write_cache_size` -  (bool)
    /// * `column_isolation` -  (u16)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `is_manual_attach` -  (bool)
    /// * `media_type` -  (u16)
    /// * `minimum_logical_data_copies` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `other_usage_description` -  (String)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `read_cache_size` -  (u64)
    /// * `resiliency_setting_name` -  (String)
    /// * `size` -  (u64)
    /// * `storage_fault_domains_to_use` -  (MSFT_StorageFaultDomain[])
    /// * `storage_tiers` -  (MSFT_StorageTier[])
    /// * `storage_tier_sizes` -  (u64[])
    /// * `track_valid_data` -  (bool)
    /// * `usage` -  (u16)
    /// * `use_maximum_size` -  (bool)
    /// * `write_cache_reserve_size` -  (u64)
    /// * `write_cache_size` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_virtual_disk3(&self, friendly_name: &String, size: u64, use_maximum_size: bool, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, resiliency_setting_name: &String, usage: u16, other_usage_description: &String, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, auto_number_of_columns: bool, interleave: u64, number_of_groups: u16, fault_domain_awareness: u16, column_isolation: u16, minimum_logical_data_copies: u16, storage_fault_domains_to_use: &Vec<MSFT_StorageFaultDomain>, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, write_cache_size: u64, auto_write_cache_size: bool, write_cache_reserve_size: u64, read_cache_size: u64, track_valid_data: bool, is_manual_attach: bool, add_to_cluster: bool, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "UseMaximumSize".to_string(), value: use_maximum_size.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "AutoNumberOfColumns".to_string(), value: auto_number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "MinimumLogicalDataCopies".to_string(), value: minimum_logical_data_copies.into() });
        args.push(MethodParameter { name: "StorageFaultDomainsToUse".to_string(), value: storage_fault_domains_to_use.into() });
        args.push(MethodParameter { name: "StorageTiers".to_string(), value: storage_tiers.into() });
        args.push(MethodParameter { name: "StorageTierSizes".to_string(), value: storage_tier_sizes.into() });
        args.push(MethodParameter { name: "WriteCacheSize".to_string(), value: write_cache_size.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });
        args.push(MethodParameter { name: "WriteCacheReserveSize".to_string(), value: write_cache_reserve_size.into() });
        args.push(MethodParameter { name: "ReadCacheSize".to_string(), value: read_cache_size.into() });
        args.push(MethodParameter { name: "TrackValidData".to_string(), value: track_valid_data.into() });
        args.push(MethodParameter { name: "IsManualAttach".to_string(), value: is_manual_attach.into() });
        args.push(MethodParameter { name: "AddToCluster".to_string(), value: add_to_cluster.into() });

        let result = self.invoke_method("CreateVirtualDisk3", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method creates a virtual disk and single volume using the resources of the storage pool.

    /// * `access_path` - If set to a valid access path, the system will attempt to use this path as a way to access the local volume. If the access path could not be set, or this parameter was left NULL, a new access path will be automatically assigned. (String)
    /// * `file_server` - The file server that will own this volume. (MSFT_FileServer)
    /// * `file_system` - Specifies the file system to format the created volume. Specifying a CSV file system is only supported on a storage spaces subsystem. For CSV the pool must be clusterable and the volume created will be a cluster shared volume. (StoragePool_FileSystem)
    /// * `friendly_name` - This parameter allows the user to specify the FriendlyName at the time of the volume creation. FriendlyNames are expected to be descriptive, however they are not required to be unique. The filesystem's label will also be set to this friendly name. (String)
    /// * `number_of_columns` - Specifies the number of underlying physical disks across which data should be striped. If specified, this value will override the NumberOfColumnsDefault value that would have been inherited from the resiliency setting specified by ResiliencySettingName. (u16)
    /// * `physical_disk_redundancy` - Specifies how many physical disk failures the virtual disk should be able to withstand before data loss occurs. If specified, this value will override the PhysicalDiskRedundancyDefault which would have been inherited from the resiliency setting specified by ResiliencySettingName. (u16)
    /// * `provisioning_type` - Denotes the provisioning type of the volume.  1 - 'Thin': The storage for the volume is allocated on-demand.  2 - 'Fixed': The storage for the volume is allocated up front. (StoragePool_ProvisioningType)
    /// * `resiliency_setting_name` - This parameter specifies the resiliency setting to use as a template for this volume. This property's value should correspond with the particular resiliency setting instance's Name property. Only resiliency settings associated with this storage pool may be used. (String)
    /// * `size` - Indicates the size for the virtual disk. Note that some storage subsystems will round the size up or down to a multiple of its allocation unit size. The size of the resulting volume will be the maximum size possible for the resulting virtual disk. (u64)
    /// * `storage_tiers` - Storage tiers on this virtual disk (MSFT_StorageTier[])
    /// * `storage_tier_sizes` - Sizes of each tier (u64[])

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. When the operation has completed, an association should exist between the storage job and the created objects. (MSFT_StorageJob)
    /// * `created_volume` -  (MSFT_Volume)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_volume(&self, friendly_name: &String, size: u64, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, provisioning_type: StoragePool_ProvisioningType, resiliency_setting_name: &String, physical_disk_redundancy: u16, number_of_columns: u16, file_system: StoragePool_FileSystem, access_path: &String, file_server: MSFT_FileServer, created_volume: &mut MSFT_Volume, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "StorageTiers".to_string(), value: storage_tiers.into() });
        args.push(MethodParameter { name: "StorageTierSizes".to_string(), value: storage_tier_sizes.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });
        args.push(MethodParameter { name: "FileServer".to_string(), value: file_server.into() });

        let result = self.invoke_method("CreateVolume", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_volume = result.get_value("CreatedVolume")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_path` -  (String)
    /// * `allocation_unit_size` -  (u32)
    /// * `file_server` -  (MSFT_FileServer)
    /// * `file_system` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `number_of_columns` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `read_cache_size` -  (u64)
    /// * `resiliency_setting_name` -  (String)
    /// * `size` -  (u64)
    /// * `storage_tiers` -  (MSFT_StorageTier[])
    /// * `storage_tier_sizes` -  (u64[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_volume` -  (MSFT_Volume)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_volume2(&self, friendly_name: &String, size: u64, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, provisioning_type: u16, resiliency_setting_name: &String, physical_disk_redundancy: u16, number_of_columns: u16, file_system: u16, access_path: &String, allocation_unit_size: u32, read_cache_size: u64, file_server: MSFT_FileServer, created_volume: &mut MSFT_Volume, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "StorageTiers".to_string(), value: storage_tiers.into() });
        args.push(MethodParameter { name: "StorageTierSizes".to_string(), value: storage_tier_sizes.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "ReadCacheSize".to_string(), value: read_cache_size.into() });
        args.push(MethodParameter { name: "FileServer".to_string(), value: file_server.into() });

        let result = self.invoke_method("CreateVolume2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_volume = result.get_value("CreatedVolume")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// Creates a storage tier template on the storage pool. This method is available only when the SupportsStorageTierCreation property on the storage subsystem is set to TRUE. If it is set to FALSE, this method will fail with MI_RESULT_NOT_SUPPORTED. This method is also not supported for primordial pools.

    /// * `description` - Description of the storage tier (String)
    /// * `friendly_name` - Friendly name of the storage tier (String)
    /// * `media_type` - Media type of the storage tier (StoragePool_MediaType)

    /// * `created_storage_job` - If RunAsJob is set to TRUE and this method takes a while to execute, this parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `created_storage_tier` -  (MSFT_StorageTier)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_tier(&self, friendly_name: &String, media_type: StoragePool_MediaType, description: &String, created_storage_tier: &mut MSFT_StorageTier, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("CreateStorageTier", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_tier = result.get_value("CreatedStorageTier")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `column_isolation` -  (u16)
    /// * `description` -  (String)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `media_type` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `resiliency_setting_name` -  (String)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_storage_tier` -  (MSFT_StorageTier)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_tier2(&self, friendly_name: &String, provisioning_type: u16, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, resiliency_setting_name: &String, interleave: u64, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, physical_disk_redundancy: u16, description: &String, created_storage_tier: &mut MSFT_StorageTier, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("CreateStorageTier2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_tier = result.get_value("CreatedStorageTier")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `allocation_unit_size` -  (u64)
    /// * `column_isolation` -  (u16)
    /// * `description` -  (String)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `media_type` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `resiliency_setting_name` -  (String)
    /// * `storage_fault_domains_to_use` -  (MSFT_StorageFaultDomain[])
    /// * `usage` -  (u16)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_storage_tier` -  (MSFT_StorageTier)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_tier3(&self, friendly_name: &String, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, storage_fault_domains_to_use: &Vec<MSFT_StorageFaultDomain>, resiliency_setting_name: &String, usage: u16, interleave: u64, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, physical_disk_redundancy: u16, description: &String, created_storage_tier: &mut MSFT_StorageTier, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "StorageFaultDomainsToUse".to_string(), value: storage_fault_domains_to_use.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });

        let result = self.invoke_method("CreateStorageTier3", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_tier = result.get_value("CreatedStorageTier")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method deletes an empty storage pool. If the storage pool contains any virtual disks, these virtual disks should be removed first.

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method will upgrade the version of the storage pool.

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn upgrade(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Upgrade", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn optimize(&self, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Optimize", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method will add one or more physical disks from the primordial storage pool to an existing concrete storage pool.

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `usage` -  (StoragePool_Usage)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_physical_disk(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, usage: StoragePool_Usage, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });

        let result = self.invoke_method("AddPhysicalDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `usage` -  (u16)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_physical_disk2(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, usage: u16, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });

        let result = self.invoke_method("AddPhysicalDisk2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method removes one or more physical disks from the pool and returns all previously allocated space on the disk to the available capacity in the primordial pool.

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_physical_disk(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });

        let result = self.invoke_method("RemovePhysicalDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_physical_disk2(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });

        let result = self.invoke_method("RemovePhysicalDisk2", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method returns the supported sizes for a virtual disk created on this storage pool. These sizes can either be returned in an array of all supported sizes, through a min, max, and divisor, or both.

    /// * `resiliency_setting_name` - Specifies the name of the resiliency setting that should be used when determining the supported sizes. Note that the sizes returned may be different depending on the resiliency setting. (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_sizes` - This output parameter will contain an array of all of the supported sizes by the storage pool. This parameter may be NULL if the number of supported sizes is large, but is useful for storage pools that support only a select number of virtual disk sizes. (u64[])
    /// * `virtual_disk_size_divisor` - This parameter indicates the interval in which the supported sizes increment. For example: If the minimum supported size is 10 GB, and this parameter is 2 GB, then the supported sizes for this pool would be 10 GB, 12 GB, 14 GB, etc. until the maximum supported size is reached. (u64)
    /// * `virtual_disk_size_max` - This parameter denotes the maximum supported size that a virtual disk created in this pool can be. (u64)
    /// * `virtual_disk_size_min` - This parameter denotes the minimum supported size that a virtual disk created in this pool can be. (u64)
    pub fn get_supported_size(&self, resiliency_setting_name: &String, supported_sizes: &mut Vec<u64>, virtual_disk_size_min: &mut u64, virtual_disk_size_max: &mut u64, virtual_disk_size_divisor: &mut u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });

        let result = self.invoke_method("GetSupportedSize", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_sizes = result.get_value("SupportedSizes")?;
        let virtual_disk_size_divisor = result.get_value("VirtualDiskSizeDivisor")?;
        let virtual_disk_size_max = result.get_value("VirtualDiskSizeMax")?;
        let virtual_disk_size_min = result.get_value("VirtualDiskSizeMin")?;
        Ok(result.return_value)

    }


/// 

    /// * `fault_domain_awareness` -  (u16)
    /// * `resiliency_setting_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_sizes` -  (u64[])
    /// * `virtual_disk_size_divisor` -  (u64)
    /// * `virtual_disk_size_max` -  (u64)
    /// * `virtual_disk_size_min` -  (u64)
    pub fn get_supported_size2(&self, resiliency_setting_name: &String, fault_domain_awareness: u16, supported_sizes: &mut Vec<u64>, virtual_disk_size_min: &mut u64, virtual_disk_size_max: &mut u64, virtual_disk_size_divisor: &mut u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });

        let result = self.invoke_method("GetSupportedSize2", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_sizes = result.get_value("SupportedSizes")?;
        let virtual_disk_size_divisor = result.get_value("VirtualDiskSizeDivisor")?;
        let virtual_disk_size_max = result.get_value("VirtualDiskSizeMax")?;
        let virtual_disk_size_min = result.get_value("VirtualDiskSizeMin")?;
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


/// This method allows the storage pool to be renamed.

    /// * `friendly_name` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_friendly_name(&self, friendly_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });

        let result = self.invoke_method("SetFriendlyName", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the storage pool's intended usage to be updated. Not all storage pools may allow this and will return 1 - 'Not Supported' if this operation cannot be performed.

    /// * `other_usage_description` - If Usage is set to 1 - 'Other', this parameter takes in the string representation of a vendor defined usage for this storage pool. This parameter must not be set if Usage is a value other than 1 - 'Other'. (String)
    /// * `usage` - Denotes the new intended usage of the storage pool. (StoragePool_Usage)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_usage(&self, usage: StoragePool_Usage, other_usage_description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });

        let result = self.invoke_method("SetUsage", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the user to update or set various defaults on the storage pool. Note that not all parameters must be specified, and only those given will be updated.

    /// * `auto_write_cache_size` - Indicates whether the provider should pick up the auto write cache size (bool)
    /// * `enclosure_aware_default` - This parameter indicates the default allocation policy for virtual disks created in an enclosure aware storage pool. For example, an enclosure aware subsystem could balance each data copy of the virtual disk across multiple physical enclosures such that each enclosure contains a full data copy of the virtual disk. (bool)
    /// * `provisioning_type_default` - Specifies the new default provisioning type of the storage pool. (StoragePool_ProvisioningTypeDefault)
    /// * `resiliency_setting_name_default` - Specifies the new default resiliency setting that should be used by this storage pool. The resiliency setting specified must already be associated with this storage pool. (String)
    /// * `write_cache_size_default` - New default size of write cache for virtual disk creation (u64)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_defaults(&self, provisioning_type_default: StoragePool_ProvisioningTypeDefault, resiliency_setting_name_default: &String, enclosure_aware_default: bool, write_cache_size_default: u64, auto_write_cache_size: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProvisioningTypeDefault".to_string(), value: provisioning_type_default.into() });
        args.push(MethodParameter { name: "ResiliencySettingNameDefault".to_string(), value: resiliency_setting_name_default.into() });
        args.push(MethodParameter { name: "EnclosureAwareDefault".to_string(), value: enclosure_aware_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });

        let result = self.invoke_method("SetDefaults", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `auto_write_cache_size` -  (bool)
    /// * `fault_domain_awareness_default` -  (u16)
    /// * `media_type_default` -  (u16)
    /// * `provisioning_type_default` -  (u16)
    /// * `resiliency_setting_name_default` -  (String)
    /// * `write_cache_size_default` -  (u64)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_defaults2(&self, provisioning_type_default: u16, media_type_default: u16, resiliency_setting_name_default: &String, fault_domain_awareness_default: u16, write_cache_size_default: u64, auto_write_cache_size: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProvisioningTypeDefault".to_string(), value: provisioning_type_default.into() });
        args.push(MethodParameter { name: "MediaTypeDefault".to_string(), value: media_type_default.into() });
        args.push(MethodParameter { name: "ResiliencySettingNameDefault".to_string(), value: resiliency_setting_name_default.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });

        let result = self.invoke_method("SetDefaults2", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the user to update or set various attributes on the storage pool. Note that not all parameters must be specified, and only those given will be updated.

    /// * `clear_on_deallocate` -  (bool)
    /// * `is_power_protected` -  (bool)
    /// * `is_read_only` -  (bool)
    /// * `repair_policy` -  (StoragePool_RepairPolicy)
    /// * `retire_missing_physical_disks` -  (StoragePool_RetireMissingPhysicalDisks)
    /// * `thin_provisioning_alert_thresholds` - Percentages at which an alert should be generated (u16[])

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, is_read_only: bool, clear_on_deallocate: bool, is_power_protected: bool, repair_policy: StoragePool_RepairPolicy, retire_missing_physical_disks: StoragePool_RetireMissingPhysicalDisks, thin_provisioning_alert_thresholds: &Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsReadOnly".to_string(), value: is_read_only.into() });
        args.push(MethodParameter { name: "ClearOnDeallocate".to_string(), value: clear_on_deallocate.into() });
        args.push(MethodParameter { name: "IsPowerProtected".to_string(), value: is_power_protected.into() });
        args.push(MethodParameter { name: "RepairPolicy".to_string(), value: repair_policy.into() });
        args.push(MethodParameter { name: "RetireMissingPhysicalDisks".to_string(), value: retire_missing_physical_disks.into() });
        args.push(MethodParameter { name: "ThinProvisioningAlertThresholds".to_string(), value: thin_provisioning_alert_thresholds.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

