// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PhysicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PhysicalDisk {
    #[serde(flatten)]
    pub base: MSFT_StorageFaultDomain,

/// A string representation of the Adapter's serial number.
    #[serde(rename = "AdapterSerialNumber")]
    pub adapter_serial_number: Option<String>,

/// This field indicates the sum of used space on this physical disk. This should include usage from all storage pools and other data stored on the disk.
    #[serde(rename = "AllocatedSize")]
    pub allocated_size: Option<u64>,

/// 
    #[serde(rename = "BusType")]
    pub bus_type: Option<PhysicalDisk_BusType>,

/// Indicates the reason why this physical disk cannot be added to a concrete pool
    #[serde(rename = "CannotPoolReason")]
    pub cannot_pool_reason: Vec<PhysicalDisk_CannotPoolReason>,

/// Indicates whether this physical disk can be added to a concrete pool or not
    #[serde(rename = "CanPool")]
    pub can_pool: Option<bool>,

/// DeviceId is an address or other identifier that uniquely names the physical disk.
    #[serde(rename = "DeviceId")]
    pub device_id: Option<String>,

/// Indicates the enclosure number in which the disk physically resides
    #[serde(rename = "EnclosureNumber")]
    pub enclosure_number: Option<u16>,

/// This field is a string representation of the physical disk's firmware version.
    #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,

/// FruId is an identifier of the replacement unit housing the physical disk.
    #[serde(rename = "FruId")]
    pub fru_id: Option<String>,

/// Indicates whether the physical disk's identification LEDs are active or not. This is typically used in maintenance operations.
    #[serde(rename = "IsIndicationEnabled")]
    pub is_indication_enabled: Option<bool>,

/// Indicates whether this physical disk is partially consumed by a system or service whose use is outside of normal storage pool operations.
    #[serde(rename = "IsPartial")]
    pub is_partial: Option<bool>,

/// This field indicates the logical sector size of the physical disk in bytes. For example: a 4K native disk should report 4096, while a 512 emulated disk should report 512.
    #[serde(rename = "LogicalSectorSize")]
    pub logical_sector_size: Option<u64>,

/// Media type of this physical disk
    #[serde(rename = "MediaType")]
    pub media_type: Option<PhysicalDisk_MediaType>,

/// If CannotPoolReason contains 1 - 'Other', this field contains the string representing the vendor defined reason why this physical disk cannot be added to a concrete pool. This property must be NULL if CannotPoolReason does not contain 1 - 'Other'.
    #[serde(rename = "OtherCannotPoolReasonDescription")]
    pub other_cannot_pool_reason_description: Option<String>,

/// This field is a string representation of the physical disk's part number or SKU.
    #[serde(rename = "PartNumber")]
    pub part_number: Option<String>,

/// This field indicates the physical sector size of the physical disk in bytes. For example: for 4K native and 512 emulated disks, the value should be 4096.
    #[serde(rename = "PhysicalSectorSize")]
    pub physical_sector_size: Option<u64>,

/// Indicates the total physical storage size of the disk in bytes
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// Indicates the enclosure slot number in which the disk physically resides
    #[serde(rename = "SlotNumber")]
    pub slot_number: Option<u16>,

/// This field is a string representation of the physical disk's software version.
    #[serde(rename = "SoftwareVersion")]
    pub software_version: Option<String>,

/// This field indicates the rotational speed of spindle-based physical disks. For solid state devices (SSDs) or other non-rotational media, this field should set to 0. For rotating media which has an unknown speed, this field should be set to -1 (UINT32_MAX).
    #[serde(rename = "SpindleSpeed")]
    pub spindle_speed: Option<u32>,

/// 
    #[serde(rename = "StoragePoolUniqueId")]
    pub storage_pool_unique_id: Option<String>,

/// This field describes the supported usages of this physical disk.
    #[serde(rename = "SupportedUsages")]
    pub supported_usages: Vec<PhysicalDisk_SupportedUsages>,

/// UniqueIdFormat indicates the type of identifier used in the UniqueId field. The identifier used in UniqueId must be the highest available identifier using the following order of preference: 8 (highest), 3, 2, 1, 0 (lowest). For example: if the physical disk device exposes identifiers of type 0, 1, and 3, UniqueId must be the identifier of type 3, and UniqueIdFormat should be set to 3.
    #[serde(rename = "UniqueIdFormat")]
    pub unique_id_format: Option<PhysicalDisk_UniqueIdFormat>,

/// This field describes the intended usage of this physical disk within a concrete pool. Storage pools are required to follow the assigned policy for a physical disk. 
/// 1 - 'Auto-Select': This physical disk should only be used for data storage. 
/// 2 - 'Manual-Select': This physical disk should only be used if manually selected by an administrator at the time of virtual disk creation. A manual-select disk is selected using the PhysicalDisksToUse parameter to CreateVirtualDisk. 
/// 3 - 'Hot Spare': This physical disk should be used as a hot spare. 
/// 4 - 'Retired': This physical disk should be retired from use. At a minimum, no new allocations should go to this disk. If the virtual disks that reside on this disk are repaired, the data should be moved to another active physical disk.
    #[serde(rename = "Usage")]
    pub usage: Option<PhysicalDisk_Usage>,

/// This field indicates the size in bytes of the user data footprint from virtual disks on this physical disk.
    #[serde(rename = "VirtualDiskFootprint")]
    pub virtual_disk_footprint: Option<u64>,
}

impl MSFT_PhysicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageFaultDomain::new(),
            adapter_serial_number: None,
            allocated_size: None,
            bus_type: None,
            cannot_pool_reason: Vec::new(),
            can_pool: None,
            device_id: None,
            enclosure_number: None,
            firmware_version: None,
            fru_id: None,
            is_indication_enabled: None,
            is_partial: None,
            logical_sector_size: None,
            media_type: None,
            other_cannot_pool_reason_description: None,
            part_number: None,
            physical_sector_size: None,
            size: None,
            slot_number: None,
            software_version: None,
            spindle_speed: None,
            storage_pool_unique_id: None,
            supported_usages: Vec::new(),
            unique_id_format: None,
            usage: None,
            virtual_disk_footprint: None,
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

    /// Sets the value of BusType
    pub fn set_bus_type(&mut self, value: PhysicalDisk_BusType) {
        self.bus_type = Some(value);
    }

    /// Gets the value of BusType
    pub fn get_bus_type(&self) -> Option<&PhysicalDisk_BusType> {
        self.bus_type.as_ref()
    }

    /// Sets the value of CannotPoolReason
    pub fn set_cannot_pool_reason(&mut self, value: Vec<PhysicalDisk_CannotPoolReason>) {
        self.cannot_pool_reason = value;
    }

    /// Gets the value of CannotPoolReason
    pub fn get_cannot_pool_reason(&self) -> &Vec<PhysicalDisk_CannotPoolReason> {
        &self.cannot_pool_reason
    }

    /// Sets the value of CanPool
    pub fn set_can_pool(&mut self, value: bool) {
        self.can_pool = Some(value);
    }

    /// Gets the value of CanPool
    pub fn get_can_pool(&self) -> Option<&bool> {
        self.can_pool.as_ref()
    }

    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of EnclosureNumber
    pub fn set_enclosure_number(&mut self, value: u16) {
        self.enclosure_number = Some(value);
    }

    /// Gets the value of EnclosureNumber
    pub fn get_enclosure_number(&self) -> Option<&u16> {
        self.enclosure_number.as_ref()
    }

    /// Sets the value of FirmwareVersion
    pub fn set_firmware_version(&mut self, value: String) {
        self.firmware_version = Some(value);
    }

    /// Gets the value of FirmwareVersion
    pub fn get_firmware_version(&self) -> Option<&String> {
        self.firmware_version.as_ref()
    }

    /// Sets the value of FruId
    pub fn set_fru_id(&mut self, value: String) {
        self.fru_id = Some(value);
    }

    /// Gets the value of FruId
    pub fn get_fru_id(&self) -> Option<&String> {
        self.fru_id.as_ref()
    }

    /// Sets the value of IsIndicationEnabled
    pub fn set_is_indication_enabled(&mut self, value: bool) {
        self.is_indication_enabled = Some(value);
    }

    /// Gets the value of IsIndicationEnabled
    pub fn get_is_indication_enabled(&self) -> Option<&bool> {
        self.is_indication_enabled.as_ref()
    }

    /// Sets the value of IsPartial
    pub fn set_is_partial(&mut self, value: bool) {
        self.is_partial = Some(value);
    }

    /// Gets the value of IsPartial
    pub fn get_is_partial(&self) -> Option<&bool> {
        self.is_partial.as_ref()
    }

    /// Sets the value of LogicalSectorSize
    pub fn set_logical_sector_size(&mut self, value: u64) {
        self.logical_sector_size = Some(value);
    }

    /// Gets the value of LogicalSectorSize
    pub fn get_logical_sector_size(&self) -> Option<&u64> {
        self.logical_sector_size.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: PhysicalDisk_MediaType) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&PhysicalDisk_MediaType> {
        self.media_type.as_ref()
    }

    /// Sets the value of OtherCannotPoolReasonDescription
    pub fn set_other_cannot_pool_reason_description(&mut self, value: String) {
        self.other_cannot_pool_reason_description = Some(value);
    }

    /// Gets the value of OtherCannotPoolReasonDescription
    pub fn get_other_cannot_pool_reason_description(&self) -> Option<&String> {
        self.other_cannot_pool_reason_description.as_ref()
    }

    /// Sets the value of PartNumber
    pub fn set_part_number(&mut self, value: String) {
        self.part_number = Some(value);
    }

    /// Gets the value of PartNumber
    pub fn get_part_number(&self) -> Option<&String> {
        self.part_number.as_ref()
    }

    /// Sets the value of PhysicalSectorSize
    pub fn set_physical_sector_size(&mut self, value: u64) {
        self.physical_sector_size = Some(value);
    }

    /// Gets the value of PhysicalSectorSize
    pub fn get_physical_sector_size(&self) -> Option<&u64> {
        self.physical_sector_size.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of SlotNumber
    pub fn set_slot_number(&mut self, value: u16) {
        self.slot_number = Some(value);
    }

    /// Gets the value of SlotNumber
    pub fn get_slot_number(&self) -> Option<&u16> {
        self.slot_number.as_ref()
    }

    /// Sets the value of SoftwareVersion
    pub fn set_software_version(&mut self, value: String) {
        self.software_version = Some(value);
    }

    /// Gets the value of SoftwareVersion
    pub fn get_software_version(&self) -> Option<&String> {
        self.software_version.as_ref()
    }

    /// Sets the value of SpindleSpeed
    pub fn set_spindle_speed(&mut self, value: u32) {
        self.spindle_speed = Some(value);
    }

    /// Gets the value of SpindleSpeed
    pub fn get_spindle_speed(&self) -> Option<&u32> {
        self.spindle_speed.as_ref()
    }

    /// Sets the value of StoragePoolUniqueId
    pub fn set_storage_pool_unique_id(&mut self, value: String) {
        self.storage_pool_unique_id = Some(value);
    }

    /// Gets the value of StoragePoolUniqueId
    pub fn get_storage_pool_unique_id(&self) -> Option<&String> {
        self.storage_pool_unique_id.as_ref()
    }

    /// Sets the value of SupportedUsages
    pub fn set_supported_usages(&mut self, value: Vec<PhysicalDisk_SupportedUsages>) {
        self.supported_usages = value;
    }

    /// Gets the value of SupportedUsages
    pub fn get_supported_usages(&self) -> &Vec<PhysicalDisk_SupportedUsages> {
        &self.supported_usages
    }

    /// Sets the value of UniqueIdFormat
    pub fn set_unique_id_format(&mut self, value: PhysicalDisk_UniqueIdFormat) {
        self.unique_id_format = Some(value);
    }

    /// Gets the value of UniqueIdFormat
    pub fn get_unique_id_format(&self) -> Option<&PhysicalDisk_UniqueIdFormat> {
        self.unique_id_format.as_ref()
    }

    /// Sets the value of Usage
    pub fn set_usage(&mut self, value: PhysicalDisk_Usage) {
        self.usage = Some(value);
    }

    /// Gets the value of Usage
    pub fn get_usage(&self) -> Option<&PhysicalDisk_Usage> {
        self.usage.as_ref()
    }

    /// Sets the value of VirtualDiskFootprint
    pub fn set_virtual_disk_footprint(&mut self, value: u64) {
        self.virtual_disk_footprint = Some(value);
    }

    /// Gets the value of VirtualDiskFootprint
    pub fn get_virtual_disk_footprint(&self) -> Option<&u64> {
        self.virtual_disk_footprint.as_ref()
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


/// This method allows a user to perform certain maintenance tasks on the physical disk. 

    /// * `enable_indication` - If set to TRUE, this instructs the physical disk to enable its indication LED. The indication LED should remain enabled until a second call to Maintenance is made with this parameter specified as FALSE. (bool)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn maintenance(&self, enable_indication: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnableIndication".to_string(), value: enable_indication.into() });

        let result = self.invoke_method("Maintenance", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable_indication` -  (bool)
    /// * `enable_maintenance_mode` -  (bool)
    /// * `ignore_detached_virtual_disks` -  (bool)
    /// * `timeout` -  (u32)
    /// * `validate_maintenance_mode` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn maintenance2(&self, enable_indication: bool, validate_maintenance_mode: bool, enable_maintenance_mode: bool, timeout: u32, ignore_detached_virtual_disks: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnableIndication".to_string(), value: enable_indication.into() });
        args.push(MethodParameter { name: "ValidateMaintenanceMode".to_string(), value: validate_maintenance_mode.into() });
        args.push(MethodParameter { name: "EnableMaintenanceMode".to_string(), value: enable_maintenance_mode.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "IgnoreDetachedVirtualDisks".to_string(), value: ignore_detached_virtual_disks.into() });

        let result = self.invoke_method("Maintenance2", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method resets the health and operational status of the physical disk. Exact behavior of this method is dependent on whether this physical disk belongs to a concrete pool. 
/// If it is a member of a concrete pool, the health and operational statuses should be reset to 1 - 'Healthy', and 1 - 'OK', respectively. If any additional errors are detected after Reset, the health and operational statuses should reflect these new errors. 
/// If the physical disk is not a member of a concrete pool, then this method should not only reset the health and operational statuses, but it should return the disk into a state where it is usable as storage for a concrete pool. For example: If a physical disk had become missing and then has reappeared (after it has been replaced) this physical disk is expected to be in the primordial pool only with an operational status indicating its data is either split or unrecognized. Calling Reset should clear the physical disk of any data, remove any remaining ties to its former concrete pool, and return the disk to a healthy, usable state.

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn reset(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Reset", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the physical disk to be renamed.

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


/// This method allows the physical disk's description to be changed.

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


/// This method allows the physical disk's usage to be updated.

    /// * `usage` - This field describes the intended usage of this physical disk within a concrete pool. Storage pools are required to follow the assigned policy for a physical disk.  1 - 'Auto-Select': This physical disk should only be used for data storage.  2 - 'Manual-Select': This physical disk should only be used if manually selected by an administrator at the time of virtual disk creation. A manual-select disk is selected using the PhysicalDisksToUse parameter to CreateVirtualDisk.  3 - 'Hot Spare': This physical disk should be used as a hot spare.  4 - 'Retired': This physical disk should be retired from use. At a minimum, no new allocations should go to this disk. If the virtual disks that reside on this disk are repaired, the data should be moved to another active physical disk. (PhysicalDisk_Usage)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_usage(&self, usage: PhysicalDisk_Usage, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Usage".to_string(), value: usage.into() });

        let result = self.invoke_method("SetUsage", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// This method allows the physical disk's attributes to be updated.

    /// * `media_type` - Media type of this physical disk (PhysicalDisk_MediaType)

    /// * `extended_status` - ExtendedStatus allows the storage provider to return extended (implementation specific) error information. (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, media_type: PhysicalDisk_MediaType, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `is_hidden` -  (bool)
    /// * `media_type` -  (u16)
    /// * `storage_enclosure_id` -  (String)
    /// * `storage_scale_unit_id` -  (String)

    /// * `extended_status` -  (String)
    /// * `return_value` -  (u32)
    pub fn set_attributes2(&self, media_type: u16, storage_enclosure_id: &String, storage_scale_unit_id: &String, is_hidden: bool, extended_status: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "MediaType".to_string(), value: media_type.into() });
        args.push(MethodParameter { name: "StorageEnclosureId".to_string(), value: storage_enclosure_id.into() });
        args.push(MethodParameter { name: "StorageScaleUnitId".to_string(), value: storage_scale_unit_id.into() });
        args.push(MethodParameter { name: "IsHidden".to_string(), value: is_hidden.into() });

        let result = self.invoke_method("SetAttributes2", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `is_device_cache_enabled` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_device_cache_enabled(&self, is_device_cache_enabled: &mut bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("IsDeviceCacheEnabled", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let is_device_cache_enabled = result.get_value("IsDeviceCacheEnabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `is_power_protected` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_power_protected(&self, is_power_protected: &mut bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("IsPowerProtected", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let is_power_protected = result.get_value("IsPowerProtected")?;
        Ok(result.return_value)

    }


/// 

    /// * `active_slot_number` -  (u16)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `firmware_version_in_slot` -  (String[])
    /// * `is_slot_writable` -  (bool[])
    /// * `number_of_slots` -  (u16)
    /// * `return_value` -  (u32)
    /// * `slot_number` -  (u16[])
    /// * `supports_update` -  (bool)
    pub fn get_firmware_information(&self, supports_update: &mut bool, number_of_slots: &mut u16, active_slot_number: &mut u16, slot_number: &mut Vec<u16>, is_slot_writable: &mut Vec<bool>, firmware_version_in_slot: &mut Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetFirmwareInformation", &[])?;
        let active_slot_number = result.get_value("ActiveSlotNumber")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let firmware_version_in_slot = result.get_value("FirmwareVersionInSlot")?;
        let is_slot_writable = result.get_value("IsSlotWritable")?;
        let number_of_slots = result.get_value("NumberOfSlots")?;
        let slot_number = result.get_value("SlotNumber")?;
        let supports_update = result.get_value("SupportsUpdate")?;
        Ok(result.return_value)

    }


/// 

    /// * `image_path` -  (String)
    /// * `slot_number` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn update_firmware(&self, image_path: &String, slot_number: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ImagePath".to_string(), value: image_path.into() });
        args.push(MethodParameter { name: "SlotNumber".to_string(), value: slot_number.into() });

        let result = self.invoke_method("UpdateFirmware", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `format` -  (u16)
    /// * `storage_pool_friendly_name` -  (String)
    /// * `storage_pool_metadata_length` -  (u64)
    /// * `storage_pool_minimum_allocation_size` -  (u64)
    /// * `storage_pool_version` -  (u16)
    /// * `virtual_disk_allocation_unit_size` -  (u64)
    /// * `virtual_disk_friendly_name` -  (String)
    /// * `virtual_disk_provisioning_type` -  (u16)
    /// * `virtual_disk_resiliency_setting_name` -  (u16)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `created_storage_object` -  (MSFT_StorageObject)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn convert(&self, format: u16, storage_pool_friendly_name: &String, storage_pool_version: u16, storage_pool_metadata_length: u64, storage_pool_minimum_allocation_size: u64, virtual_disk_friendly_name: &String, virtual_disk_provisioning_type: u16, virtual_disk_allocation_unit_size: u64, virtual_disk_resiliency_setting_name: u16, created_storage_object: &mut MSFT_StorageObject, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Format".to_string(), value: format.into() });
        args.push(MethodParameter { name: "StoragePoolFriendlyName".to_string(), value: storage_pool_friendly_name.into() });
        args.push(MethodParameter { name: "StoragePoolVersion".to_string(), value: storage_pool_version.into() });
        args.push(MethodParameter { name: "StoragePoolMetadataLength".to_string(), value: storage_pool_metadata_length.into() });
        args.push(MethodParameter { name: "StoragePoolMinimumAllocationSize".to_string(), value: storage_pool_minimum_allocation_size.into() });
        args.push(MethodParameter { name: "VirtualDiskFriendlyName".to_string(), value: virtual_disk_friendly_name.into() });
        args.push(MethodParameter { name: "VirtualDiskProvisioningType".to_string(), value: virtual_disk_provisioning_type.into() });
        args.push(MethodParameter { name: "VirtualDiskAllocationUnitSize".to_string(), value: virtual_disk_allocation_unit_size.into() });
        args.push(MethodParameter { name: "VirtualDiskResiliencySettingName".to_string(), value: virtual_disk_resiliency_setting_name.into() });

        let result = self.invoke_method("Convert", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let created_storage_object = result.get_value("CreatedStorageObject")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

