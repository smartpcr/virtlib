// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Volume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Volume {
    #[serde(flatten)]
    pub base: CIM_StorageVolume,

/// 
    #[serde(rename = "Automount")]
    pub automount: Option<bool>,

/// 
    #[serde(rename = "BootVolume")]
    pub boot_volume: Option<bool>,

/// 
    #[serde(rename = "Capacity")]
    pub capacity: Option<u64>,

/// 
    #[serde(rename = "Compressed")]
    pub compressed: Option<bool>,

/// 
    #[serde(rename = "DirtyBitSet")]
    pub dirty_bit_set: Option<bool>,

/// 
    #[serde(rename = "DriveLetter")]
    pub drive_letter: Option<String>,

/// 
    #[serde(rename = "DriveType")]
    pub drive_type: Option<u32>,

/// 
    #[serde(rename = "FileSystem")]
    pub file_system: Option<String>,

/// 
    #[serde(rename = "FreeSpace")]
    pub free_space: Option<u64>,

/// 
    #[serde(rename = "IndexingEnabled")]
    pub indexing_enabled: Option<bool>,

/// 
    #[serde(rename = "Label")]
    pub label: Option<String>,

/// 
    #[serde(rename = "MaximumFileNameLength")]
    pub maximum_file_name_length: Option<u32>,

/// 
    #[serde(rename = "PageFilePresent")]
    pub page_file_present: Option<bool>,

/// 
    #[serde(rename = "QuotasEnabled")]
    pub quotas_enabled: Option<bool>,

/// 
    #[serde(rename = "QuotasIncomplete")]
    pub quotas_incomplete: Option<bool>,

/// 
    #[serde(rename = "QuotasRebuilding")]
    pub quotas_rebuilding: Option<bool>,

/// 
    #[serde(rename = "SerialNumber")]
    pub serial_number: Option<u32>,

/// 
    #[serde(rename = "SupportsDiskQuotas")]
    pub supports_disk_quotas: Option<bool>,

/// 
    #[serde(rename = "SupportsFileBasedCompression")]
    pub supports_file_based_compression: Option<bool>,

/// 
    #[serde(rename = "SystemVolume")]
    pub system_volume: Option<bool>,
}

impl Win32_Volume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_StorageVolume::new(),
            automount: None,
            boot_volume: None,
            capacity: None,
            compressed: None,
            dirty_bit_set: None,
            drive_letter: None,
            drive_type: None,
            file_system: None,
            free_space: None,
            indexing_enabled: None,
            label: None,
            maximum_file_name_length: None,
            page_file_present: None,
            quotas_enabled: None,
            quotas_incomplete: None,
            quotas_rebuilding: None,
            serial_number: None,
            supports_disk_quotas: None,
            supports_file_based_compression: None,
            system_volume: None,
        }
    }


    /// Sets the value of Automount
    pub fn set_automount(&mut self, value: bool) {
        self.automount = Some(value);
    }

    /// Gets the value of Automount
    pub fn get_automount(&self) -> Option<&bool> {
        self.automount.as_ref()
    }

    /// Sets the value of BootVolume
    pub fn set_boot_volume(&mut self, value: bool) {
        self.boot_volume = Some(value);
    }

    /// Gets the value of BootVolume
    pub fn get_boot_volume(&self) -> Option<&bool> {
        self.boot_volume.as_ref()
    }

    /// Sets the value of Capacity
    pub fn set_capacity(&mut self, value: u64) {
        self.capacity = Some(value);
    }

    /// Gets the value of Capacity
    pub fn get_capacity(&self) -> Option<&u64> {
        self.capacity.as_ref()
    }

    /// Sets the value of Compressed
    pub fn set_compressed(&mut self, value: bool) {
        self.compressed = Some(value);
    }

    /// Gets the value of Compressed
    pub fn get_compressed(&self) -> Option<&bool> {
        self.compressed.as_ref()
    }

    /// Sets the value of DirtyBitSet
    pub fn set_dirty_bit_set(&mut self, value: bool) {
        self.dirty_bit_set = Some(value);
    }

    /// Gets the value of DirtyBitSet
    pub fn get_dirty_bit_set(&self) -> Option<&bool> {
        self.dirty_bit_set.as_ref()
    }

    /// Sets the value of DriveLetter
    pub fn set_drive_letter(&mut self, value: String) {
        self.drive_letter = Some(value);
    }

    /// Gets the value of DriveLetter
    pub fn get_drive_letter(&self) -> Option<&String> {
        self.drive_letter.as_ref()
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

    /// Sets the value of FreeSpace
    pub fn set_free_space(&mut self, value: u64) {
        self.free_space = Some(value);
    }

    /// Gets the value of FreeSpace
    pub fn get_free_space(&self) -> Option<&u64> {
        self.free_space.as_ref()
    }

    /// Sets the value of IndexingEnabled
    pub fn set_indexing_enabled(&mut self, value: bool) {
        self.indexing_enabled = Some(value);
    }

    /// Gets the value of IndexingEnabled
    pub fn get_indexing_enabled(&self) -> Option<&bool> {
        self.indexing_enabled.as_ref()
    }

    /// Sets the value of Label
    pub fn set_label(&mut self, value: String) {
        self.label = Some(value);
    }

    /// Gets the value of Label
    pub fn get_label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    /// Sets the value of MaximumFileNameLength
    pub fn set_maximum_file_name_length(&mut self, value: u32) {
        self.maximum_file_name_length = Some(value);
    }

    /// Gets the value of MaximumFileNameLength
    pub fn get_maximum_file_name_length(&self) -> Option<&u32> {
        self.maximum_file_name_length.as_ref()
    }

    /// Sets the value of PageFilePresent
    pub fn set_page_file_present(&mut self, value: bool) {
        self.page_file_present = Some(value);
    }

    /// Gets the value of PageFilePresent
    pub fn get_page_file_present(&self) -> Option<&bool> {
        self.page_file_present.as_ref()
    }

    /// Sets the value of QuotasEnabled
    pub fn set_quotas_enabled(&mut self, value: bool) {
        self.quotas_enabled = Some(value);
    }

    /// Gets the value of QuotasEnabled
    pub fn get_quotas_enabled(&self) -> Option<&bool> {
        self.quotas_enabled.as_ref()
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

    /// Sets the value of SerialNumber
    pub fn set_serial_number(&mut self, value: u32) {
        self.serial_number = Some(value);
    }

    /// Gets the value of SerialNumber
    pub fn get_serial_number(&self) -> Option<&u32> {
        self.serial_number.as_ref()
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

    /// Sets the value of SystemVolume
    pub fn set_system_volume(&mut self, value: bool) {
        self.system_volume = Some(value);
    }

    /// Gets the value of SystemVolume
    pub fn get_system_volume(&self) -> Option<&bool> {
        self.system_volume.as_ref()
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

    /// * `volume` -  (String[])

    /// * `return_value` -  (u32)
    pub fn schedule_auto_chk(&self, volume: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Volume".to_string(), value: volume.into() });
        self.invoke_method("ScheduleAutoChk", &args)

    }


/// 

    /// * `volume` -  (String[])

    /// * `return_value` -  (u32)
    pub fn exclude_from_auto_chk(&self, volume: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Volume".to_string(), value: volume.into() });
        self.invoke_method("ExcludeFromAutoChk", &args)

    }


/// 

    /// * `cluster_size` -  (u32)
    /// * `enable_compression` -  (bool)
    /// * `file_system` -  (String)
    /// * `label` -  (String)
    /// * `quick_format` -  (bool)
    /// * `version` -  (u32)

    /// * `return_value` -  (u32)
    pub fn format(&self, file_system: &String, quick_format: bool, cluster_size: u32, label: &String, enable_compression: bool, version: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "QuickFormat".to_string(), value: quick_format.into() });
        args.push(MethodParameter { name: "ClusterSize".to_string(), value: cluster_size.into() });
        args.push(MethodParameter { name: "Label".to_string(), value: label.into() });
        args.push(MethodParameter { name: "EnableCompression".to_string(), value: enable_compression.into() });
        args.push(MethodParameter { name: "Version".to_string(), value: version.into() });
        self.invoke_method("Format", &args)

    }


/// 

    /// * `force` -  (bool)

    /// * `defrag_analysis` -  (serde_json::Value)
    /// * `return_value` -  (u32)
    pub fn defrag(&self, force: bool, defrag_analysis: &mut serde_json::Value) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });

        let result = self.invoke_method("Defrag", &args)?;
        let defrag_analysis = result.get_value("DefragAnalysis")?;
        Ok(result.return_value)

    }


/// 

    /// * `defrag_analysis` -  (serde_json::Value)
    /// * `defrag_recommended` -  (bool)
    /// * `return_value` -  (u32)
    pub fn defrag_analysis(&self, defrag_recommended: &mut bool, defrag_analysis: &mut serde_json::Value) -> Result<(), WmiError> {

        let result = self.invoke_method("DefragAnalysis", &[])?;
        let defrag_analysis = result.get_value("DefragAnalysis")?;
        let defrag_recommended = result.get_value("DefragRecommended")?;
        Ok(result.return_value)

    }


/// 

    /// * `directory` -  (String)

    /// * `return_value` -  (u32)
    pub fn add_mount_point(&self, directory: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Directory".to_string(), value: directory.into() });
        self.invoke_method("AddMountPoint", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn mount(&self) -> Result<(), WmiError> {
        self.invoke_method("Mount", &[])

    }


/// 

    /// * `force` -  (bool)
    /// * `permanent` -  (bool)

    /// * `return_value` -  (u32)
    pub fn dismount(&self, force: bool, permanent: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Permanent".to_string(), value: permanent.into() });
        self.invoke_method("Dismount", &args)

    }

}

