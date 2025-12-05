// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_Volume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_Volume {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "AllocationUnitSize")]
    pub allocation_unit_size: Option<u32>,

/// 
    #[serde(rename = "DedupMode")]
    pub dedup_mode: Option<u32>,

/// 
    #[serde(rename = "DriveLetter")]
    pub drive_letter: Option<char>,

/// 
    #[serde(rename = "DriveType")]
    pub drive_type: Option<u32>,

/// 
    #[serde(rename = "FileSystem")]
    pub file_system: Option<String>,

/// 
    #[serde(rename = "FileSystemLabel")]
    pub file_system_label: Option<String>,

/// 
    #[serde(rename = "FileSystemType")]
    pub file_system_type: Option<u16>,

/// 
    #[serde(rename = "HealthStatus")]
    pub health_status: Option<u16>,

/// 
    #[serde(rename = "OperationalStatus")]
    pub operational_status: Vec<u16>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "ReFSDedupMode")]
    pub re_fsdedup_mode: Option<u32>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "SizeRemaining")]
    pub size_remaining: Option<u64>,
}

impl MSFT_Volume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            allocation_unit_size: None,
            dedup_mode: None,
            drive_letter: None,
            drive_type: None,
            file_system: None,
            file_system_label: None,
            file_system_type: None,
            health_status: None,
            operational_status: Vec::new(),
            path: None,
            re_fsdedup_mode: None,
            size: None,
            size_remaining: None,
        }
    }


    /// Sets the value of AllocationUnitSize
    pub fn set_allocation_unit_size(&mut self, value: u32) {
        self.allocation_unit_size = Some(value);
    }

    /// Gets the value of AllocationUnitSize
    pub fn get_allocation_unit_size(&self) -> Option<&u32> {
        self.allocation_unit_size.as_ref()
    }

    /// Sets the value of DedupMode
    pub fn set_dedup_mode(&mut self, value: u32) {
        self.dedup_mode = Some(value);
    }

    /// Gets the value of DedupMode
    pub fn get_dedup_mode(&self) -> Option<&u32> {
        self.dedup_mode.as_ref()
    }

    /// Sets the value of DriveLetter
    pub fn set_drive_letter(&mut self, value: char) {
        self.drive_letter = Some(value);
    }

    /// Gets the value of DriveLetter
    pub fn get_drive_letter(&self) -> Option<&char> {
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

    /// Sets the value of FileSystemLabel
    pub fn set_file_system_label(&mut self, value: String) {
        self.file_system_label = Some(value);
    }

    /// Gets the value of FileSystemLabel
    pub fn get_file_system_label(&self) -> Option<&String> {
        self.file_system_label.as_ref()
    }

    /// Sets the value of FileSystemType
    pub fn set_file_system_type(&mut self, value: u16) {
        self.file_system_type = Some(value);
    }

    /// Gets the value of FileSystemType
    pub fn get_file_system_type(&self) -> Option<&u16> {
        self.file_system_type.as_ref()
    }

    /// Sets the value of HealthStatus
    pub fn set_health_status(&mut self, value: u16) {
        self.health_status = Some(value);
    }

    /// Gets the value of HealthStatus
    pub fn get_health_status(&self) -> Option<&u16> {
        self.health_status.as_ref()
    }

    /// Sets the value of OperationalStatus
    pub fn set_operational_status(&mut self, value: Vec<u16>) {
        self.operational_status = value;
    }

    /// Gets the value of OperationalStatus
    pub fn get_operational_status(&self) -> &Vec<u16> {
        &self.operational_status
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of ReFSDedupMode
    pub fn set_re_fsdedup_mode(&mut self, value: u32) {
        self.re_fsdedup_mode = Some(value);
    }

    /// Gets the value of ReFSDedupMode
    pub fn get_re_fsdedup_mode(&self) -> Option<&u32> {
        self.re_fsdedup_mode.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of SizeRemaining
    pub fn set_size_remaining(&mut self, value: u64) {
        self.size_remaining = Some(value);
    }

    /// Gets the value of SizeRemaining
    pub fn get_size_remaining(&self) -> Option<&u64> {
        self.size_remaining.as_ref()
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

    /// * `allocation_unit_size` -  (u32)
    /// * `compress` -  (bool)
    /// * `dev_drive` -  (bool)
    /// * `disable_heat_gathering` -  (bool)
    /// * `file_system` -  (String)
    /// * `file_system_label` -  (String)
    /// * `force` -  (bool)
    /// * `full` -  (bool)
    /// * `is_dax` -  (bool)
    /// * `no_trim` -  (bool)
    /// * `run_as_job` -  (bool)
    /// * `set_integrity_streams` -  (bool)
    /// * `sha256_checksums` -  (bool)
    /// * `short_file_name_support` -  (bool)
    /// * `use_large_frs` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `formatted_volume` -  (MSFT_Volume)
    /// * `return_value` -  (u32)
    pub fn format(&self, file_system: &String, file_system_label: &String, allocation_unit_size: u32, full: bool, force: bool, compress: bool, short_file_name_support: bool, set_integrity_streams: bool, use_large_frs: bool, disable_heat_gathering: bool, is_dax: bool, no_trim: bool, sha256_checksums: bool, dev_drive: bool, run_as_job: bool, formatted_volume: &mut MSFT_Volume, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });
        args.push(MethodParameter { name: "FileSystemLabel".to_string(), value: file_system_label.into() });
        args.push(MethodParameter { name: "AllocationUnitSize".to_string(), value: allocation_unit_size.into() });
        args.push(MethodParameter { name: "Full".to_string(), value: full.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Compress".to_string(), value: compress.into() });
        args.push(MethodParameter { name: "ShortFileNameSupport".to_string(), value: short_file_name_support.into() });
        args.push(MethodParameter { name: "SetIntegrityStreams".to_string(), value: set_integrity_streams.into() });
        args.push(MethodParameter { name: "UseLargeFRS".to_string(), value: use_large_frs.into() });
        args.push(MethodParameter { name: "DisableHeatGathering".to_string(), value: disable_heat_gathering.into() });
        args.push(MethodParameter { name: "IsDAX".to_string(), value: is_dax.into() });
        args.push(MethodParameter { name: "NoTrim".to_string(), value: no_trim.into() });
        args.push(MethodParameter { name: "SHA256Checksums".to_string(), value: sha256_checksums.into() });
        args.push(MethodParameter { name: "DevDrive".to_string(), value: dev_drive.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Format", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let formatted_volume = result.get_value("FormattedVolume")?;
        Ok(result.return_value)

    }


/// 

    /// * `detect_leaks` -  (u32)
    /// * `directory_ids` -  (u64[])
    /// * `offline_scan_and_fix` -  (bool)
    /// * `run_as_job` -  (bool)
    /// * `salvage` -  (u32)
    /// * `scan` -  (bool)
    /// * `scratch_dir` -  (String)
    /// * `scratch_file` -  (String)
    /// * `spot_fix` -  (bool)
    /// * `target_dir` -  (String)
    /// * `target_file` -  (String)
    /// * `threads` -  (u32)
    /// * `triage` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `output` -  (u32)
    /// * `return_value` -  (u32)
    pub fn repair(&self, offline_scan_and_fix: bool, scan: bool, spot_fix: bool, detect_leaks: u32, scratch_file: &String, threads: u32, triage: bool, directory_ids: &Vec<u64>, salvage: u32, scratch_dir: &String, target_file: &String, target_dir: &String, output: &mut u32, run_as_job: Option<bool>, created_storage_job: &mut Option<MSFT_StorageJob>, extended_status: &mut Option<MSFT_StorageExtendedStatus>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OfflineScanAndFix".to_string(), value: offline_scan_and_fix.into() });
        args.push(MethodParameter { name: "Scan".to_string(), value: scan.into() });
        args.push(MethodParameter { name: "SpotFix".to_string(), value: spot_fix.into() });
        args.push(MethodParameter { name: "DetectLeaks".to_string(), value: detect_leaks.into() });
        args.push(MethodParameter { name: "ScratchFile".to_string(), value: scratch_file.into() });
        args.push(MethodParameter { name: "Threads".to_string(), value: threads.into() });
        args.push(MethodParameter { name: "Triage".to_string(), value: triage.into() });
        args.push(MethodParameter { name: "DirectoryIds".to_string(), value: directory_ids.into() });
        args.push(MethodParameter { name: "Salvage".to_string(), value: salvage.into() });
        args.push(MethodParameter { name: "ScratchDir".to_string(), value: scratch_dir.into() });
        args.push(MethodParameter { name: "TargetFile".to_string(), value: target_file.into() });
        args.push(MethodParameter { name: "TargetDir".to_string(), value: target_dir.into() });
        if let Some(val) = run_as_job {
            args.push(MethodParameter { name: "RunAsJob".to_string(), value: val.into() });
        }

        let result = self.invoke_method("Repair", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let output = result.get_value("Output")?;
        Ok(result.return_value)

    }


/// 

    /// * `analyze` -  (bool)
    /// * `defrag` -  (bool)
    /// * `normal_priority` -  (bool)
    /// * `re_trim` -  (bool)
    /// * `run_as_job` -  (bool)
    /// * `slab_consolidate` -  (bool)
    /// * `tier_optimize` -  (bool)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn optimize(&self, re_trim: bool, analyze: bool, defrag: bool, slab_consolidate: bool, tier_optimize: bool, normal_priority: bool, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReTrim".to_string(), value: re_trim.into() });
        args.push(MethodParameter { name: "Analyze".to_string(), value: analyze.into() });
        args.push(MethodParameter { name: "Defrag".to_string(), value: defrag.into() });
        args.push(MethodParameter { name: "SlabConsolidate".to_string(), value: slab_consolidate.into() });
        args.push(MethodParameter { name: "TierOptimize".to_string(), value: tier_optimize.into() });
        args.push(MethodParameter { name: "NormalPriority".to_string(), value: normal_priority.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Optimize", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `file_system_label` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_file_system_label(&self, file_system_label: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileSystemLabel".to_string(), value: file_system_label.into() });

        let result = self.invoke_method("SetFileSystemLabel", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_file_systems` -  (String[])
    pub fn get_supported_file_systems(&self, supported_file_systems: &mut Vec<String>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSupportedFileSystems", &[])?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_file_systems = result.get_value("SupportedFileSystems")?;
        Ok(result.return_value)

    }


/// 

    /// * `file_system` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `supported_cluster_sizes` -  (u32[])
    pub fn get_supported_cluster_sizes(&self, file_system: &String, supported_cluster_sizes: &mut Vec<u32>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FileSystem".to_string(), value: file_system.into() });

        let result = self.invoke_method("GetSupportedClusterSizes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let supported_cluster_sizes = result.get_value("SupportedClusterSizes")?;
        Ok(result.return_value)

    }


/// 

    /// * `corruption_count` -  (u32)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_corruption_count(&self, corruption_count: &mut u32, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetCorruptionCount", &[])?;
        let corruption_count = result.get_value("CorruptionCount")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    /// * `volume_scrub_enabled` -  (bool)
    pub fn get_attributes(&self, volume_scrub_enabled: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("GetAttributes", &[])?;
        let volume_scrub_enabled = result.get_value("VolumeScrubEnabled")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable_volume_scrub` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_attributes(&self, enable_volume_scrub: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EnableVolumeScrub".to_string(), value: enable_volume_scrub.into() });

        let result = self.invoke_method("SetAttributes", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn flush(&self) -> Result<(), WmiError> {
        self.invoke_method("Flush", &[])

    }


/// 

    /// * `run_as_job` -  (bool)
    /// * `size` -  (u64)

    /// * `created_storage_job` -  (MSFT_StorageJob)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn resize(&self, size: u64, run_as_job: bool, created_storage_job: &mut MSFT_StorageJob, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Size".to_string(), value: size.into() });
        args.push(MethodParameter { name: "RunAsJob".to_string(), value: run_as_job.into() });

        let result = self.invoke_method("Resize", &args)?;
        let created_storage_job = result.get_value("CreatedStorageJob")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `diagnose_results` -  (MSFT_StorageDiagnoseResult[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn diagnose(&self, diagnose_results: &mut Vec<MSFT_StorageDiagnoseResult>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("Diagnose", &[])?;
        let diagnose_results = result.get_value("DiagnoseResults")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `dedup_mode` -  (u32)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_dedup_mode(&self, dedup_mode: u32, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DedupMode".to_string(), value: dedup_mode.into() });

        let result = self.invoke_method("SetDedupMode", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `dedup_properties` -  (MSFT_DedupProperties)
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_dedup_properties(&self, dedup_properties: &mut MSFT_DedupProperties, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetDedupProperties", &[])?;
        let dedup_properties = result.get_value("DedupProperties")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `action_results` -  (MSFT_HealthAction[])
    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn get_actions(&self, action_results: &mut Vec<MSFT_HealthAction>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {

        let result = self.invoke_method("GetActions", &[])?;
        let action_results = result.get_value("ActionResults")?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

