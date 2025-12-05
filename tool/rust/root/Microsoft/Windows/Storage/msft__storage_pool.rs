// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StoragePool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StoragePool {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// 
    #[serde(rename = "ClearOnDeallocate")]
    pub clear_on_deallocate: Option<bool>,

/// 
    #[serde(rename = "EnclosureAwareDefault")]
    pub enclosure_aware_default: Option<bool>,

/// 
    #[serde(rename = "FaultDomainAwarenessDefault")]
    pub fault_domain_awareness_default: Option<u16>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "IsClustered")]
    pub is_clustered: Option<bool>,

/// 
    #[serde(rename = "IsPowerProtected")]
    pub is_power_protected: Option<bool>,

/// 
    #[serde(rename = "IsPrimordial")]
    pub is_primordial: Option<bool>,

/// 
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// 
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u64>,

/// 
    #[serde(rename = "MediaTypeDefault")]
    pub media_type_default: Option<u16>,

/// 
    #[serde(rename = "MetadataLength")]
    pub metadata_length: Option<u64>,

/// 
    #[serde(rename = "MinimumAllocationSize")]
    pub minimum_allocation_size: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// 
    #[serde(rename = "OtherUsageDescription")]
    pub other_usage_description: Option<String>,

/// 
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u64>,

/// 
    #[serde(rename = "ProvisioningTypeDefault")]
    pub provisioning_type_default: Option<u16>,

/// 
    #[serde(rename = "ReadOnlyReason")]
    pub read_only_reason: Option<u16>,

/// 
    #[serde(rename = "RepairPolicy")]
    pub repair_policy: Option<u16>,

/// 
    #[serde(rename = "ResiliencySettingNameDefault")]
    pub resiliency_setting_name_default: Option<String>,

/// 
    #[serde(rename = "RetireMissingPhysicalDisks")]
    pub retire_missing_physical_disks: Option<u16>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "SupportedProvisioningTypes")]
    pub supported_provisioning_types: Vec<u16>,

/// 
    #[serde(rename = "SupportsDeduplication")]
    pub supports_deduplication: Option<bool>,

/// 
    #[serde(rename = "ThinProvisioningAlertThresholds")]
    pub thin_provisioning_alert_thresholds: Vec<u16>,

/// 
    #[serde(rename = "Usage")]
    pub usage: Option<u16>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<u16>,

/// 
    #[serde(rename = "WriteCacheSizeDefault")]
    pub write_cache_size_default: Option<u64>,

/// 
    #[serde(rename = "WriteCacheSizeMax")]
    pub write_cache_size_max: Option<u64>,

/// 
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
    pub fn set_fault_domain_awareness_default(&mut self, value: u16) {
        self.fault_domain_awareness_default = Some(value);
    }

    /// Gets the value of FaultDomainAwarenessDefault
    pub fn get_fault_domain_awareness_default(&self) -> Option<&u16> {
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
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
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
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
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
    pub fn set_provisioning_type_default(&mut self, value: u16) {
        self.provisioning_type_default = Some(value);
    }

    /// Gets the value of ProvisioningTypeDefault
    pub fn get_provisioning_type_default(&self) -> Option<&u16> {
        self.provisioning_type_default.as_ref()
    }

    /// Sets the value of ReadOnlyReason
    pub fn set_read_only_reason(&mut self, value: u16) {
        self.read_only_reason = Some(value);
    }

    /// Gets the value of ReadOnlyReason
    pub fn get_read_only_reason(&self) -> Option<&u16> {
        self.read_only_reason.as_ref()
    }

    /// Sets the value of RepairPolicy
    pub fn set_repair_policy(&mut self, value: u16) {
        self.repair_policy = Some(value);
    }

    /// Gets the value of RepairPolicy
    pub fn get_repair_policy(&self) -> Option<&u16> {
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
    pub fn set_retire_missing_physical_disks(&mut self, value: u16) {
        self.retire_missing_physical_disks = Some(value);
    }

    /// Gets the value of RetireMissingPhysicalDisks
    pub fn get_retire_missing_physical_disks(&self) -> Option<&u16> {
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
    pub fn set_supported_provisioning_types(&mut self, value: Vec<u16>) {
        self.supported_provisioning_types = value;
    }

    /// Gets the value of SupportedProvisioningTypes
    pub fn get_supported_provisioning_types(&self) -> &Vec<u16> {
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
    pub fn set_usage(&mut self, value: u16) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&u16> {
        self.usage.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: u16) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&u16> {
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

/// 

    /// * `add_to_cluster` -  (bool)
    /// * `allocation_unit_size` -  (u64)
    /// * `auto_number_of_columns` -  (bool)
    /// * `auto_write_cache_size` -  (bool)
    /// * `column_isolation` -  (u16)
    /// * `fault_domain_awareness` -  (u16)
    /// * `friendly_name` -  (String)
    /// * `interleave` -  (u64)
    /// * `is_enclosure_aware` -  (bool)
    /// * `is_manual_attach` -  (bool)
    /// * `media_type` -  (u16)
    /// * `minimum_logical_data_copies` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `other_usage_description` -  (String)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `physical_disks_to_use` -  (MSFT_PhysicalDisk[])
    /// * `provisioning_type` -  (u16)
    /// * `read_cache_size` -  (u64)
    /// * `resiliency_setting_name` -  (String)
    /// * `run_as_job` -  (bool)
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
    pub fn create_virtual_disk(&self, friendly_name: &String, size: u64, use_maximum_size: bool, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, resiliency_setting_name: &String, usage: u16, other_usage_description: &String, number_of_data_copies: u16, physical_disk_redundancy: u16, number_of_columns: u16, auto_number_of_columns: bool, interleave: u64, number_of_groups: u16, is_enclosure_aware: bool, fault_domain_awareness: u16, column_isolation: u16, minimum_logical_data_copies: u16, physical_disks_to_use: &Vec<MSFT_PhysicalDisk>, storage_fault_domains_to_use: &Vec<MSFT_StorageFaultDomain>, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, write_cache_size: u64, auto_write_cache_size: bool, write_cache_reserve_size: u64, read_cache_size: u64, track_valid_data: bool, is_manual_attach: bool, add_to_cluster: bool, run_as_job: bool, created_virtual_disk: &mut MSFT_VirtualDisk, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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
        args.push(MethodParameter { name: "IsEnclosureAware".to_string(), value: is_enclosure_aware.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "MinimumLogicalDataCopies".to_string(), value: minimum_logical_data_copies.into() });
        args.push(MethodParameter { name: "PhysicalDisksToUse".to_string(), value: physical_disks_to_use.into() });
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
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateVirtualDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
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
    /// * `run_as_job` -  (bool)
    /// * `size` -  (u64)
    /// * `storage_tiers` -  (MSFT_StorageTier[])
    /// * `storage_tier_sizes` -  (u64[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_volume` -  (MSFT_Volume)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_volume(&self, friendly_name: &String, size: u64, storage_tiers: &Vec<MSFT_StorageTier>, storage_tier_sizes: &Vec<u64>, provisioning_type: u16, resiliency_setting_name: &String, physical_disk_redundancy: u16, number_of_columns: u16, file_system: u16, access_path: &String, allocation_unit_size: u32, read_cache_size: u64, file_server: MSFT_FileServer, created_volume: &mut MSFT_Volume, run_as_job: Option<bool>, created_storage_job: &mut Option<MSFT_StorageJob>, extended_status: &mut Option<MSFT_StorageExtendedStatus>) -> Result<(), WmiError> {
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
        if let Some(val) = run_as_job {
            args.push(MethodParameter { name: "RunAsJob".to_string(), value: val.into() });
        }

        let result = self.invoke_method("CreateVolume", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_volume = result.get_value("CreatedVolume")?;
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
    /// * `run_as_job` -  (bool)
    /// * `storage_fault_domains_to_use` -  (MSFT_StorageFaultDomain[])
    /// * `usage` -  (u16)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_storage_tier` -  (MSFT_StorageTier)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_storage_tier(&self, friendly_name: &String, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, storage_fault_domains_to_use: &Vec<MSFT_StorageFaultDomain>, resiliency_setting_name: &String, usage: u16, interleave: u64, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, physical_disk_redundancy: u16, description: &String, run_as_job: bool, created_storage_tier: &mut MSFT_StorageTier, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateStorageTier", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_tier = result.get_value("CreatedStorageTier")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("DeleteObject", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
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


/// 

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `run_as_job` -  (bool)
    /// * `usage` -  (u16)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_physical_disk(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, usage: u16, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("AddPhysicalDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_physical_disk(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PhysicalDisks".to_string(), value: physical_disks.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("RemovePhysicalDisk", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
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
    pub fn get_supported_size(&self, resiliency_setting_name: &String, fault_domain_awareness: u16, supported_sizes: &mut Vec<u64>, virtual_disk_size_min: &mut u64, virtual_disk_size_max: &mut u64, virtual_disk_size_divisor: &mut u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });

        let result = self.invoke_method("GetSupportedSize", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_sizes = result.get_value("SupportedSizes")?;
        let virtual_disk_size_divisor = result.get_value("VirtualDiskSizeDivisor")?;
        let virtual_disk_size_max = result.get_value("VirtualDiskSizeMax")?;
        let virtual_disk_size_min = result.get_value("VirtualDiskSizeMin")?;
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

    /// * `friendly_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_friendly_name(&self, friendly_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });

        let result = self.invoke_method("SetFriendlyName", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `other_usage_description` -  (String)
    /// * `usage` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_usage(&self, usage: u16, other_usage_description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });

        let result = self.invoke_method("SetUsage", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `auto_write_cache_size` -  (bool)
    /// * `enclosure_aware_default` -  (bool)
    /// * `fault_domain_awareness_default` -  (u16)
    /// * `media_type_default` -  (u16)
    /// * `provisioning_type_default` -  (u16)
    /// * `resiliency_setting_name_default` -  (String)
    /// * `write_cache_size_default` -  (u64)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_defaults(&self, provisioning_type_default: u16, media_type_default: u16, resiliency_setting_name_default: &String, enclosure_aware_default: bool, fault_domain_awareness_default: u16, write_cache_size_default: u64, auto_write_cache_size: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProvisioningTypeDefault".to_string(), value: provisioning_type_default.into() });
        args.push(MethodParameter { name: "MediaTypeDefault".to_string(), value: media_type_default.into() });
        args.push(MethodParameter { name: "ResiliencySettingNameDefault".to_string(), value: resiliency_setting_name_default.into() });
        args.push(MethodParameter { name: "EnclosureAwareDefault".to_string(), value: enclosure_aware_default.into() });
        args.push(MethodParameter { name: "FaultDomainAwarenessDefault".to_string(), value: fault_domain_awareness_default.into() });
        args.push(MethodParameter { name: "WriteCacheSizeDefault".to_string(), value: write_cache_size_default.into() });
        args.push(MethodParameter { name: "AutoWriteCacheSize".to_string(), value: auto_write_cache_size.into() });

        let result = self.invoke_method("SetDefaults", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `clear_on_deallocate` -  (bool)
    /// * `is_power_protected` -  (bool)
    /// * `is_read_only` -  (bool)
    /// * `repair_policy` -  (u16)
    /// * `retire_missing_physical_disks` -  (u16)
    /// * `thin_provisioning_alert_thresholds` -  (u16[])

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, is_read_only: bool, clear_on_deallocate: bool, is_power_protected: bool, repair_policy: u16, retire_missing_physical_disks: u16, thin_provisioning_alert_thresholds: &Vec<u16>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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

