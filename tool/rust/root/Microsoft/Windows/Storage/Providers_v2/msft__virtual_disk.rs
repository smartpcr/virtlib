// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_VirtualDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_VirtualDisk {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// Indicates whether the virtual disk is available for read and/or write access
    #[serde(rename = "Access")]
    pub access: Option<VirtualDisk_Access>,

/// The currently allocated size of the virtual disk. If the virtual disk's ProvisioningType is 2 - 'Fixed', this value should equal Size. If the ProvisioningType is 1 - 'Thin', this value is the amount of space actually allocated (i.e. some value less than Size).
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// Specifies the allocation unit size in bytes for this virtual disk.
    #[serde(rename = "AllocationUnitSize")]
    pub allocation_unit_size: Option<u64>,

/// 
    #[serde(rename = "ColumnIsolation")]
    pub column_isolation: Option<u16>,

/// Denotes the reason why this virtual disk is detached. This field will only be set when the virtual disk's OperationalStatus includes 0xD002 - 'Detached'. Note that this field is specific to Storage Spaces.
    #[serde(rename = "DetachedReason")]
    pub detached_reason: Option<VirtualDisk_DetachedReason>,

/// Determines the current allocation behavior for this virtual disk. Fault domain aware virtual disks will intelligently pick the physical disks to use for their redundancy to balance the fault tolerance between two (or more) fault domain units of the specified type.
    #[serde(rename = "FaultDomainAwareness")]
    pub fault_domain_awareness: Option<VirtualDisk_FaultDomainAwareness>,

/// This field indicates the total storage pool capacity being consumed by this virtual disk. For example: in the case of a 2-way mirrored virtual disk of size 1 GB, the footprint on the pool will be approximately 2 GB.
    #[serde(rename = "FootprintOnPool")]
    pub footprint_on_pool: Option<u64>,

/// A user-settable, display-oriented string representing the name of the virtual disk.
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// Denotes the current health status of the virtual disk. Health of a virtual disk is derived from the health of the backing physical disks, and whether or not the virtual disk can maintain the required levels of resiliency.
///  0 - 'Healthy': All physical disks are present and in a healthy state. 
/// 1 - 'Warning': The majority of physical disks are healthy, but one or more may be failing I/O requests. 
/// 2 - 'Unhealthy': The majority of physical disks are unhealthy or in a failed state, and the virtual disk no longer has data integrity.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<VirtualDisk_HealthStatus>,

/// This field indicates the number of bytes that will form a strip in common striping-based resiliency settings. The strip is defined as the size of the portion of a stripe that lies on one physical disk. Thus, Interleave * NumberOfColumns will yield the size of one stripe of user data.
    #[serde(rename = "Interleave")]
    pub interleave: Option<u64>,

/// 
    #[serde(rename = "IsDeduplicationEnabled")]
    pub is_deduplication_enabled: Option<bool>,

/// Determines the current allocation behavior for this virtual disk. Enclosure aware virtual disks will intelligently pick the physical disks to use for their redundancy. If TRUE, the virtual disk will attempt to use physical disks from different enclosures to balance the fault tolerance between two (or more) physical enclosures.
    #[serde(rename = "IsEnclosureAware")]
    pub is_enclosure_aware: Option<bool>,

/// If TRUE, this virtual disk will only be attached to the system if an explicit call is made to the Attach method. Note that this property is specific to Storage Spaces.
    #[serde(rename = "IsManualAttach")]
    pub is_manual_attach: Option<bool>,

/// Indicates whether this virtual disk is a snapshot of another virtual disk
    #[serde(rename = "IsSnapshot")]
    pub is_snapshot: Option<bool>,

/// Indicates whether or not there are tiers associated with this virtual disk.
    #[serde(rename = "IsTiered")]
    pub is_tiered: Option<bool>,

/// 
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u64>,

/// This parameter indicates the maximum IO bandwidth supported by the virtual disk.
    #[serde(rename = "MaxIoBandwidth")]
    pub max_io_bandwidth: Option<u64>,

/// This parameter indicates the maximum IOPS supported by the virtual disk.
    #[serde(rename = "MaxIops")]
    pub max_iops: Option<u64>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u16>,

/// This field indicates the minimum number of logical data copies to enforce.
    #[serde(rename = "MinimumLogicalDataCopies")]
    pub minimum_logical_data_copies: Option<u16>,

/// Name is a semi-unique (scoped to the owning storage subsystem), human-readable string used to identify the virtual disk.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NameFormat")]
    pub name_format: Option<VirtualDisk_NameFormat>,

/// 
    #[serde(rename = "NumberOfAvailableCopies")]
    pub number_of_available_copies: Option<u16>,

/// This field indicates the number of underlying physical disks across which data for this virtual disk is striped.
    #[serde(rename = "NumberOfColumns")]
    pub number_of_columns: Option<u16>,

/// This field indicates the number of complete data copies that are being maintained. For example, RAID 5 maintains 1 copy of data, whereas RAID 1 maintains at least 2 copies.
    #[serde(rename = "NumberOfDataCopies")]
    pub number_of_data_copies: Option<u16>,

/// 
    #[serde(rename = "NumberOfGroups")]
    pub number_of_groups: Option<u16>,

/// Indicates the current operating conditions of the virtual disk. Unlike HealthStatus, this field indicates the status of hardware, software, and infrastructure issues related to this virtual disk, and can contain multiple values. Various operational statuses are defined. 
/// 11 - 'In Service': describes a virtual disk being configured, maintained, or otherwise administered. 
/// 0xD002 - 'Detached': This value is reserved for Windows. This value indicates a virtual disk that is visible to the host system but does not have a disk device object. 
/// 0xD003 - 'Incomplete': describes a virtual disk which does not have enough redundancy remaining to successfully repair or regenerate its data.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<VirtualDisk_OperationalStatus>,

/// If OperationalStatus contains 1 - 'Other', this field contains the string representing the vendor defined operational status. This property must be NULL if OperationalStatus does not contain 1 - 'Other'.
    #[serde(rename = "OtherOperationalStatusDescription")]
    pub other_operational_status_description: Option<String>,

/// If the virtual disk's Usage field is set to 1 - 'Other', this field must contain a description of the vendor or user defined usage. If Usage is not set to 1 - 'Other', this field must not be set.
    #[serde(rename = "OtherUsageDescription")]
    pub other_usage_description: Option<String>,

/// This field indicates what type of parity layout is being used for parity resiliency settings. This field should be NULL if the virtual disk does not use a parity resiliency setting.
    #[serde(rename = "ParityLayout")]
    pub parity_layout: Option<VirtualDisk_ParityLayout>,

/// This field indicates how many backing physical disks can fail without compromising data redundancy. For example: RAID 0 cannot tolerate any failures, RAID 5 can tolerate a single drive failure, and RAID 6 can tolerate 2 failures.
    #[serde(rename = "PhysicalDiskRedundancy")]
    pub physical_disk_redundancy: Option<u16>,

/// 
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u64>,

/// Denotes the provisioning scheme of the virtual disk. 
/// 1 - 'Thin' indicates that the virtual disk's capacity is allocated on demand. 
/// 2 - 'Fixed' indicates that the virtual disk's capacity is fully allocated upon creation. 
    #[serde(rename = "ProvisioningType")]
    pub provisioning_type: Option<VirtualDisk_ProvisioningType>,

/// Size of the read cache for the virtual disk
    #[serde(rename = "ReadCacheSize")]
    pub read_cache_size: Option<u64>,

/// 
    #[serde(rename = "RequestNoSinglePointOfFailure")]
    pub request_no_single_point_of_failure: Option<bool>,

/// The name of the resiliency setting used to create this virtual disk.
    #[serde(rename = "ResiliencySettingName")]
    pub resiliency_setting_name: Option<String>,

/// The logical size of the virtual disk measured in bytes
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// Indicates whether valid data tracking is enforced for this virtual disk.
    #[serde(rename = "TrackValidData")]
    pub track_valid_data: Option<bool>,

/// UniqueIdFormat indicates the type of identifier used in the UniqueId field. The identifier used in UniqueId must be the highest available identifier using the following order of preference: 8 (highest), 3, 2, 1, 0 (lowest). For example: if the virtual disk device exposes identifiers of type 0, 1, and 3, UniqueId must be the identifier of type 3, and UniqueIdFormat should be set to 3.
    #[serde(rename = "UniqueIdFormat")]
    pub unique_id_format: Option<VirtualDisk_UniqueIdFormat>,

/// Certain values for UniqueIdFormat may include various sub-formats. This field is a free-form string used to describe the specific format used in UniqueId.
    #[serde(rename = "UniqueIdFormatDescription")]
    pub unique_id_format_description: Option<String>,

/// This field indicates the intended usage for this virtual disk.
    #[serde(rename = "Usage")]
    pub usage: Option<VirtualDisk_Usage>,

/// Size of the write cache reserve region.
    #[serde(rename = "WriteCacheReserveSize")]
    pub write_cache_reserve_size: Option<u64>,

/// Size of the write cache for the virtual disk
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
    pub fn set_access(&mut self, value: VirtualDisk_Access) {
        self.access = Some(value);
    }

    /// Gets the value of Access
    pub fn get_access(&self) -> Option<&VirtualDisk_Access> {
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
    pub fn set_detached_reason(&mut self, value: VirtualDisk_DetachedReason) {
        self.detached_reason = Some(value);
    }

    /// Gets the value of DetachedReason
    pub fn get_detached_reason(&self) -> Option<&VirtualDisk_DetachedReason> {
        self.detached_reason.as_ref()
    }

    /// Sets the value of FaultDomainAwareness
    pub fn set_fault_domain_awareness(&mut self, value: VirtualDisk_FaultDomainAwareness) {
        self.fault_domain_awareness = Some(value);
    }

    /// Gets the value of FaultDomainAwareness
    pub fn get_fault_domain_awareness(&self) -> Option<&VirtualDisk_FaultDomainAwareness> {
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
    pub fn set_health_status(&mut self, value: VirtualDisk_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&VirtualDisk_HealthStatus> {
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
    pub fn set_name_format(&mut self, value: VirtualDisk_NameFormat) {
        self.name_format = Some(value);
    }

    /// Gets the value of NameFormat
    pub fn get_name_format(&self) -> Option<&VirtualDisk_NameFormat> {
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
    pub fn set_operational_status(&mut self, value: Vec<VirtualDisk_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<VirtualDisk_OperationalStatus> {
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
    pub fn set_parity_layout(&mut self, value: VirtualDisk_ParityLayout) {
        self.parity_layout = Some(value);
    }

    /// Gets the value of ParityLayout
    pub fn get_parity_layout(&self) -> Option<&VirtualDisk_ParityLayout> {
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
    pub fn set_provisioning_type(&mut self, value: VirtualDisk_ProvisioningType) {
        self.provisioning_type = Some(value);
    }

    /// Gets the value of ProvisioningType
    pub fn get_provisioning_type(&self) -> Option<&VirtualDisk_ProvisioningType> {
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
    pub fn set_unique_id_format(&mut self, value: VirtualDisk_UniqueIdFormat) {
        self.unique_id_format = Some(value);
    }

    /// Gets the value of UniqueIdFormat
    pub fn get_unique_id_format(&self) -> Option<&VirtualDisk_UniqueIdFormat> {
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
    pub fn set_usage(&mut self, value: VirtualDisk_Usage) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&VirtualDisk_Usage> {
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


/// This method deletes the virtual disk. After this method is called, the space used by the virtual disk will be reclaimed and the user will be unable to reverse the delete operation.

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method shows a virtual disk to an initiator. This operation is also known as 'exposing' or 'unmasking' a virtual disk.

    /// * `host_type` - This field indicates the operating system type running on the host of the initiator port. (VirtualDisk_HostType)
    /// * `initiator_address` - The address of the initiator to which the virtual disk should be shown (String)
    /// * `target_port_addresses` - An array of target port addresses from which the virtual disk should be shown (String[])

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn show(&self, target_port_addresses: &Vec<String>, initiator_address: &String, host_type: VirtualDisk_HostType, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "InitiatorAddress".to_string(), value: initiator_address.into() });
        args.push(MethodParameter { name: "HostType".to_string(), value: host_type.into() });

        let result = self.invoke_method("Show", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method hides a virtual disk from an initiator. This operation is also known as 'unexposing' or 'masking' a virtual disk.

    /// * `initiator_address` - The address of the initiator to which the virtual disk should be hidden (String)
    /// * `target_port_addresses` - An array of target port addresses from which the virtual disk should be hidden. Note: this array may contain a subset of the addresses originally given in Show. (String[])

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn hide(&self, target_port_addresses: &Vec<String>, initiator_address: &String, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetPortAddresses".to_string(), value: target_port_addresses.into() });
        args.push(MethodParameter { name: "InitiatorAddress".to_string(), value: initiator_address.into() });

        let result = self.invoke_method("Hide", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method creates a point in time snapshot of the virtual disk.

    /// * `friendly_name` - The desired name of the snapshot virtual disk (String)
    /// * `target_storage_pool_name` - This field indicates which storage pool should be used to hold the created snapshot. If this field is not set, this method will default to using the same storage pool that contains the source virtual disk. (String)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_snapshot(&self, friendly_name: &String, target_storage_pool_name: &String, created_storage_job: &mut MSFT_StorageJob, created_virtual_disk: &mut MSFT_VirtualDisk, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStoragePoolName".to_string(), value: target_storage_pool_name.into() });

        let result = self.invoke_method("CreateSnapshot", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method creates a clone of the virtual disk, resulting in another virtual disk with identical data to the source.

    /// * `friendly_name` - The desired name of the virtual disk clone (String)
    /// * `target_storage_pool_name` - This field indicates which storage pool should be used to hold the created clone. If this field is not set, this method will default to using the same storage pool that contains the source virtual disk. (String)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `created_virtual_disk` -  (MSFT_VirtualDisk)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_clone(&self, friendly_name: &String, target_storage_pool_name: &String, created_storage_job: &mut MSFT_StorageJob, created_virtual_disk: &mut MSFT_VirtualDisk, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStoragePoolName".to_string(), value: target_storage_pool_name.into() });

        let result = self.invoke_method("CreateClone", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_virtual_disk = result.get_value("CreatedVirtualDisk")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows a virtual disk to be resized. The size specified must be in the range of valid values given by the GetSupportedSize method on the storage pool object.

    /// * `size` - As input, this parameter contains the requested size for the virtual disk to become. As output, this parameter contains the size that was actually achieved after the resize operation. (u64)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `size` - As input, this parameter contains the requested size for the virtual disk to become. As output, this parameter contains the size that was actually achieved after the resize operation. (u64)
    pub fn resize(&self, size: &mut u64, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();

        let result = self.invoke_method("Resize", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let size = result.get_value("Size")?;
        Ok(result.return_value)

    }


/// This method initiates a repair of the virtual disk - restoring data and redundancy to different (or new) physical disks within the storage pool.

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn repair(&self, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Repair", &[])?;
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


/// This method allows the virtual disk to be renamed.

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


/// This method allows the virtual disk's intended usage to be updated. Not all virtual disks may allow this and will return 1 - 'Not Supported' if this operation cannot be performed.

    /// * `other_usage_description` - If Usage is set to 1 - 'Other', this parameter takes in the string representation of a vendor defined usage for this virtual disk. This parameter must not be set if Usage is a value other than 1 - 'Other'. (String)
    /// * `usage` -  (VirtualDisk_Usage)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_usage(&self, usage: VirtualDisk_Usage, other_usage_description: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });
        args.push(MethodParameter { name: "OtherUsageDescription".to_string(), value: other_usage_description.into() });

        let result = self.invoke_method("SetUsage", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the user to update or set various attributes on the virtual disk. Note that not all parameters must be specified, and only those given will be updated.

    /// * `access` -  (VirtualDisk_Access)
    /// * `is_manual_attach` -  (bool)
    /// * `storage_node_name` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, is_manual_attach: bool, storage_node_name: &String, access: VirtualDisk_Access, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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


/// Attaches a Storage Spaces based virtual disk to the system. This operation is similar to Show and Hide, however there is no need for target and initiator configuration since everything is done locally. Depending on the system's NewDiskPolicy (formerly SAN policy), a Storage Space may need to be Attached before it can be used.

    /// * `storage_node_name` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn attach(&self, storage_node_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageNodeName".to_string(), value: storage_node_name.into() });

        let result = self.invoke_method("Attach", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// Detaches a Storage Spaces based virtual disk from the system. This operation is similar to Hide, however there is no need for target and initiator configuration since everything is done locally. Detaching a Storage Space will result in it's corresponding disk object to be suprise removed from the system. Note that detaching can happen in response to certain failure and warning conditions (such as failing redundancy, or thin provisioning capacity limits being reached).

    /// * `storage_node_name` -  (String)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn detach(&self, storage_node_name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageNodeName".to_string(), value: storage_node_name.into() });

        let result = self.invoke_method("Detach", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method will add one or more physical disks for manual allocation.

    /// * `physical_disks` -  (MSFT_PhysicalDisk[])
    /// * `usage` -  (VirtualDisk_Usage)

    /// * `created_storage_job` - This parameter returns a reference to the storage job used to track the long running operation. (MSFT_StorageJob)
    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_physical_disk(&self, physical_disks: &Vec<MSFT_PhysicalDisk>, usage: VirtualDisk_Usage, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
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


/// This method will remove one or more physical disks from manual allocation.

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


/// 

    /// * `storage_fault_domains` -  (MSFT_StorageFaultDomain[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_storage_fault_domain(&self, storage_fault_domains: &Vec<MSFT_StorageFaultDomain>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageFaultDomains".to_string(), value: storage_fault_domains.into() });

        let result = self.invoke_method("AddStorageFaultDomain", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `storage_fault_domains` -  (MSFT_StorageFaultDomain[])

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_storage_fault_domain(&self, storage_fault_domains: &Vec<MSFT_StorageFaultDomain>, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StorageFaultDomains".to_string(), value: storage_fault_domains.into() });

        let result = self.invoke_method("RemoveStorageFaultDomain", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `friendly_name` -  (String)
    /// * `recovery_point_objective` -  (u16)
    /// * `replication_settings` -  (MSFT_ReplicationSettings)
    /// * `sync_type` -  (u16)
    /// * `target_storage_pool_object_id` -  (String)
    /// * `target_storage_subsystem` -  (MSFT_ReplicaPeer)
    /// * `target_virtual_disk_object_id` -  (String)

    /// * `created_replica_peer` -  (MSFT_ReplicaPeer)
    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_replica(&self, friendly_name: &String, target_storage_subsystem: MSFT_ReplicaPeer, target_virtual_disk_object_id: &String, target_storage_pool_object_id: &String, recovery_point_objective: u16, replication_settings: MSFT_ReplicationSettings, sync_type: u16, created_replica_peer: &mut MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "TargetStorageSubsystem".to_string(), value: target_storage_subsystem.into() });
        args.push(MethodParameter { name: "TargetVirtualDiskObjectId".to_string(), value: target_virtual_disk_object_id.into() });
        args.push(MethodParameter { name: "TargetStoragePoolObjectId".to_string(), value: target_storage_pool_object_id.into() });
        args.push(MethodParameter { name: "RecoveryPointObjective".to_string(), value: recovery_point_objective.into() });
        args.push(MethodParameter { name: "ReplicationSettings".to_string(), value: replication_settings.into() });
        args.push(MethodParameter { name: "SyncType".to_string(), value: sync_type.into() });

        let result = self.invoke_method("CreateReplica", &args)?;
        let created_replica_peer = result.get_value("CreatedReplicaPeer")?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `operation` -  (u16)
    /// * `virtual_disk_replica_peer` -  (MSFT_ReplicaPeer)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_replication_relationship(&self, operation: u16, virtual_disk_replica_peer: MSFT_ReplicaPeer, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Operation".to_string(), value: operation.into() });
        args.push(MethodParameter { name: "VirtualDiskReplicaPeer".to_string(), value: virtual_disk_replica_peer.into() });

        let result = self.invoke_method("SetReplicationRelationship", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

