// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Disk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Disk {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// A string representation of the Adapter's serial number.
    #[serde(rename = "AdapterSerialNumber")]
    pub adapter_serial_number: Option<String>,

/// The amount of space currently used on the disk.
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// This property indicates that the computer is configured to start off of this disk. On computers with BIOS firmware, this is the first disk that the firmware detects during startup. On computers that use EFI firmware, this is the disk that contains the EFI System Partition (ESP). If there are no disks or multiple disks with an ESP partition, this flag is not set for any disk.
    #[serde(rename = "BootFromDisk")]
    pub boot_from_disk: Option<bool>,

/// Denotes the I/O bus type used by this disk.
    #[serde(rename = "BusType")]
    pub bus_type: Option<Disk_BusType>,

/// A string representation of the disk's firmware version.
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// FriendlyName is a user-friendly, display-oriented string to identify the disk.
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// The GPT guid of the disk. This property is only valid on GPT disks and will be NULL for all other disk types.
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// The health status of the Volume.
/// 0 - 'Healthy': The disk is functioning normally.
/// 1 - 'Warning': The disk is still functioning, but has detected errors or issues that require administrator intervention.
/// 2 - 'Unhealthy': The volume is not functioning, due to errors or failures. The volume needs immediate attention from an administrator.
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<Disk_HealthStatus>,

/// This property indicates that the computer has booted off of this disk.
    #[serde(rename = "IsBoot")]
    pub is_boot: Option<bool>,

/// If IsClustered is TRUE, this disk is used in a clustered environment.
    #[serde(rename = "IsClustered")]
    pub is_clustered: Option<bool>,

/// If IsHighlyAvailable is TRUE, the disk is highly available.
    #[serde(rename = "IsHighlyAvailable")]
    pub is_highly_available: Option<bool>,

/// 
    #[serde(rename = "IsOffline")]
    pub is_offline: Option<bool>,

/// 
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// If IsScaleOut is TRUE, the disk is scaled out.
    #[serde(rename = "IsScaleOut")]
    pub is_scale_out: Option<bool>,

/// If IsSystem is TRUE, this disk contains the system partition.
    #[serde(rename = "IsSystem")]
    pub is_system: Option<bool>,

/// This field indicates the largest contiguous block of free space on the disk. This is also the largest size of a partition which can be created on the disk.
    #[serde(rename = "LargestFreeExtent")]
    pub largest_free_extent: Option<u64>,

/// Location contains the PnP location path of the disk. The format of this string depends on the bus type. If the bus type is SCSI, SAS, or PCI RAID, the format is <AdapterPnpLocationPath>#<BusType>(P<PathId>T<TargetId>L<LunId>). If the bus type is IDE, ATA, PATA, or SATA, the format is <AdapterPnpLocationPath>#<BusType>(C<PathId>T<TargetId>L<LunId>). For example, a SCSI location may look like: PCIROOT(0)#PCI(1C00)#PCI(0000)#SCSI(P00T01L01). Note: For Hyper-V and VHD images, this member is NULL because the virtual controller does not return the location path.
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// This field indicates the logical sector size of the disk in bytes. For example: a 4K native disk will report 4096, while a 512 emulated disk will report 512.
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u32>,

/// A string representation of the disk's hardware manufacturer.
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// A string representation of the disk's model.
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// The operating system's number for the disk. Disk 0 is typically the boot device. Disk numbers may not necessarily remain the same across reboots.
    #[serde(rename = "Number")]
    pub number: Option<u32>,

/// 
    #[serde(rename = "NumberOfPartitions")]
    pub number_of_partitions: Option<u32>,

/// If IsOffline is TRUE, this property informs the user of the specific reason for the disk being offline. 
/// 1 - 'Policy': The user requested the disk to be offline. 
/// 2 - 'Redundant Path': The disk is used for multi-path I/O. 
/// 3 - 'Snapshot': The disk is a snapshot disk. 
/// 4 - 'Collision': There was a signature or identifier collision with another disk. 
/// 5 - 'Resource Exhaustion': There were insufficient resources to bring the disk online. 
/// 6 - 'Critical Write Failures': There were critical write failures on the disk. 
/// 7 - 'Data Integrity Scan Required': A data integrity scan is required.
    #[serde(rename = "OfflineReason")]
    pub offline_reason: Option<Disk_OfflineReason>,

/// An array of values that denote the current operational status of the volume.
/// 0 - 'Unknown': The operational status is unknown.
/// 1 - 'Other': A vendor-specific OperationalStatus has been specified by setting the OtherOperationalStatusDescription property.
/// 2 - 'OK': The disk is responding to commands and is in a normal operating state.
/// 3 - 'Degraded': The disk is responding to commands, but is not running in an optimal operating state.
/// 4 - 'Stressed': The disk is functioning, but needs attention. For example, the disk might be overloaded or overheated.
/// 5 - 'Predictive Failure': The disk is functioning, but a failure is likely to occur in the near future.
/// 6 - 'Error': An error has occurred.
/// 7 - 'Non-Recoverable Error': A non-recoverable error has occurred.
/// 8 - 'Starting': The disk is in the process of starting.
/// 9 - 'Stopping': The disk is in the process of stopping.
/// 10 - 'Stopped': The disk was stopped or shut down in a clean and orderly fashion.
/// 11 - 'In Service': The disk is being configured, maintained, cleaned, or otherwise administered.
/// 12 - 'No Contact': The storage provider has knowledge of the disk, but has never been able to establish communication with it.
/// 13 - 'Lost Communication': The storage provider has knowledge of the disk and has contacted it successfully in the past, but the disk is currently unreachable.
/// 14 - 'Aborted': Similar to Stopped, except that the disk stopped abruptly and may require configuration or maintenance.
/// 15 - 'Dormant': The disk is reachable, but it is inactive.
/// 16 - 'Supporting Entity in Error': This status value does not necessarily indicate trouble with the disk, but it does indicate that another device or connection that the disk depends on may need attention.
/// 17 - 'Completed': The disk has completed an operation. This status value should be combined with OK, Error, or Degraded, depending on the outcome of the operation.
/// 0xD010 - 'Online': In Windows-based storage subsystems, this indicates that the object is online.
/// 0xD011 - 'Not Ready': In Windows-based storage subsystems, this indicates that the object is not ready.
/// 0xD012 - 'No Media': In Windows-based storage subsystems, this indicates that the object has no media present.
/// 0xD013 - 'Offline': In Windows-based storage subsystems, this indicates that the object is offline.
/// 0xD014 - 'Failed': In Windows-based storage subsystems, this indicates that the object is in a failed state.
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<Disk_OperationalStatus>,

/// 
    #[serde(rename = "PartitionStyle")]
    pub partition_style: Option<Disk_PartitionStyle>,

/// Path can be used to open an operating system handle to the disk device.
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// This field indicates the physical sector size of the disk in bytes. For example: both 4K native disks and 512 emulated disks will report 4096.
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u32>,

/// Denotes the provisioning type of the disk device. 
/// 1 - 'Thin' means that the storage for the disk is allocated on-demand. 
/// 2 - 'Fixed' means that the storage is allocated up front.
    #[serde(rename = "ProvisioningType")]
    pub provisioning_type: Option<Disk_ProvisioningType>,

/// A string representation of the disk's serial number.
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// The MBR signature of the disk. This property is only valid on MBR disks and will be NULL for all other disk types.
    #[serde(rename = "Signature")]
    pub signature: Option<u32>,

/// The total size of the disk, measured in bytes.
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// UniqueIdFormat informs the user what VPD Page 0x83 descriptor type was used to populate the UniqueId field.
    #[serde(rename = "UniqueIdFormat")]
    pub unique_id_format: Option<Disk_UniqueIdFormat>,
}

impl MSFT_Disk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            adapter_serial_number: None,
            allocated_size: None,
            boot_from_disk: None,
            bus_type: None,
            firmware_version: None,
            friendly_name: None,
            guid: None,
            health_status: None,
            is_boot: None,
            is_clustered: None,
            is_highly_available: None,
            is_offline: None,
            is_read_only: None,
            is_scale_out: None,
            is_system: None,
            largest_free_extent: None,
            location: None,
            logical_sector_size: None,
            manufacturer: None,
            model: None,
            number: None,
            number_of_partitions: None,
            offline_reason: None,
            operational_status: Vec::new(),
            partition_style: None,
            path: None,
            physical_sector_size: None,
            provisioning_type: None,
            serial_number: None,
            signature: None,
            size: None,
            unique_id_format: None,
        }
    }


    /// Sets the value of AdapterSerialNumber
    pub fn set_adapter_serial_number(&mut self, value: String) {
        self.adapter_serial_number = Some(value);
    }

    /// Gets the value of AdapterSerialNumber
    pub fn get_adapter_serial_number(&self) -> Option<&String> {
        self.adapter_serial_number.as_ref()
    }

    /// Sets the value of AllocatedSize
    pub fn set_allocated_size(&mut self, value: u64) {
        self.allocated_size = Some(value);
    }

    /// Gets the value of AllocatedSize
    pub fn get_allocated_size(&self) -> Option<&u64> {
        self.allocated_size.as_ref()
    }

    /// Sets the value of BootFromDisk
    pub fn set_boot_from_disk(&mut self, value: bool) {
        self.boot_from_disk = Some(value);
    }

    /// Gets the value of BootFromDisk
    pub fn get_boot_from_disk(&self) -> Option<&bool> {
        self.boot_from_disk.as_ref()
    }

    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: Disk_BusType) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&Disk_BusType> {
        self.bus_type.as_ref()
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

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: Disk_HealthStatus) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&Disk_HealthStatus> {
        self.health_status.as_ref()
    }

    /// Sets the value of IsBoot
    pub fn set_is_boot(&mut self, value: bool) {
        self.is_boot = Some(value);
    }

    /// Gets the value of IsBoot
    pub fn get_is_boot(&self) -> Option<&bool> {
        self.is_boot.as_ref()
    }

    /// Sets the value of IsClustered
    pub fn set_is_clustered(&mut self, value: bool) {
        self.is_clustered = Some(value);
    }

    /// Gets the value of IsClustered
    pub fn get_is_clustered(&self) -> Option<&bool> {
        self.is_clustered.as_ref()
    }

    /// Sets the value of IsHighlyAvailable
    pub fn set_is_highly_available(&mut self, value: bool) {
        self.is_highly_available = Some(value);
    }

    /// Gets the value of IsHighlyAvailable
    pub fn get_is_highly_available(&self) -> Option<&bool> {
        self.is_highly_available.as_ref()
    }

    /// Sets the value of IsOffline
    pub fn set_is_offline(&mut self, value: bool) {
        self.is_offline = Some(value);
    }

    /// Gets the value of IsOffline
    pub fn get_is_offline(&self) -> Option<&bool> {
        self.is_offline.as_ref()
    }

    /// Sets the value of IsReadOnly
    pub fn set_is_read_only(&mut self, value: bool) {
        self.is_read_only = Some(value);
    }

    /// Gets the value of IsReadOnly
    pub fn get_is_read_only(&self) -> Option<&bool> {
        self.is_read_only.as_ref()
    }

    /// Sets the value of IsScaleOut
    pub fn set_is_scale_out(&mut self, value: bool) {
        self.is_scale_out = Some(value);
    }

    /// Gets the value of IsScaleOut
    pub fn get_is_scale_out(&self) -> Option<&bool> {
        self.is_scale_out.as_ref()
    }

    /// Sets the value of IsSystem
    pub fn set_is_system(&mut self, value: bool) {
        self.is_system = Some(value);
    }

    /// Gets the value of IsSystem
    pub fn get_is_system(&self) -> Option<&bool> {
        self.is_system.as_ref()
    }

    /// Sets the value of LargestFreeExtent
    pub fn set_largest_free_extent(&mut self, value: u64) {
        self.largest_free_extent = Some(value);
    }

    /// Gets the value of LargestFreeExtent
    pub fn get_largest_free_extent(&self) -> Option<&u64> {
        self.largest_free_extent.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of LogicalSectorSize
    pub fn set_logical_sector_size(&mut self, value: u32) {
        self.logical_sector_size = Some(value);
    }

    /// Gets the value of LogicalSectorSize
    pub fn get_logical_sector_size(&self) -> Option<&u32> {
        self.logical_sector_size.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of Number
    pub fn set_number(&mut self, value: u32) {
        self.number = Some(value);
    }

    /// Gets the value of Number
    pub fn get_number(&self) -> Option<&u32> {
        self.number.as_ref()
    }

    /// Sets the value of NumberOfPartitions
    pub fn set_number_of_partitions(&mut self, value: u32) {
        self.number_of_partitions = Some(value);
    }

    /// Gets the value of NumberOfPartitions
    pub fn get_number_of_partitions(&self) -> Option<&u32> {
        self.number_of_partitions.as_ref()
    }

    /// Sets the value of OfflineReason
    pub fn set_offline_reason(&mut self, value: Disk_OfflineReason) {
        self.offline_reason = Some(value);
    }

    /// Gets the value of OfflineReason
    pub fn get_offline_reason(&self) -> Option<&Disk_OfflineReason> {
        self.offline_reason.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<Disk_OperationalStatus>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<Disk_OperationalStatus> {
        &self.operational_status
    }

    /// Sets the value of PartitionStyle
    pub fn set_partition_style(&mut self, value: Disk_PartitionStyle) {
        self.partition_style = Some(value);
    }

    /// Gets the value of PartitionStyle
    pub fn get_partition_style(&self) -> Option<&Disk_PartitionStyle> {
        self.partition_style.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of PhysicalSectorSize
    pub fn set_physical_sector_size(&mut self, value: u32) {
        self.physical_sector_size = Some(value);
    }

    /// Gets the value of PhysicalSectorSize
    pub fn get_physical_sector_size(&self) -> Option<&u32> {
        self.physical_sector_size.as_ref()
    }

    /// Sets the value of ProvisioningType
    pub fn set_provisioning_type(&mut self, value: Disk_ProvisioningType) {
        self.provisioning_type = Some(value);
    }

    /// Gets the value of ProvisioningType
    pub fn get_provisioning_type(&self) -> Option<&Disk_ProvisioningType> {
        self.provisioning_type.as_ref()
    }

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: String) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&String> {
        self.serial_number.as_ref()
    }

    /// Sets the value of Signature
    pub fn set_signature(&mut self, value: u32) {
        self.signature = Some(value);
    }

    /// Gets the value of Signature
    pub fn get_signature(&self) -> Option<&u32> {
        self.signature.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of UniqueIdFormat
    pub fn set_unique_id_format(&mut self, value: Disk_UniqueIdFormat) {
        self.unique_id_format = Some(value);
    }

    /// Gets the value of UniqueIdFormat
    pub fn get_unique_id_format(&self) -> Option<&Disk_UniqueIdFormat> {
        self.unique_id_format.as_ref()
    }

/// 

    /// * `alignment` -  (u32)
    /// * `assign_drive_letter` -  (bool)
    /// * `drive_letter` -  (char)
    /// * `gpt_type` -  (String)
    /// * `is_active` -  (bool)
    /// * `is_hidden` -  (bool)
    /// * `mbr_type` -  (u16)
    /// * `offset` -  (u64)
    /// * `size` -  (u64)
    /// * `use_maximum_size` -  (bool)

    /// * `created_partition` -  (MSFT_Partition)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_partition(&self, size: u64, use_maximum_size: bool, offset: u64, alignment: u32, drive_letter: char, assign_drive_letter: bool, mbr_type: u16, gpt_type: &String, is_hidden: bool, is_active: bool, created_partition: &mut MSFT_Partition, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "UseMaximumSize".to_string(), value: use_maximum_size.into() });
        args.push(MethodParameter { name: "Offset".to_string(), value: offset.into() });
        args.push(MethodParameter { name: "Alignment".to_string(), value: alignment.into() });
        args.push(MethodParameter { name: "DriveLetter".to_string(), value: drive_letter.into() });
        args.push(MethodParameter { name: "AssignDriveLetter".to_string(), value: assign_drive_letter.into() });
        args.push(MethodParameter { name: "MbrType".to_string(), value: mbr_type.into() });
        args.push(MethodParameter { name: "GptType".to_string(), value: gpt_type.into() });
        args.push(MethodParameter { name: "IsHidden".to_string(), value: is_hidden.into() });
        args.push(MethodParameter { name: "IsActive".to_string(), value: is_active.into() });

        let result = self.invoke_method("CreatePartition", &args)?;
        let created_partition = result.get_value("CreatedPartition")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `partition_style` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn initialize(&self, partition_style: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PartitionStyle".to_string(), value: partition_style.into() });

        let result = self.invoke_method("Initialize", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `remove_data` -  (bool)
    /// * `remove_oem` -  (bool)
    /// * `sanitize` -  (bool)
    /// * `zero_out_entire_disk` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn clear(&self, remove_data: bool, remove_oem: bool, zero_out_entire_disk: bool, sanitize: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RemoveData".to_string(), value: remove_data.into() });
        args.push(MethodParameter { name: "RemoveOEM".to_string(), value: remove_oem.into() });
        args.push(MethodParameter { name: "ZeroOutEntireDisk".to_string(), value: zero_out_entire_disk.into() });
        args.push(MethodParameter { name: "Sanitize".to_string(), value: sanitize.into() });

        let result = self.invoke_method("Clear", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `partition_style` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn convert_style(&self, partition_style: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PartitionStyle".to_string(), value: partition_style.into() });

        let result = self.invoke_method("ConvertStyle", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn offline(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Offline", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn online(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Online", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `guid` -  (String)
    /// * `is_read_only` -  (bool)
    /// * `signature` -  (u32)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, is_read_only: bool, signature: u32, guid: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsReadOnly".to_string(), value: is_read_only.into() });
        args.push(MethodParameter { name: "Signature".to_string(), value: signature.into() });
        args.push(MethodParameter { name: "Guid".to_string(), value: guid.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn refresh(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Refresh", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_path` -  (String)
    /// * `allocation_unit_size` -  (u32)
    /// * `file_system` -  (u16)
    /// * `friendly_name` -  (String)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_volume` -  (MSFT_Volume)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_volume(&self, friendly_name: &String, file_system: u16, access_path: &String, allocation_unit_size: u32, created_volume: &mut MSFT_Volume, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });

        let result = self.invoke_method("CreateVolume", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_volume = result.get_value("CreatedVolume")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `scale_out` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn enable_high_availability(&self, scale_out: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScaleOut".to_string(), value: scale_out.into() });

        let result = self.invoke_method("EnableHighAvailability", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn disable_high_availability(&self, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DisableHighAvailability", &[])?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

