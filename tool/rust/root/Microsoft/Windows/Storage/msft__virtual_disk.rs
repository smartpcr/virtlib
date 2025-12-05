// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDisk {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "Access")]
    pub access: Option<u16>,

/// 
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// 
    #[serde(rename = "AllocationUnitSize")]
    pub allocation_unit_size: Option<u64>,

/// 
    #[serde(rename = "ColumnIsolation")]
    pub column_isolation: Option<u16>,

/// 
    #[serde(rename = "DetachedReason")]
    pub detached_reason: Option<u16>,

/// 
    #[serde(rename = "FaultDomainAwareness")]
    pub fault_domain_awareness: Option<u16>,

/// 
    #[serde(rename = "FootprintOnPool")]
    pub footprint_on_pool: Option<u64>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "Interleave")]
    pub interleave: Option<u64>,

/// 
    #[serde(rename = "IsDeduplicationEnabled")]
    pub is_deduplication_enabled: Option<bool>,

/// 
    #[serde(rename = "IsEnclosureAware")]
    pub is_enclosure_aware: Option<bool>,

/// 
    #[serde(rename = "IsManualAttach")]
    pub is_manual_attach: Option<bool>,

/// 
    #[serde(rename = "IsSnapshot")]
    pub is_snapshot: Option<bool>,

/// 
    #[serde(rename = "IsTiered")]
    pub is_tiered: Option<bool>,

/// 
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u64>,

/// 
    #[serde(rename = "MaxIoBandwidth")]
    pub max_io_bandwidth: Option<u64>,

/// 
    #[serde(rename = "MaxIops")]
    pub max_iops: Option<u64>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u16>,

/// 
    #[serde(rename = "MinimumLogicalDataCopies")]
    pub minimum_logical_data_copies: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NameFormat")]
    pub name_format: Option<u16>,

/// 
    #[serde(rename = "NumberOfAvailableCopies")]
    pub number_of_available_copies: Option<u16>,

/// 
    #[serde(rename = "NumberOfColumns")]
    pub number_of_columns: Option<u16>,

/// 
    #[serde(rename = "NumberOfDataCopies")]
    pub number_of_data_copies: Option<u16>,

/// 
    #[serde(rename = "NumberOfGroups")]
    pub number_of_groups: Option<u16>,

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
    #[serde(rename = "ParityLayout")]
    pub parity_layout: Option<u16>,

/// 
    #[serde(rename = "PhysicalDiskRedundancy")]
    pub physical_disk_redundancy: Option<u16>,

/// 
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u64>,

/// 
    #[serde(rename = "ProvisioningType")]
    pub provisioning_type: Option<u16>,

/// 
    #[serde(rename = "ReadCacheSize")]
    pub read_cache_size: Option<u64>,

/// 
    #[serde(rename = "RequestNoSinglePointOfFailure")]
    pub request_no_single_point_of_failure: Option<bool>,

/// 
    #[serde(rename = "ResiliencySettingName")]
    pub resiliency_setting_name: Option<String>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "TrackValidData")]
    pub track_valid_data: Option<bool>,

/// 
    #[serde(rename = "UniqueIdFormat")]
    pub unique_id_format: Option<u16>,

/// 
    #[serde(rename = "UniqueIdFormatDescription")]
    pub unique_id_format_description: Option<String>,

/// 
    #[serde(rename = "Usage")]
    pub usage: Option<u16>,

/// 
    #[serde(rename = "WriteCacheReserveSize")]
    pub write_cache_reserve_size: Option<u64>,

/// 
    #[serde(rename = "WriteCacheSize")]
    pub write_cache_size: Option<u64>,
}

impl MSFT_VirtualDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            access: None,
            allocated_size: None,
            allocation_unit_size: None,
            column_isolation: None,
            detached_reason: None,
            fault_domain_awareness: None,
            footprint_on_pool: None,
            friendly_name: None,
            health_status: None,
            interleave: None,
            is_deduplication_enabled: None,
            is_enclosure_aware: None,
            is_manual_attach: None,
            is_snapshot: None,
            is_tiered: None,
            logical_sector_size: None,
            max_io_bandwidth: None,
            max_iops: None,
            media_type: None,
            minimum_logical_data_copies: None,
            name: None,
            name_format: None,
            number_of_available_copies: None,
            number_of_columns: None,
            number_of_data_copies: None,
            number_of_groups: None,
            operational_status: Vec::new(),
            other_operational_status_description: None,
            other_usage_description: None,
            parity_layout: None,
            physical_disk_redundancy: None,
            physical_sector_size: None,
            provisioning_type: None,
            read_cache_size: None,
            request_no_single_point_of_failure: None,
            resiliency_setting_name: None,
            size: None,
            track_valid_data: None,
            unique_id_format: None,
            unique_id_format_description: None,
            usage: None,
            write_cache_reserve_size: None,
            write_cache_size: None,
        }
    }


    /// Sets the value of Access
    pub fn set_access(&mut self, value: u16) {
        self.access = Some(value);
    }

    /// Gets the value of Access
    pub fn get_access(&self) -> Option<&u16> {
        self.access.as_ref()
    }

    /// Sets the value of AllocatedSize
    pub fn set_allocated_size(&mut self, value: u64) {
        self.allocated_size = Some(value);
    }

    /// Gets the value of AllocatedSize
    pub fn get_allocated_size(&self) -> Option<&u64> {
        self.allocated_size.as_ref()
    }

    /// Sets the value of AllocationUnitSize
    pub fn set_allocation_unit_size(&mut self, value: u64) {
        self.allocation_unit_size = Some(value);
    }

    /// Gets the value of AllocationUnitSize
    pub fn get_allocation_unit_size(&self) -> Option<&u64> {
        self.allocation_unit_size.as_ref()
    }

    /// Sets the value of ColumnIsolation
    pub fn set_column_isolation(&mut self, value: u16) {
        self.column_isolation = Some(value);
    }

    /// Gets the value of ColumnIsolation
    pub fn get_column_isolation(&self) -> Option<&u16> {
        self.column_isolation.as_ref()
    }

    /// Sets the value of DetachedReason
    pub fn set_detached_reason(&mut self, value: u16) {
        self.detached_reason = Some(value);
    }

    /// Gets the value of DetachedReason
    pub fn get_detached_reason(&self) -> Option<&u16> {
        self.detached_reason.as_ref()
    }

    /// Sets the value of FaultDomainAwareness
    pub fn set_fault_domain_awareness(&mut self, value: u16) {
        self.fault_domain_awareness = Some(value);
    }

    /// Gets the value of FaultDomainAwareness
    pub fn get_fault_domain_awareness(&self) -> Option<&u16> {
        self.fault_domain_awareness.as_ref()
    }

    /// Sets the value of FootprintOnPool
    pub fn set_footprint_on_pool(&mut self, value: u64) {
        self.footprint_on_pool = Some(value);
    }

    /// Gets the value of FootprintOnPool
    pub fn get_footprint_on_pool(&self) -> Option<&u64> {
        self.footprint_on_pool.as_ref()
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

    /// Sets the value of Interleave
    pub fn set_interleave(&mut self, value: u64) {
        self.interleave = Some(value);
    }

    /// Gets the value of Interleave
    pub fn get_interleave(&self) -> Option<&u64> {
        self.interleave.as_ref()
    }

    /// Sets the value of IsDeduplicationEnabled
    pub fn set_is_deduplication_enabled(&mut self, value: bool) {
        self.is_deduplication_enabled = Some(value);
    }

    /// Gets the value of IsDeduplicationEnabled
    pub fn get_is_deduplication_enabled(&self) -> Option<&bool> {
        self.is_deduplication_enabled.as_ref()
    }

    /// Sets the value of IsEnclosureAware
    pub fn set_is_enclosure_aware(&mut self, value: bool) {
        self.is_enclosure_aware = Some(value);
    }

    /// Gets the value of IsEnclosureAware
    pub fn get_is_enclosure_aware(&self) -> Option<&bool> {
        self.is_enclosure_aware.as_ref()
    }

    /// Sets the value of IsManualAttach
    pub fn set_is_manual_attach(&mut self, value: bool) {
        self.is_manual_attach = Some(value);
    }

    /// Gets the value of IsManualAttach
    pub fn get_is_manual_attach(&self) -> Option<&bool> {
        self.is_manual_attach.as_ref()
    }

    /// Sets the value of IsSnapshot
    pub fn set_is_snapshot(&mut self, value: bool) {
        self.is_snapshot = Some(value);
    }

    /// Gets the value of IsSnapshot
    pub fn get_is_snapshot(&self) -> Option<&bool> {
        self.is_snapshot.as_ref()
    }

    /// Sets the value of IsTiered
    pub fn set_is_tiered(&mut self, value: bool) {
        self.is_tiered = Some(value);
    }

    /// Gets the value of IsTiered
    pub fn get_is_tiered(&self) -> Option<&bool> {
        self.is_tiered.as_ref()
    }

    /// Sets the value of LogicalSectorSize
    pub fn set_logical_sector_size(&mut self, value: u64) {
        self.logical_sector_size = Some(value);
    }

    /// Gets the value of LogicalSectorSize
    pub fn get_logical_sector_size(&self) -> Option<&u64> {
        self.logical_sector_size.as_ref()
    }

    /// Sets the value of MaxIoBandwidth
    pub fn set_max_io_bandwidth(&mut self, value: u64) {
        self.max_io_bandwidth = Some(value);
    }

    /// Gets the value of MaxIoBandwidth
    pub fn get_max_io_bandwidth(&self) -> Option<&u64> {
        self.max_io_bandwidth.as_ref()
    }

    /// Sets the value of MaxIops
    pub fn set_max_iops(&mut self, value: u64) {
        self.max_iops = Some(value);
    }

    /// Gets the value of MaxIops
    pub fn get_max_iops(&self) -> Option<&u64> {
        self.max_iops.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u16) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u16> {
        self.media_type.as_ref()
    }

    /// Sets the value of MinimumLogicalDataCopies
    pub fn set_minimum_logical_data_copies(&mut self, value: u16) {
        self.minimum_logical_data_copies = Some(value);
    }

    /// Gets the value of MinimumLogicalDataCopies
    pub fn get_minimum_logical_data_copies(&self) -> Option<&u16> {
        self.minimum_logical_data_copies.as_ref()
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

    /// Sets the value of NumberOfAvailableCopies
    pub fn set_number_of_available_copies(&mut self, value: u16) {
        self.number_of_available_copies = Some(value);
    }

    /// Gets the value of NumberOfAvailableCopies
    pub fn get_number_of_available_copies(&self) -> Option<&u16> {
        self.number_of_available_copies.as_ref()
    }

    /// Sets the value of NumberOfColumns
    pub fn set_number_of_columns(&mut self, value: u16) {
        self.number_of_columns = Some(value);
    }

    /// Gets the value of NumberOfColumns
    pub fn get_number_of_columns(&self) -> Option<&u16> {
        self.number_of_columns.as_ref()
    }

    /// Sets the value of NumberOfDataCopies
    pub fn set_number_of_data_copies(&mut self, value: u16) {
        self.number_of_data_copies = Some(value);
    }

    /// Gets the value of NumberOfDataCopies
    pub fn get_number_of_data_copies(&self) -> Option<&u16> {
        self.number_of_data_copies.as_ref()
    }

    /// Sets the value of NumberOfGroups
    pub fn set_number_of_groups(&mut self, value: u16) {
        self.number_of_groups = Some(value);
    }

    /// Gets the value of NumberOfGroups
    pub fn get_number_of_groups(&self) -> Option<&u16> {
        self.number_of_groups.as_ref()
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

    /// Sets the value of ParityLayout
    pub fn set_parity_layout(&mut self, value: u16) {
        self.parity_layout = Some(value);
    }

    /// Gets the value of ParityLayout
    pub fn get_parity_layout(&self) -> Option<&u16> {
        self.parity_layout.as_ref()
    }

    /// Sets the value of PhysicalDiskRedundancy
    pub fn set_physical_disk_redundancy(&mut self, value: u16) {
        self.physical_disk_redundancy = Some(value);
    }

    /// Gets the value of PhysicalDiskRedundancy
    pub fn get_physical_disk_redundancy(&self) -> Option<&u16> {
        self.physical_disk_redundancy.as_ref()
    }

    /// Sets the value of PhysicalSectorSize
    pub fn set_physical_sector_size(&mut self, value: u64) {
        self.physical_sector_size = Some(value);
    }

    /// Gets the value of PhysicalSectorSize
    pub fn get_physical_sector_size(&self) -> Option<&u64> {
        self.physical_sector_size.as_ref()
    }

    /// Sets the value of ProvisioningType
    pub fn set_provisioning_type(&mut self, value: u16) {
        self.provisioning_type = Some(value);
    }

    /// Gets the value of ProvisioningType
    pub fn get_provisioning_type(&self) -> Option<&u16> {
        self.provisioning_type.as_ref()
    }

    /// Sets the value of ReadCacheSize
    pub fn set_read_cache_size(&mut self, value: u64) {
        self.read_cache_size = Some(value);
    }

    /// Gets the value of ReadCacheSize
    pub fn get_read_cache_size(&self) -> Option<&u64> {
        self.read_cache_size.as_ref()
    }

    /// Sets the value of RequestNoSinglePointOfFailure
    pub fn set_request_no_single_point_of_failure(&mut self, value: bool) {
        self.request_no_single_point_of_failure = Some(value);
    }

    /// Gets the value of RequestNoSinglePointOfFailure
    pub fn get_request_no_single_point_of_failure(&self) -> Option<&bool> {
        self.request_no_single_point_of_failure.as_ref()
    }

    /// Sets the value of ResiliencySettingName
    pub fn set_resiliency_setting_name(&mut self, value: String) {
        self.resiliency_setting_name = Some(value);
    }

    /// Gets the value of ResiliencySettingName
    pub fn get_resiliency_setting_name(&self) -> Option<&String> {
        self.resiliency_setting_name.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of TrackValidData
    pub fn set_track_valid_data(&mut self, value: bool) {
        self.track_valid_data = Some(value);
    }

    /// Gets the value of TrackValidData
    pub fn get_track_valid_data(&self) -> Option<&bool> {
        self.track_valid_data.as_ref()
    }

    /// Sets the value of UniqueIdFormat
    pub fn set_unique_id_format(&mut self, value: u16) {
        self.unique_id_format = Some(value);
    }

    /// Gets the value of UniqueIdFormat
    pub fn get_unique_id_format(&self) -> Option<&u16> {
        self.unique_id_format.as_ref()
    }

    /// Sets the value of UniqueIdFormatDescription
    pub fn set_unique_id_format_description(&mut self, value: String) {
        self.unique_id_format_description = Some(value);
    }

    /// Gets the value of UniqueIdFormatDescription
    pub fn get_unique_id_format_description(&self) -> Option<&String> {
        self.unique_id_format_description.as_ref()
    }

    /// Sets the value of Usage
    pub fn set_usage(&mut self, value: u16) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&u16> {
        self.usage.as_ref()
    }

    /// Sets the value of WriteCacheReserveSize
    pub fn set_write_cache_reserve_size(&mut self, value: u64) {
        self.write_cache_reserve_size = Some(value);
    }

    /// Gets the value of WriteCacheReserveSize
    pub fn get_write_cache_reserve_size(&self) -> Option<&u64> {
        self.write_cache_reserve_size.as_ref()
    }

    /// Sets the value of WriteCacheSize
    pub fn set_write_cache_size(&mut self, value: u64) {
        self.write_cache_size = Some(value);
    }

    /// Gets the value of WriteCacheSize
    pub fn get_write_cache_size(&self) -> Option<&u64> {
        self.write_cache_size.as_ref()
    }

/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `physical_extents` -  (MSFT_PhysicalExtent[])
    /// * `return_value` -  (u32)
    pub fn get_physical_extent(&self, physical_extents: &mut Vec<MSFT_PhysicalExtent>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetPhysicalExtent", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let physical_extents = result.get_value("PhysicalExtents")?;
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

    /// * `host_type` -  (u16)
    /// * `initiator_address` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `target_port_addresses` -  (String[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn show(&self, target_port_addresses: &Vec<String>, initiator_address: &String, host_type: u16, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "InitiatorAddress".to_string(), value: initiator_address.into() });
        args.push(MethodParameter { name: "HostType".to_string(), value: host_type.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Show", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `initiator_address` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `target_port_addresses` -  (String[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn hide(&self, target_port_addresses: &Vec<String>, initiator_address: &String, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "InitiatorAddress".to_string(), value: initiator_address.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Hide", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `target_storage_pool_name` -  (String)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_snapshot(&self, friendly_name: &String, target_storage_pool_name: &String, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, created_virtual_disk: &mut MSFT_VirtualDisk, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStoragePoolName".to_string(), value: target_storage_pool_name.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateSnapshot", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `run_as_job` -  (bool)
    /// * `target_storage_pool_name` -  (String)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_clone(&self, friendly_name: &String, target_storage_pool_name: &String, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, created_virtual_disk: &mut MSFT_VirtualDisk, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStoragePoolName".to_string(), value: target_storage_pool_name.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateClone", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `size` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `size` -  (u64)
    pub fn resize(&self, size: &mut u64, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Resize", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let size = result.get_value("Size")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn repair(&self, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Repair", &args)?;
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

    /// * `access` -  (u16)
    /// * `is_manual_attach` -  (bool)
    /// * `storage_node_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, is_manual_attach: bool, storage_node_name: &String, access: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsManualAttach".to_string(), value: is_manual_attach.into() });
        args.push(MethodParameter { name: "StorageNodeName".to_string(), value: storage_node_name.into() });
        args.push(MethodParameter { name: "Access".to_string(), value: access.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `allocation_unit_size` -  (u64)
    /// * `column_isolation` -  (u16)
    /// * `fault_domain_awareness` -  (u16)
    /// * `interleave` -  (u64)
    /// * `max_io_bandwidth` -  (u64)
    /// * `max_iops` -  (u64)
    /// * `media_type` -  (u16)
    /// * `number_of_columns` -  (u16)
    /// * `number_of_data_copies` -  (u16)
    /// * `number_of_groups` -  (u16)
    /// * `physical_disk_redundancy` -  (u16)
    /// * `provisioning_type` -  (u16)
    /// * `resiliency_setting_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_properties(&self, provisioning_type: u16, allocation_unit_size: u64, media_type: u16, fault_domain_awareness: u16, column_isolation: u16, resiliency_setting_name: &String, physical_disk_redundancy: u16, number_of_data_copies: u16, number_of_groups: u16, number_of_columns: u16, interleave: u64, max_iops: u64, max_io_bandwidth: u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ProvisioningType".to_string(), value: provisioning_type.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "FaultDomainAwareness".to_string(), value: fault_domain_awareness.into() });
        args.push(MethodParameter { name: "ColumnIsolation".to_string(), value: column_isolation.into() });
        args.push(MethodParameter { name: "ResiliencySettingName".to_string(), value: resiliency_setting_name.into() });
        args.push(MethodParameter { name: "PhysicalDiskRedundancy".to_string(), value: physical_disk_redundancy.into() });
        args.push(MethodParameter { name: "NumberOfDataCopies".to_string(), value: number_of_data_copies.into() });
        args.push(MethodParameter { name: "NumberOfGroups".to_string(), value: number_of_groups.into() });
        args.push(MethodParameter { name: "NumberOfColumns".to_string(), value: number_of_columns.into() });
        args.push(MethodParameter { name: "Interleave".to_string(), value: interleave.into() });
        args.push(MethodParameter { name: "MaxIops".to_string(), value: max_iops.into() });
        args.push(MethodParameter { name: "MaxIoBandwidth".to_string(), value: max_io_bandwidth.into() });

        let result = self.invoke_method("SetProperties", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `storage_node_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn attach(&self, storage_node_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageNodeName".to_string(), value: storage_node_name.into() });

        let result = self.invoke_method("Attach", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `storage_node_name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn detach(&self, storage_node_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageNodeName".to_string(), value: storage_node_name.into() });

        let result = self.invoke_method("Detach", &args)?;
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

    /// * `run_as_job` -  (bool)
    /// * `storage_fault_domains` -  (MSFT_StorageFaultDomain[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_storage_fault_domain(&self, storage_fault_domains: &Vec<MSFT_StorageFaultDomain>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageFaultDomains".to_string(), value: storage_fault_domains.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("AddStorageFaultDomain", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `storage_fault_domains` -  (MSFT_StorageFaultDomain[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_storage_fault_domain(&self, storage_fault_domains: &Vec<MSFT_StorageFaultDomain>, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageFaultDomains".to_string(), value: storage_fault_domains.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("RemoveStorageFaultDomain", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `recovery_point_objective` -  (u16)
    /// * `replication_settings` -  (MSFT_ReplicationSettings)
    /// * `run_as_job` -  (bool)
    /// * `sync_type` -  (u16)
    /// * `target_storage_pool_object_id` -  (String)
    /// * `target_storage_subsystem` -  (MSFT_ReplicaPeer)
    /// * `target_virtual_disk_object_id` -  (String)

    /// * `created_replica_peer` -  (MSFT_ReplicaPeer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_replica(&self, friendly_name: &String, target_storage_subsystem: MSFT_ReplicaPeer, target_virtual_disk_object_id: &String, target_storage_pool_object_id: &String, recovery_point_objective: u16, replication_settings: MSFT_ReplicationSettings, sync_type: u16, run_as_job: bool, created_replica_peer: &mut MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStorageSubsystem".to_string(), value: target_storage_subsystem.into() });
        args.push(MethodParameter { name: "TargetVirtualDiskObjectId".to_string(), value: target_virtual_disk_object_id.into() });
        args.push(MethodParameter { name: "TargetStoragePoolObjectId".to_string(), value: target_storage_pool_object_id.into() });
        args.push(MethodParameter { name: "RecoveryPointObjective".to_string(), value: recovery_point_objective.into() });
        args.push(MethodParameter { name: "ReplicationSettings".to_string(), value: replication_settings.into() });
        args.push(MethodParameter { name: "SyncType".to_string(), value: sync_type.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("CreateReplica", &args)?;
        let created_replica_peer = result.get_value("CreatedReplicaPeer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `operation` -  (u16)
    /// * `run_as_job` -  (bool)
    /// * `virtual_disk_replica_peer` -  (MSFT_ReplicaPeer)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_replication_relationship(&self, operation: u16, virtual_disk_replica_peer: MSFT_ReplicaPeer, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Operation".to_string(), value: operation.into() });
        args.push(MethodParameter { name: "VirtualDiskReplicaPeer".to_string(), value: virtual_disk_replica_peer.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("SetReplicationRelationship", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

