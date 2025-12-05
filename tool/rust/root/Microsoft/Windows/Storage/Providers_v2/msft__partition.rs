// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Partition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Partition {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// This property is an array of all the various mount points for the partition. This list includes drive letters, as well as mounted folders.
    #[serde(rename = "AccessPaths")]
    pub access_paths: Vec<String>,

/// This property is identical to the ObjectId field of the disk object that contains this partition.
    #[serde(rename = "DiskId")]
    pub disk_id: Option<String>,

/// The operating system's number for the disk that contains this partition. Disk numbers may not necessarily remain the same across reboots.
    #[serde(rename = "DiskNumber")]
    pub disk_number: Option<u32>,

/// The currently assigned drive letter to the partition. This property is NULL if no drive letter has been assigned.
    #[serde(rename = "DriveLetter")]
    pub drive_letter: Option<char>,

/// This property indicates the partition's GPT type. This property is only valid when the disk's PartitionStyle property is set to 2 - 'GPT' and will be NULL for all other partition styles.
    #[serde(rename = "GptType")]
    pub gpt_type: Option<Partition_GptType>,

/// This property is a string representation of the partition's GPT GUID. This property is only valid if the disk's PartitionStyle property is set to 2 - 'GPT' and will be NULL for all other partition stlyes.
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// Signifies whether or not the partition is active and can be booted. This property is only relevant for MBR disks.
    #[serde(rename = "IsActive")]
    pub is_active: Option<bool>,

/// 
    #[serde(rename = "IsBoot")]
    pub is_boot: Option<bool>,

/// 
    #[serde(rename = "IsDAX")]
    pub is_dax: Option<bool>,

/// If this property is set to TRUE, the partition is not detected by the mount manager. As a result, the partition does not receive a drive letter, does not receive a volume GUID path, does not host volume mount points, and is not enumerated by calls to FindFirstVolume and FindNextVolume. This ensures that applications such as disk defragmenter do not access the partition. The Volume Shadow Copy Service (VSS) uses this attribute on its shadow copies.
    #[serde(rename = "IsHidden")]
    pub is_hidden: Option<bool>,

/// 
    #[serde(rename = "IsOffline")]
    pub is_offline: Option<bool>,

/// 
    #[serde(rename = "IsReadOnly")]
    pub is_read_only: Option<bool>,

/// If this property is set to TRUE, the partition is a shadow copy of another partition. This attribute is used by the Volume Shadow Copy service (VSS). This attribute is an indication for file system filter driver-based software (such as antivirus programs) to avoid attaching to the volume. An application can use this attribute to differentiate a shadow copy partition from a production partition. For example, an application that performs a fast recovery will break a shadow copy virtual disk by clearing the read-only and hidden attributes and this attribute. This attribute is set when the shadow copy is created and cleared when the shadow copy is broken.
    #[serde(rename = "IsShadowCopy")]
    pub is_shadow_copy: Option<bool>,

/// 
    #[serde(rename = "IsSystem")]
    pub is_system: Option<bool>,

/// This property indicates the partition's MBR type. This property is only valid when the disk's PartitionStyle property is set to 1 - 'MBR' and will be NULL for all other partition styles.
    #[serde(rename = "MbrType")]
    pub mbr_type: Option<Partition_MbrType>,

/// If this property is set to TRUE, the operating system does not assign a drive letter automatically when the partition is discovered. This is only honored for GPT disks and is assumed to be FALSE for MBR disks. This attribute is useful in storage area network (SAN) environments.
    #[serde(rename = "NoDefaultDriveLetter")]
    pub no_default_drive_letter: Option<bool>,

/// This property indicates the partition's offset from the beginning of the disk, measured in bytes.
    #[serde(rename = "Offset")]
    pub offset: Option<u64>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Option<Partition_OperationalStatus>,

/// The operating system's number for the partition. Ordering is based on the partition's offset, relative to other partitions. This means that the value for this property may change based off of the partition configuration in the offset range preceding this partition.
    #[serde(rename = "PartitionNumber")]
    pub partition_number: Option<u32>,

/// Total size of the partition, measured in bytes.
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "TransitionState")]
    pub transition_state: Option<u16>,
}

impl MSFT_Partition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            access_paths: Vec::new(),
            disk_id: None,
            disk_number: None,
            drive_letter: None,
            gpt_type: None,
            guid: None,
            is_active: None,
            is_boot: None,
            is_dax: None,
            is_hidden: None,
            is_offline: None,
            is_read_only: None,
            is_shadow_copy: None,
            is_system: None,
            mbr_type: None,
            no_default_drive_letter: None,
            offset: None,
            operational_status: None,
            partition_number: None,
            size: None,
            transition_state: None,
        }
    }


    /// Sets the value of AccessPaths
    pub fn set_access_paths(&mut self, value: Vec<String>) {
        self.access_paths = value;
    }

    /// Gets the value of AccessPaths
    pub fn get_access_paths(&self) -> &Vec<String> {
        &self.access_paths
    }

    /// Sets the value of DiskId
    pub fn set_disk_id(&mut self, value: String) {
        self.disk_id = Some(value);
    }

    /// Gets the value of DiskId
    pub fn get_disk_id(&self) -> Option<&String> {
        self.disk_id.as_ref()
    }

    /// Sets the value of DiskNumber
    pub fn set_disk_number(&mut self, value: u32) {
        self.disk_number = Some(value);
    }

    /// Gets the value of DiskNumber
    pub fn get_disk_number(&self) -> Option<&u32> {
        self.disk_number.as_ref()
    }

    /// Sets the value of DriveLetter
    pub fn set_drive_letter(&mut self, value: char) {
        self.drive_letter = Some(value);
    }

    /// Gets the value of DriveLetter
    pub fn get_drive_letter(&self) -> Option<&char> {
        self.drive_letter.as_ref()
    }

    /// Sets the value of GptType
    pub fn set_gpt_type(&mut self, value: Partition_GptType) {
        self.gpt_type = Some(value);
    }

    /// Gets the value of GptType
    pub fn get_gpt_type(&self) -> Option<&Partition_GptType> {
        self.gpt_type.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of IsActive
    pub fn set_is_active(&mut self, value: bool) {
        self.is_active = Some(value);
    }

    /// Gets the value of IsActive
    pub fn get_is_active(&self) -> Option<&bool> {
        self.is_active.as_ref()
    }

    /// Sets the value of IsBoot
    pub fn set_is_boot(&mut self, value: bool) {
        self.is_boot = Some(value);
    }

    /// Gets the value of IsBoot
    pub fn get_is_boot(&self) -> Option<&bool> {
        self.is_boot.as_ref()
    }

    /// Sets the value of IsDAX
    pub fn set_is_dax(&mut self, value: bool) {
        self.is_dax = Some(value);
    }

    /// Gets the value of IsDAX
    pub fn get_is_dax(&self) -> Option<&bool> {
        self.is_dax.as_ref()
    }

    /// Sets the value of IsHidden
    pub fn set_is_hidden(&mut self, value: bool) {
        self.is_hidden = Some(value);
    }

    /// Gets the value of IsHidden
    pub fn get_is_hidden(&self) -> Option<&bool> {
        self.is_hidden.as_ref()
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

    /// Sets the value of IsShadowCopy
    pub fn set_is_shadow_copy(&mut self, value: bool) {
        self.is_shadow_copy = Some(value);
    }

    /// Gets the value of IsShadowCopy
    pub fn get_is_shadow_copy(&self) -> Option<&bool> {
        self.is_shadow_copy.as_ref()
    }

    /// Sets the value of IsSystem
    pub fn set_is_system(&mut self, value: bool) {
        self.is_system = Some(value);
    }

    /// Gets the value of IsSystem
    pub fn get_is_system(&self) -> Option<&bool> {
        self.is_system.as_ref()
    }

    /// Sets the value of MbrType
    pub fn set_mbr_type(&mut self, value: Partition_MbrType) {
        self.mbr_type = Some(value);
    }

    /// Gets the value of MbrType
    pub fn get_mbr_type(&self) -> Option<&Partition_MbrType> {
        self.mbr_type.as_ref()
    }

    /// Sets the value of NoDefaultDriveLetter
    pub fn set_no_default_drive_letter(&mut self, value: bool) {
        self.no_default_drive_letter = Some(value);
    }

    /// Gets the value of NoDefaultDriveLetter
    pub fn get_no_default_drive_letter(&self) -> Option<&bool> {
        self.no_default_drive_letter.as_ref()
    }

    /// Sets the value of Offset
    pub fn set_offset(&mut self, value: u64) {
        self.offset = Some(value);
    }

    /// Gets the value of Offset
    pub fn get_offset(&self) -> Option<&u64> {
        self.offset.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Partition_OperationalStatus) {
        self.operational_status = Some(value);
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> Option<&Partition_OperationalStatus> {
        self.operational_status.as_ref()
    }

    /// Sets the value of PartitionNumber
    pub fn set_partition_number(&mut self, value: u32) {
        self.partition_number = Some(value);
    }

    /// Gets the value of PartitionNumber
    pub fn get_partition_number(&self) -> Option<&u32> {
        self.partition_number.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of TransitionState
    pub fn set_transition_state(&mut self, value: u16) {
        self.transition_state = Some(value);
    }

    /// Gets the value of TransitionState
    pub fn get_transition_state(&self) -> Option<&u16> {
        self.transition_state.as_ref()
    }

/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn delete_object(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("DeleteObject", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_paths` -  (String[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_access_paths(&self, access_paths: &mut Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAccessPaths", &[])?;
        let access_paths = result.get_value("AccessPaths")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_path` -  (String)
    /// * `assign_drive_letter` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn add_access_path(&self, access_path: &String, assign_drive_letter: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });
        args.push(MethodParameter { name: "AssignDriveLetter".to_string(), value: assign_drive_letter.into() });

        let result = self.invoke_method("AddAccessPath", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `access_path` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_access_path(&self, access_path: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AccessPath".to_string(), value: access_path.into() });

        let result = self.invoke_method("RemoveAccessPath", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `size` -  (u64)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn resize(&self, size: u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });

        let result = self.invoke_method("Resize", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `size_max` -  (u64)
    /// * `size_min` -  (u64)
    pub fn get_supported_size(&self, size_min: &mut u64, size_max: &mut u64, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSupportedSize", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let size_max = result.get_value("SizeMax")?;
        let size_min = result.get_value("SizeMin")?;
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

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn offline(&self, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Offline", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `gpt_type` -  (String)
    /// * `is_active` -  (bool)
    /// * `is_dax` -  (bool)
    /// * `is_hidden` -  (bool)
    /// * `is_read_only` -  (bool)
    /// * `is_shadow_copy` -  (bool)
    /// * `mbr_type` -  (u16)
    /// * `no_default_drive_letter` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, is_read_only: bool, no_default_drive_letter: bool, is_active: bool, is_hidden: bool, is_shadow_copy: bool, is_dax: bool, mbr_type: u16, gpt_type: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IsReadOnly".to_string(), value: is_read_only.into() });
        args.push(MethodParameter { name: "NoDefaultDriveLetter".to_string(), value: no_default_drive_letter.into() });
        args.push(MethodParameter { name: "IsActive".to_string(), value: is_active.into() });
        args.push(MethodParameter { name: "IsHidden".to_string(), value: is_hidden.into() });
        args.push(MethodParameter { name: "IsShadowCopy".to_string(), value: is_shadow_copy.into() });
        args.push(MethodParameter { name: "IsDAX".to_string(), value: is_dax.into() });
        args.push(MethodParameter { name: "MbrType".to_string(), value: mbr_type.into() });
        args.push(MethodParameter { name: "GptType".to_string(), value: gpt_type.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

