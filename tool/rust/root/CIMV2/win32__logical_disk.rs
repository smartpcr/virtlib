// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LogicalDisk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LogicalDisk {
    #[serde(flatten)]
    pub base: CIM_LogicalDisk,

/// 
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,

/// 
    #[serde(rename = "DriveType")]
    pub drive_type: Option<u32>,

/// 
    #[serde(rename = "FileSystem")]
    pub file_system: Option<String>,

/// 
    #[serde(rename = "MaximumComponentLength")]
    pub maximum_component_length: Option<u32>,

/// 
    #[serde(rename = "MediaType")]
    pub media_type: Option<u32>,

/// 
    #[serde(rename = "ProviderName")]
    pub provider_name: Option<String>,

/// 
    #[serde(rename = "QuotasDisabled")]
    pub quotas_disabled: Option<bool>,

/// 
    #[serde(rename = "QuotasIncomplete")]
    pub quotas_incomplete: Option<bool>,

/// 
    #[serde(rename = "QuotasRebuilding")]
    pub quotas_rebuilding: Option<bool>,

/// 
    #[serde(rename = "SupportsDiskQuotas")]
    pub supports_disk_quotas: Option<bool>,

/// 
    #[serde(rename = "SupportsFileBasedCompression")]
    pub supports_file_based_compression: Option<bool>,

/// 
    #[serde(rename = "VolumeDirty")]
    pub volume_dirty: Option<bool>,

/// 
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,

/// 
    #[serde(rename = "VolumeSerialNumber")]
    pub volume_serial_number: Option<String>,
}

impl Win32_LogicalDisk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDisk::new(),
            compressed: None,
            drive_type: None,
            file_system: None,
            maximum_component_length: None,
            media_type: None,
            provider_name: None,
            quotas_disabled: None,
            quotas_incomplete: None,
            quotas_rebuilding: None,
            supports_disk_quotas: None,
            supports_file_based_compression: None,
            volume_dirty: None,
            volume_name: None,
            volume_serial_number: None,
        }
    }


    /// Sets the value of Compressed
    pub fn set_compressed(&mut self, value: bool) {
        self.compressed = Some(value);
    }

    /// Gets the value of Compressed
    pub fn get_compressed(&self) -> Option<&bool> {
        self.compressed.as_ref()
    }

    /// Sets the value of DriveType
    pub fn set_drive_type(&mut self, value: u32) {
        self.drive_type = Some(value);
    }

    /// Gets the value of DriveType
    pub fn get_drive_type(&self) -> Option<&u32> {
        self.drive_type.as_ref()
    }

    /// Sets the value of FileSystem
    pub fn set_file_system(&mut self, value: String) {
        self.file_system = Some(value);
    }

    /// Gets the value of FileSystem
    pub fn get_file_system(&self) -> Option<&String> {
        self.file_system.as_ref()
    }

    /// Sets the value of MaximumComponentLength
    pub fn set_maximum_component_length(&mut self, value: u32) {
        self.maximum_component_length = Some(value);
    }

    /// Gets the value of MaximumComponentLength
    pub fn get_maximum_component_length(&self) -> Option<&u32> {
        self.maximum_component_length.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: u32) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&u32> {
        self.media_type.as_ref()
    }

    /// Sets the value of ProviderName
    pub fn set_provider_name(&mut self, value: String) {
        self.provider_name = Some(value);
    }

    /// Gets the value of ProviderName
    pub fn get_provider_name(&self) -> Option<&String> {
        self.provider_name.as_ref()
    }

    /// Sets the value of QuotasDisabled
    pub fn set_quotas_disabled(&mut self, value: bool) {
        self.quotas_disabled = Some(value);
    }

    /// Gets the value of QuotasDisabled
    pub fn get_quotas_disabled(&self) -> Option<&bool> {
        self.quotas_disabled.as_ref()
    }

    /// Sets the value of QuotasIncomplete
    pub fn set_quotas_incomplete(&mut self, value: bool) {
        self.quotas_incomplete = Some(value);
    }

    /// Gets the value of QuotasIncomplete
    pub fn get_quotas_incomplete(&self) -> Option<&bool> {
        self.quotas_incomplete.as_ref()
    }

    /// Sets the value of QuotasRebuilding
    pub fn set_quotas_rebuilding(&mut self, value: bool) {
        self.quotas_rebuilding = Some(value);
    }

    /// Gets the value of QuotasRebuilding
    pub fn get_quotas_rebuilding(&self) -> Option<&bool> {
        self.quotas_rebuilding.as_ref()
    }

    /// Sets the value of SupportsDiskQuotas
    pub fn set_supports_disk_quotas(&mut self, value: bool) {
        self.supports_disk_quotas = Some(value);
    }

    /// Gets the value of SupportsDiskQuotas
    pub fn get_supports_disk_quotas(&self) -> Option<&bool> {
        self.supports_disk_quotas.as_ref()
    }

    /// Sets the value of SupportsFileBasedCompression
    pub fn set_supports_file_based_compression(&mut self, value: bool) {
        self.supports_file_based_compression = Some(value);
    }

    /// Gets the value of SupportsFileBasedCompression
    pub fn get_supports_file_based_compression(&self) -> Option<&bool> {
        self.supports_file_based_compression.as_ref()
    }

    /// Sets the value of VolumeDirty
    pub fn set_volume_dirty(&mut self, value: bool) {
        self.volume_dirty = Some(value);
    }

    /// Gets the value of VolumeDirty
    pub fn get_volume_dirty(&self) -> Option<&bool> {
        self.volume_dirty.as_ref()
    }

    /// Sets the value of VolumeName
    pub fn set_volume_name(&mut self, value: String) {
        self.volume_name = Some(value);
    }

    /// Gets the value of VolumeName
    pub fn get_volume_name(&self) -> Option<&String> {
        self.volume_name.as_ref()
    }

    /// Sets the value of VolumeSerialNumber
    pub fn set_volume_serial_number(&mut self, value: String) {
        self.volume_serial_number = Some(value);
    }

    /// Gets the value of VolumeSerialNumber
    pub fn get_volume_serial_number(&self) -> Option<&String> {
        self.volume_serial_number.as_ref()
    }

/// 

    /// * `fix_errors` -  (bool)
    /// * `force_dismount` -  (bool)
    /// * `ok_to_run_at_boot_up` -  (bool)
    /// * `recover_bad_sectors` -  (bool)
    /// * `skip_folder_cycle` -  (bool)
    /// * `vigorous_index_check` -  (bool)

    /// * `return_value` -  (u32)
    pub fn chkdsk(&self, fix_errors: bool, vigorous_index_check: bool, skip_folder_cycle: bool, force_dismount: bool, recover_bad_sectors: bool, ok_to_run_at_boot_up: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FixErrors".to_string(), value: fix_errors.into() });
        args.push(MethodParameter { name: "VigorousIndexCheck".to_string(), value: vigorous_index_check.into() });
        args.push(MethodParameter { name: "SkipFolderCycle".to_string(), value: skip_folder_cycle.into() });
        args.push(MethodParameter { name: "ForceDismount".to_string(), value: force_dismount.into() });
        args.push(MethodParameter { name: "RecoverBadSectors".to_string(), value: recover_bad_sectors.into() });
        args.push(MethodParameter { name: "OkToRunAtBootUp".to_string(), value: ok_to_run_at_boot_up.into() });
        self.invoke_method("Chkdsk", &args)

    }


/// 

    /// * `logical_disk` -  (String[])

    /// * `return_value` -  (u32)
    pub fn schedule_auto_chk(&self, logical_disk: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LogicalDisk".to_string(), value: logical_disk.into() });
        self.invoke_method("ScheduleAutoChk", &args)

    }


/// 

    /// * `logical_disk` -  (String[])

    /// * `return_value` -  (u32)
    pub fn exclude_from_autochk(&self, logical_disk: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LogicalDisk".to_string(), value: logical_disk.into() });
        self.invoke_method("ExcludeFromAutochk", &args)

    }

}

