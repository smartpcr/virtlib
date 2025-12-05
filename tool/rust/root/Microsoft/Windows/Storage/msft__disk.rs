// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Disk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Disk {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "AdapterSerialNumber")]
    pub adapter_serial_number: Option<String>,

/// 
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// 
    #[serde(rename = "BootFromDisk")]
    pub boot_from_disk: Option<bool>,

/// 
    #[serde(rename = "BusType")]
    pub bus_type: Option<u16>,

/// 
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "IsBoot")]
    pub is_boot: Option<bool>,

/// 
    #[serde(rename = "IsClustered")]
    pub is_clustered: Option<bool>,

/// 
    #[serde(rename = "IsHighlyAvailable")]
    pub is_highly_available: Option<bool>,

/// 
    #[serde(rename = "IsOffline")]
    pub is_offline: Option<bool>,

/// 
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// 
    #[serde(rename = "IsScaleOut")]
    pub is_scale_out: Option<bool>,

/// 
    #[serde(rename = "IsSystem")]
    pub is_system: Option<bool>,

/// 
    #[serde(rename = "LargestFreeExtent")]
    pub largest_free_extent: Option<u64>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u32>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "Number")]
    pub number: Option<u32>,

/// 
    #[serde(rename = "NumberOfPartitions")]
    pub number_of_partitions: Option<u32>,

/// 
    #[serde(rename = "OfflineReason")]
    pub offline_reason: Option<u16>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "PartitionStyle")]
    pub partition_style: Option<u16>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u32>,

/// 
    #[serde(rename = "ProvisioningType")]
    pub provisioning_type: Option<u16>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<String>,

/// 
    #[serde(rename = "Signature")]
    pub signature: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "UniqueIdFormat")]
    pub unique_id_format: Option<u16>,
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
    pub fn set_bus_type(&mut self, value: u16) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&u16> {
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
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
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
    pub fn set_offline_reason(&mut self, value: u16) {
        self.offline_reason = Some(value);
    }

    /// Gets the value of OfflineReason
    pub fn get_offline_reason(&self) -> Option<&u16> {
        self.offline_reason.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of PartitionStyle
    pub fn set_partition_style(&mut self, value: u16) {
        self.partition_style = Some(value);
    }

    /// Gets the value of PartitionStyle
    pub fn get_partition_style(&self) -> Option<&u16> {
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
    pub fn set_provisioning_type(&mut self, value: u16) {
        self.provisioning_type = Some(value);
    }

    /// Gets the value of ProvisioningType
    pub fn get_provisioning_type(&self) -> Option<&u16> {
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
    pub fn set_unique_id_format(&mut self, value: u16) {
        self.unique_id_format = Some(value);
    }

    /// Gets the value of UniqueIdFormat
    pub fn get_unique_id_format(&self) -> Option<&u16> {
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
    /// * `run_as_job` -  (bool)
    /// * `sanitize` -  (bool)
    /// * `zero_out_entire_disk` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn clear(&self, remove_data: bool, remove_oem: bool, zero_out_entire_disk: bool, sanitize: bool, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RemoveData".to_string(), value: remove_data.into() });
        args.push(MethodParameter { name: "RemoveOEM".to_string(), value: remove_oem.into() });
        args.push(MethodParameter { name: "ZeroOutEntireDisk".to_string(), value: zero_out_entire_disk.into() });
        args.push(MethodParameter { name: "Sanitize".to_string(), value: sanitize.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

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
    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_volume` -  (MSFT_Volume)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn create_volume(&self, friendly_name: &String, file_system: u16, access_path: &String, allocation_unit_size: u32, created_volume: &mut MSFT_Volume, run_as_job: Option<bool>, created_storage_job: &mut Option<MSFT_StorageJob>, extended_status: &mut Option<MSFT_StorageExtendedStatus>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
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

    /// * `run_as_job` -  (bool)
    /// * `scale_out` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn enable_high_availability(&self, scale_out: bool, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ScaleOut".to_string(), value: scale_out.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("EnableHighAvailability", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `run_as_job` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn disable_high_availability(&self, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("DisableHighAvailability", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

