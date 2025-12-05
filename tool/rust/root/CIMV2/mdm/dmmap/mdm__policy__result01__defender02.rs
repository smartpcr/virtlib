// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Defender02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Defender02 {

/// 
    #[serde(rename = "AllowArchiveScanning")]
    pub allow_archive_scanning: Option<i32>,

/// 
    #[serde(rename = "AllowBehaviorMonitoring")]
    pub allow_behavior_monitoring: Option<i32>,

/// 
    #[serde(rename = "AllowCloudProtection")]
    pub allow_cloud_protection: Option<i32>,

/// 
    #[serde(rename = "AllowEmailScanning")]
    pub allow_email_scanning: Option<i32>,

/// 
    #[serde(rename = "AllowFullScanOnMappedNetworkDrives")]
    pub allow_full_scan_on_mapped_network_drives: Option<i32>,

/// 
    #[serde(rename = "AllowFullScanRemovableDriveScanning")]
    pub allow_full_scan_removable_drive_scanning: Option<i32>,

/// 
    #[serde(rename = "AllowIntrusionPreventionSystem")]
    pub allow_intrusion_prevention_system: Option<i32>,

/// 
    #[serde(rename = "AllowIOAVProtection")]
    pub allow_ioavprotection: Option<i32>,

/// 
    #[serde(rename = "AllowOnAccessProtection")]
    pub allow_on_access_protection: Option<i32>,

/// 
    #[serde(rename = "AllowRealtimeMonitoring")]
    pub allow_realtime_monitoring: Option<i32>,

/// 
    #[serde(rename = "AllowScanningNetworkFiles")]
    pub allow_scanning_network_files: Option<i32>,

/// 
    #[serde(rename = "AllowScriptScanning")]
    pub allow_script_scanning: Option<i32>,

/// 
    #[serde(rename = "AllowUserUIAccess")]
    pub allow_user_uiaccess: Option<i32>,

/// 
    #[serde(rename = "AttackSurfaceReductionOnlyExclusions")]
    pub attack_surface_reduction_only_exclusions: Option<String>,

/// 
    #[serde(rename = "AttackSurfaceReductionRules")]
    pub attack_surface_reduction_rules: Option<String>,

/// 
    #[serde(rename = "AvgCPULoadFactor")]
    pub avg_cpuload_factor: Option<i32>,

/// 
    #[serde(rename = "CheckForSignaturesBeforeRunningScan")]
    pub check_for_signatures_before_running_scan: Option<i32>,

/// 
    #[serde(rename = "CloudBlockLevel")]
    pub cloud_block_level: Option<i32>,

/// 
    #[serde(rename = "CloudExtendedTimeout")]
    pub cloud_extended_timeout: Option<i32>,

/// 
    #[serde(rename = "ControlledFolderAccessAllowedApplications")]
    pub controlled_folder_access_allowed_applications: Option<String>,

/// 
    #[serde(rename = "ControlledFolderAccessProtectedFolders")]
    pub controlled_folder_access_protected_folders: Option<String>,

/// 
    #[serde(rename = "DaysToRetainCleanedMalware")]
    pub days_to_retain_cleaned_malware: Option<i32>,

/// 
    #[serde(rename = "DisableCatchupFullScan")]
    pub disable_catchup_full_scan: Option<i32>,

/// 
    #[serde(rename = "DisableCatchupQuickScan")]
    pub disable_catchup_quick_scan: Option<i32>,

/// 
    #[serde(rename = "EnableControlledFolderAccess")]
    pub enable_controlled_folder_access: Option<i32>,

/// 
    #[serde(rename = "EnableLowCPUPriority")]
    pub enable_low_cpupriority: Option<i32>,

/// 
    #[serde(rename = "EnableNetworkProtection")]
    pub enable_network_protection: Option<i32>,

/// 
    #[serde(rename = "ExcludedExtensions")]
    pub excluded_extensions: Option<String>,

/// 
    #[serde(rename = "ExcludedPaths")]
    pub excluded_paths: Option<String>,

/// 
    #[serde(rename = "ExcludedProcesses")]
    pub excluded_processes: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PUAProtection")]
    pub puaprotection: Option<i32>,

/// 
    #[serde(rename = "RealTimeScanDirection")]
    pub real_time_scan_direction: Option<i32>,

/// 
    #[serde(rename = "ScanParameter")]
    pub scan_parameter: Option<i32>,

/// 
    #[serde(rename = "ScheduleQuickScanTime")]
    pub schedule_quick_scan_time: Option<i32>,

/// 
    #[serde(rename = "ScheduleScanDay")]
    pub schedule_scan_day: Option<i32>,

/// 
    #[serde(rename = "ScheduleScanTime")]
    pub schedule_scan_time: Option<i32>,

/// 
    #[serde(rename = "SecurityIntelligenceLocation")]
    pub security_intelligence_location: Option<String>,

/// 
    #[serde(rename = "SignatureUpdateFallbackOrder")]
    pub signature_update_fallback_order: Option<String>,

/// 
    #[serde(rename = "SignatureUpdateFileSharesSources")]
    pub signature_update_file_shares_sources: Option<String>,

/// 
    #[serde(rename = "SignatureUpdateInterval")]
    pub signature_update_interval: Option<i32>,

/// 
    #[serde(rename = "SubmitSamplesConsent")]
    pub submit_samples_consent: Option<i32>,

/// 
    #[serde(rename = "ThreatSeverityDefaultAction")]
    pub threat_severity_default_action: Option<String>,
}

impl MDM_Policy_Result01_Defender02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_archive_scanning: None,
            allow_behavior_monitoring: None,
            allow_cloud_protection: None,
            allow_email_scanning: None,
            allow_full_scan_on_mapped_network_drives: None,
            allow_full_scan_removable_drive_scanning: None,
            allow_intrusion_prevention_system: None,
            allow_ioavprotection: None,
            allow_on_access_protection: None,
            allow_realtime_monitoring: None,
            allow_scanning_network_files: None,
            allow_script_scanning: None,
            allow_user_uiaccess: None,
            attack_surface_reduction_only_exclusions: None,
            attack_surface_reduction_rules: None,
            avg_cpuload_factor: None,
            check_for_signatures_before_running_scan: None,
            cloud_block_level: None,
            cloud_extended_timeout: None,
            controlled_folder_access_allowed_applications: None,
            controlled_folder_access_protected_folders: None,
            days_to_retain_cleaned_malware: None,
            disable_catchup_full_scan: None,
            disable_catchup_quick_scan: None,
            enable_controlled_folder_access: None,
            enable_low_cpupriority: None,
            enable_network_protection: None,
            excluded_extensions: None,
            excluded_paths: None,
            excluded_processes: None,
            instance_id: None,
            parent_id: None,
            puaprotection: None,
            real_time_scan_direction: None,
            scan_parameter: None,
            schedule_quick_scan_time: None,
            schedule_scan_day: None,
            schedule_scan_time: None,
            security_intelligence_location: None,
            signature_update_fallback_order: None,
            signature_update_file_shares_sources: None,
            signature_update_interval: None,
            submit_samples_consent: None,
            threat_severity_default_action: None,
        }
    }


    /// Sets the value of AllowArchiveScanning
    pub fn set_allow_archive_scanning(&mut self, value: i32) {
        self.allow_archive_scanning = Some(value);
    }

    /// Gets the value of AllowArchiveScanning
    pub fn get_allow_archive_scanning(&self) -> Option<&i32> {
        self.allow_archive_scanning.as_ref()
    }

    /// Sets the value of AllowBehaviorMonitoring
    pub fn set_allow_behavior_monitoring(&mut self, value: i32) {
        self.allow_behavior_monitoring = Some(value);
    }

    /// Gets the value of AllowBehaviorMonitoring
    pub fn get_allow_behavior_monitoring(&self) -> Option<&i32> {
        self.allow_behavior_monitoring.as_ref()
    }

    /// Sets the value of AllowCloudProtection
    pub fn set_allow_cloud_protection(&mut self, value: i32) {
        self.allow_cloud_protection = Some(value);
    }

    /// Gets the value of AllowCloudProtection
    pub fn get_allow_cloud_protection(&self) -> Option<&i32> {
        self.allow_cloud_protection.as_ref()
    }

    /// Sets the value of AllowEmailScanning
    pub fn set_allow_email_scanning(&mut self, value: i32) {
        self.allow_email_scanning = Some(value);
    }

    /// Gets the value of AllowEmailScanning
    pub fn get_allow_email_scanning(&self) -> Option<&i32> {
        self.allow_email_scanning.as_ref()
    }

    /// Sets the value of AllowFullScanOnMappedNetworkDrives
    pub fn set_allow_full_scan_on_mapped_network_drives(&mut self, value: i32) {
        self.allow_full_scan_on_mapped_network_drives = Some(value);
    }

    /// Gets the value of AllowFullScanOnMappedNetworkDrives
    pub fn get_allow_full_scan_on_mapped_network_drives(&self) -> Option<&i32> {
        self.allow_full_scan_on_mapped_network_drives.as_ref()
    }

    /// Sets the value of AllowFullScanRemovableDriveScanning
    pub fn set_allow_full_scan_removable_drive_scanning(&mut self, value: i32) {
        self.allow_full_scan_removable_drive_scanning = Some(value);
    }

    /// Gets the value of AllowFullScanRemovableDriveScanning
    pub fn get_allow_full_scan_removable_drive_scanning(&self) -> Option<&i32> {
        self.allow_full_scan_removable_drive_scanning.as_ref()
    }

    /// Sets the value of AllowIntrusionPreventionSystem
    pub fn set_allow_intrusion_prevention_system(&mut self, value: i32) {
        self.allow_intrusion_prevention_system = Some(value);
    }

    /// Gets the value of AllowIntrusionPreventionSystem
    pub fn get_allow_intrusion_prevention_system(&self) -> Option<&i32> {
        self.allow_intrusion_prevention_system.as_ref()
    }

    /// Sets the value of AllowIOAVProtection
    pub fn set_allow_ioavprotection(&mut self, value: i32) {
        self.allow_ioavprotection = Some(value);
    }

    /// Gets the value of AllowIOAVProtection
    pub fn get_allow_ioavprotection(&self) -> Option<&i32> {
        self.allow_ioavprotection.as_ref()
    }

    /// Sets the value of AllowOnAccessProtection
    pub fn set_allow_on_access_protection(&mut self, value: i32) {
        self.allow_on_access_protection = Some(value);
    }

    /// Gets the value of AllowOnAccessProtection
    pub fn get_allow_on_access_protection(&self) -> Option<&i32> {
        self.allow_on_access_protection.as_ref()
    }

    /// Sets the value of AllowRealtimeMonitoring
    pub fn set_allow_realtime_monitoring(&mut self, value: i32) {
        self.allow_realtime_monitoring = Some(value);
    }

    /// Gets the value of AllowRealtimeMonitoring
    pub fn get_allow_realtime_monitoring(&self) -> Option<&i32> {
        self.allow_realtime_monitoring.as_ref()
    }

    /// Sets the value of AllowScanningNetworkFiles
    pub fn set_allow_scanning_network_files(&mut self, value: i32) {
        self.allow_scanning_network_files = Some(value);
    }

    /// Gets the value of AllowScanningNetworkFiles
    pub fn get_allow_scanning_network_files(&self) -> Option<&i32> {
        self.allow_scanning_network_files.as_ref()
    }

    /// Sets the value of AllowScriptScanning
    pub fn set_allow_script_scanning(&mut self, value: i32) {
        self.allow_script_scanning = Some(value);
    }

    /// Gets the value of AllowScriptScanning
    pub fn get_allow_script_scanning(&self) -> Option<&i32> {
        self.allow_script_scanning.as_ref()
    }

    /// Sets the value of AllowUserUIAccess
    pub fn set_allow_user_uiaccess(&mut self, value: i32) {
        self.allow_user_uiaccess = Some(value);
    }

    /// Gets the value of AllowUserUIAccess
    pub fn get_allow_user_uiaccess(&self) -> Option<&i32> {
        self.allow_user_uiaccess.as_ref()
    }

    /// Sets the value of AttackSurfaceReductionOnlyExclusions
    pub fn set_attack_surface_reduction_only_exclusions(&mut self, value: String) {
        self.attack_surface_reduction_only_exclusions = Some(value);
    }

    /// Gets the value of AttackSurfaceReductionOnlyExclusions
    pub fn get_attack_surface_reduction_only_exclusions(&self) -> Option<&String> {
        self.attack_surface_reduction_only_exclusions.as_ref()
    }

    /// Sets the value of AttackSurfaceReductionRules
    pub fn set_attack_surface_reduction_rules(&mut self, value: String) {
        self.attack_surface_reduction_rules = Some(value);
    }

    /// Gets the value of AttackSurfaceReductionRules
    pub fn get_attack_surface_reduction_rules(&self) -> Option<&String> {
        self.attack_surface_reduction_rules.as_ref()
    }

    /// Sets the value of AvgCPULoadFactor
    pub fn set_avg_cpuload_factor(&mut self, value: i32) {
        self.avg_cpuload_factor = Some(value);
    }

    /// Gets the value of AvgCPULoadFactor
    pub fn get_avg_cpuload_factor(&self) -> Option<&i32> {
        self.avg_cpuload_factor.as_ref()
    }

    /// Sets the value of CheckForSignaturesBeforeRunningScan
    pub fn set_check_for_signatures_before_running_scan(&mut self, value: i32) {
        self.check_for_signatures_before_running_scan = Some(value);
    }

    /// Gets the value of CheckForSignaturesBeforeRunningScan
    pub fn get_check_for_signatures_before_running_scan(&self) -> Option<&i32> {
        self.check_for_signatures_before_running_scan.as_ref()
    }

    /// Sets the value of CloudBlockLevel
    pub fn set_cloud_block_level(&mut self, value: i32) {
        self.cloud_block_level = Some(value);
    }

    /// Gets the value of CloudBlockLevel
    pub fn get_cloud_block_level(&self) -> Option<&i32> {
        self.cloud_block_level.as_ref()
    }

    /// Sets the value of CloudExtendedTimeout
    pub fn set_cloud_extended_timeout(&mut self, value: i32) {
        self.cloud_extended_timeout = Some(value);
    }

    /// Gets the value of CloudExtendedTimeout
    pub fn get_cloud_extended_timeout(&self) -> Option<&i32> {
        self.cloud_extended_timeout.as_ref()
    }

    /// Sets the value of ControlledFolderAccessAllowedApplications
    pub fn set_controlled_folder_access_allowed_applications(&mut self, value: String) {
        self.controlled_folder_access_allowed_applications = Some(value);
    }

    /// Gets the value of ControlledFolderAccessAllowedApplications
    pub fn get_controlled_folder_access_allowed_applications(&self) -> Option<&String> {
        self.controlled_folder_access_allowed_applications.as_ref()
    }

    /// Sets the value of ControlledFolderAccessProtectedFolders
    pub fn set_controlled_folder_access_protected_folders(&mut self, value: String) {
        self.controlled_folder_access_protected_folders = Some(value);
    }

    /// Gets the value of ControlledFolderAccessProtectedFolders
    pub fn get_controlled_folder_access_protected_folders(&self) -> Option<&String> {
        self.controlled_folder_access_protected_folders.as_ref()
    }

    /// Sets the value of DaysToRetainCleanedMalware
    pub fn set_days_to_retain_cleaned_malware(&mut self, value: i32) {
        self.days_to_retain_cleaned_malware = Some(value);
    }

    /// Gets the value of DaysToRetainCleanedMalware
    pub fn get_days_to_retain_cleaned_malware(&self) -> Option<&i32> {
        self.days_to_retain_cleaned_malware.as_ref()
    }

    /// Sets the value of DisableCatchupFullScan
    pub fn set_disable_catchup_full_scan(&mut self, value: i32) {
        self.disable_catchup_full_scan = Some(value);
    }

    /// Gets the value of DisableCatchupFullScan
    pub fn get_disable_catchup_full_scan(&self) -> Option<&i32> {
        self.disable_catchup_full_scan.as_ref()
    }

    /// Sets the value of DisableCatchupQuickScan
    pub fn set_disable_catchup_quick_scan(&mut self, value: i32) {
        self.disable_catchup_quick_scan = Some(value);
    }

    /// Gets the value of DisableCatchupQuickScan
    pub fn get_disable_catchup_quick_scan(&self) -> Option<&i32> {
        self.disable_catchup_quick_scan.as_ref()
    }

    /// Sets the value of EnableControlledFolderAccess
    pub fn set_enable_controlled_folder_access(&mut self, value: i32) {
        self.enable_controlled_folder_access = Some(value);
    }

    /// Gets the value of EnableControlledFolderAccess
    pub fn get_enable_controlled_folder_access(&self) -> Option<&i32> {
        self.enable_controlled_folder_access.as_ref()
    }

    /// Sets the value of EnableLowCPUPriority
    pub fn set_enable_low_cpupriority(&mut self, value: i32) {
        self.enable_low_cpupriority = Some(value);
    }

    /// Gets the value of EnableLowCPUPriority
    pub fn get_enable_low_cpupriority(&self) -> Option<&i32> {
        self.enable_low_cpupriority.as_ref()
    }

    /// Sets the value of EnableNetworkProtection
    pub fn set_enable_network_protection(&mut self, value: i32) {
        self.enable_network_protection = Some(value);
    }

    /// Gets the value of EnableNetworkProtection
    pub fn get_enable_network_protection(&self) -> Option<&i32> {
        self.enable_network_protection.as_ref()
    }

    /// Sets the value of ExcludedExtensions
    pub fn set_excluded_extensions(&mut self, value: String) {
        self.excluded_extensions = Some(value);
    }

    /// Gets the value of ExcludedExtensions
    pub fn get_excluded_extensions(&self) -> Option<&String> {
        self.excluded_extensions.as_ref()
    }

    /// Sets the value of ExcludedPaths
    pub fn set_excluded_paths(&mut self, value: String) {
        self.excluded_paths = Some(value);
    }

    /// Gets the value of ExcludedPaths
    pub fn get_excluded_paths(&self) -> Option<&String> {
        self.excluded_paths.as_ref()
    }

    /// Sets the value of ExcludedProcesses
    pub fn set_excluded_processes(&mut self, value: String) {
        self.excluded_processes = Some(value);
    }

    /// Gets the value of ExcludedProcesses
    pub fn get_excluded_processes(&self) -> Option<&String> {
        self.excluded_processes.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PUAProtection
    pub fn set_puaprotection(&mut self, value: i32) {
        self.puaprotection = Some(value);
    }

    /// Gets the value of PUAProtection
    pub fn get_puaprotection(&self) -> Option<&i32> {
        self.puaprotection.as_ref()
    }

    /// Sets the value of RealTimeScanDirection
    pub fn set_real_time_scan_direction(&mut self, value: i32) {
        self.real_time_scan_direction = Some(value);
    }

    /// Gets the value of RealTimeScanDirection
    pub fn get_real_time_scan_direction(&self) -> Option<&i32> {
        self.real_time_scan_direction.as_ref()
    }

    /// Sets the value of ScanParameter
    pub fn set_scan_parameter(&mut self, value: i32) {
        self.scan_parameter = Some(value);
    }

    /// Gets the value of ScanParameter
    pub fn get_scan_parameter(&self) -> Option<&i32> {
        self.scan_parameter.as_ref()
    }

    /// Sets the value of ScheduleQuickScanTime
    pub fn set_schedule_quick_scan_time(&mut self, value: i32) {
        self.schedule_quick_scan_time = Some(value);
    }

    /// Gets the value of ScheduleQuickScanTime
    pub fn get_schedule_quick_scan_time(&self) -> Option<&i32> {
        self.schedule_quick_scan_time.as_ref()
    }

    /// Sets the value of ScheduleScanDay
    pub fn set_schedule_scan_day(&mut self, value: i32) {
        self.schedule_scan_day = Some(value);
    }

    /// Gets the value of ScheduleScanDay
    pub fn get_schedule_scan_day(&self) -> Option<&i32> {
        self.schedule_scan_day.as_ref()
    }

    /// Sets the value of ScheduleScanTime
    pub fn set_schedule_scan_time(&mut self, value: i32) {
        self.schedule_scan_time = Some(value);
    }

    /// Gets the value of ScheduleScanTime
    pub fn get_schedule_scan_time(&self) -> Option<&i32> {
        self.schedule_scan_time.as_ref()
    }

    /// Sets the value of SecurityIntelligenceLocation
    pub fn set_security_intelligence_location(&mut self, value: String) {
        self.security_intelligence_location = Some(value);
    }

    /// Gets the value of SecurityIntelligenceLocation
    pub fn get_security_intelligence_location(&self) -> Option<&String> {
        self.security_intelligence_location.as_ref()
    }

    /// Sets the value of SignatureUpdateFallbackOrder
    pub fn set_signature_update_fallback_order(&mut self, value: String) {
        self.signature_update_fallback_order = Some(value);
    }

    /// Gets the value of SignatureUpdateFallbackOrder
    pub fn get_signature_update_fallback_order(&self) -> Option<&String> {
        self.signature_update_fallback_order.as_ref()
    }

    /// Sets the value of SignatureUpdateFileSharesSources
    pub fn set_signature_update_file_shares_sources(&mut self, value: String) {
        self.signature_update_file_shares_sources = Some(value);
    }

    /// Gets the value of SignatureUpdateFileSharesSources
    pub fn get_signature_update_file_shares_sources(&self) -> Option<&String> {
        self.signature_update_file_shares_sources.as_ref()
    }

    /// Sets the value of SignatureUpdateInterval
    pub fn set_signature_update_interval(&mut self, value: i32) {
        self.signature_update_interval = Some(value);
    }

    /// Gets the value of SignatureUpdateInterval
    pub fn get_signature_update_interval(&self) -> Option<&i32> {
        self.signature_update_interval.as_ref()
    }

    /// Sets the value of SubmitSamplesConsent
    pub fn set_submit_samples_consent(&mut self, value: i32) {
        self.submit_samples_consent = Some(value);
    }

    /// Gets the value of SubmitSamplesConsent
    pub fn get_submit_samples_consent(&self) -> Option<&i32> {
        self.submit_samples_consent.as_ref()
    }

    /// Sets the value of ThreatSeverityDefaultAction
    pub fn set_threat_severity_default_action(&mut self, value: String) {
        self.threat_severity_default_action = Some(value);
    }

    /// Gets the value of ThreatSeverityDefaultAction
    pub fn get_threat_severity_default_action(&self) -> Option<&String> {
        self.threat_severity_default_action.as_ref()
    }
}

