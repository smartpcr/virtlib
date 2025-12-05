// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpPreference struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpPreference {

/// 
    #[serde(rename = "AllowDatagramProcessingOnWinServer")]
    pub allow_datagram_processing_on_win_server: Option<bool>,

/// 
    #[serde(rename = "AllowNetworkProtectionDownLevel")]
    pub allow_network_protection_down_level: Option<bool>,

/// 
    #[serde(rename = "AllowNetworkProtectionOnWinServer")]
    pub allow_network_protection_on_win_server: Option<bool>,

/// 
    #[serde(rename = "AllowSwitchToAsyncInspection")]
    pub allow_switch_to_async_inspection: Option<bool>,

/// 
    #[serde(rename = "ApplyDisableNetworkScanningToIOAV")]
    pub apply_disable_network_scanning_to_ioav: Option<bool>,

/// 
    #[serde(rename = "AttackSurfaceReductionOnlyExclusions")]
    pub attack_surface_reduction_only_exclusions: Vec<String>,

/// 
    #[serde(rename = "AttackSurfaceReductionRules_Actions")]
    pub attack_surface_reduction_rules__actions: Vec<u8>,

/// 
    #[serde(rename = "AttackSurfaceReductionRules_Ids")]
    pub attack_surface_reduction_rules__ids: Vec<String>,

/// 
    #[serde(rename = "AttackSurfaceReductionRules_RuleSpecificExclusions")]
    pub attack_surface_reduction_rules__rule_specific_exclusions: Vec<String>,

/// 
    #[serde(rename = "AttackSurfaceReductionRules_RuleSpecificExclusions_Id")]
    pub attack_surface_reduction_rules__rule_specific_exclusions__id: Vec<String>,

/// 
    #[serde(rename = "BruteForceProtectionAggressiveness")]
    pub brute_force_protection_aggressiveness: Option<u8>,

/// 
    #[serde(rename = "BruteForceProtectionConfiguredState")]
    pub brute_force_protection_configured_state: Option<u8>,

/// 
    #[serde(rename = "BruteForceProtectionExclusions")]
    pub brute_force_protection_exclusions: Vec<String>,

/// 
    #[serde(rename = "BruteForceProtectionLocalNetworkBlocking")]
    pub brute_force_protection_local_network_blocking: Option<bool>,

/// 
    #[serde(rename = "BruteForceProtectionMaxBlockTime")]
    pub brute_force_protection_max_block_time: Option<u32>,

/// 
    #[serde(rename = "BruteForceProtectionSkipLearningPeriod")]
    pub brute_force_protection_skip_learning_period: Option<bool>,

/// 
    #[serde(rename = "CheckForSignaturesBeforeRunningScan")]
    pub check_for_signatures_before_running_scan: Option<bool>,

/// 
    #[serde(rename = "CloudBlockLevel")]
    pub cloud_block_level: Option<u8>,

/// 
    #[serde(rename = "CloudExtendedTimeout")]
    pub cloud_extended_timeout: Option<u32>,

/// 
    #[serde(rename = "ComputerID")]
    pub computer_id: Option<String>,

/// 
    #[serde(rename = "ControlledFolderAccessAllowedApplications")]
    pub controlled_folder_access_allowed_applications: Vec<String>,

/// 
    #[serde(rename = "ControlledFolderAccessDefaultProtectedFolders")]
    pub controlled_folder_access_default_protected_folders: Vec<String>,

/// 
    #[serde(rename = "ControlledFolderAccessProtectedFolders")]
    pub controlled_folder_access_protected_folders: Vec<String>,

/// 
    #[serde(rename = "DefinitionUpdatesChannel")]
    pub definition_updates_channel: Option<u8>,

/// 
    #[serde(rename = "DisableArchiveScanning")]
    pub disable_archive_scanning: Option<bool>,

/// 
    #[serde(rename = "DisableAutoExclusions")]
    pub disable_auto_exclusions: Option<bool>,

/// 
    #[serde(rename = "DisableBehaviorMonitoring")]
    pub disable_behavior_monitoring: Option<bool>,

/// 
    #[serde(rename = "DisableBlockAtFirstSeen")]
    pub disable_block_at_first_seen: Option<bool>,

/// 
    #[serde(rename = "DisableCacheMaintenance")]
    pub disable_cache_maintenance: Option<bool>,

/// 
    #[serde(rename = "DisableCatchupFullScan")]
    pub disable_catchup_full_scan: Option<bool>,

/// 
    #[serde(rename = "DisableCatchupQuickScan")]
    pub disable_catchup_quick_scan: Option<bool>,

/// 
    #[serde(rename = "DisableCoreServiceECSIntegration")]
    pub disable_core_service_ecsintegration: Option<bool>,

/// 
    #[serde(rename = "DisableCoreServiceTelemetry")]
    pub disable_core_service_telemetry: Option<bool>,

/// 
    #[serde(rename = "DisableCpuThrottleOnIdleScans")]
    pub disable_cpu_throttle_on_idle_scans: Option<bool>,

/// 
    #[serde(rename = "DisableDatagramProcessing")]
    pub disable_datagram_processing: Option<bool>,

/// 
    #[serde(rename = "DisableDnsOverTcpParsing")]
    pub disable_dns_over_tcp_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableDnsParsing")]
    pub disable_dns_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableEmailScanning")]
    pub disable_email_scanning: Option<bool>,

/// 
    #[serde(rename = "DisableFtpParsing")]
    pub disable_ftp_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableGradualRelease")]
    pub disable_gradual_release: Option<bool>,

/// 
    #[serde(rename = "DisableHttpParsing")]
    pub disable_http_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableInboundConnectionFiltering")]
    pub disable_inbound_connection_filtering: Option<bool>,

/// 
    #[serde(rename = "DisableIOAVProtection")]
    pub disable_ioavprotection: Option<bool>,

/// 
    #[serde(rename = "DisableNetworkProtectionPerfTelemetry")]
    pub disable_network_protection_perf_telemetry: Option<bool>,

/// 
    #[serde(rename = "DisablePrivacyMode")]
    pub disable_privacy_mode: Option<bool>,

/// 
    #[serde(rename = "DisableQuicParsing")]
    pub disable_quic_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableRdpParsing")]
    pub disable_rdp_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableRealtimeMonitoring")]
    pub disable_realtime_monitoring: Option<bool>,

/// 
    #[serde(rename = "DisableRemovableDriveScanning")]
    pub disable_removable_drive_scanning: Option<bool>,

/// 
    #[serde(rename = "DisableRestorePoint")]
    pub disable_restore_point: Option<bool>,

/// 
    #[serde(rename = "DisableScanningMappedNetworkDrivesForFullScan")]
    pub disable_scanning_mapped_network_drives_for_full_scan: Option<bool>,

/// 
    #[serde(rename = "DisableScanningNetworkFiles")]
    pub disable_scanning_network_files: Option<bool>,

/// 
    #[serde(rename = "DisableScriptScanning")]
    pub disable_script_scanning: Option<bool>,

/// 
    #[serde(rename = "DisableSmtpParsing")]
    pub disable_smtp_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableSshParsing")]
    pub disable_ssh_parsing: Option<bool>,

/// 
    #[serde(rename = "DisableTamperProtection")]
    pub disable_tamper_protection: Option<bool>,

/// 
    #[serde(rename = "DisableTlsParsing")]
    pub disable_tls_parsing: Option<bool>,

/// 
    #[serde(rename = "EnableControlledFolderAccess")]
    pub enable_controlled_folder_access: Option<u8>,

/// 
    #[serde(rename = "EnableConvertWarnToBlock")]
    pub enable_convert_warn_to_block: Option<bool>,

/// 
    #[serde(rename = "EnableDnsSinkhole")]
    pub enable_dns_sinkhole: Option<bool>,

/// 
    #[serde(rename = "EnableFileHashComputation")]
    pub enable_file_hash_computation: Option<bool>,

/// 
    #[serde(rename = "EnableFullScanOnBatteryPower")]
    pub enable_full_scan_on_battery_power: Option<bool>,

/// 
    #[serde(rename = "EnableLowCpuPriority")]
    pub enable_low_cpu_priority: Option<bool>,

/// 
    #[serde(rename = "EnableNetworkProtection")]
    pub enable_network_protection: Option<u8>,

/// 
    #[serde(rename = "EnableUdpReceiveOffload")]
    pub enable_udp_receive_offload: Option<bool>,

/// 
    #[serde(rename = "EnableUdpSegmentationOffload")]
    pub enable_udp_segmentation_offload: Option<bool>,

/// 
    #[serde(rename = "EngineUpdatesChannel")]
    pub engine_updates_channel: Option<u8>,

/// 
    #[serde(rename = "ExclusionExtension")]
    pub exclusion_extension: Vec<String>,

/// 
    #[serde(rename = "ExclusionIpAddress")]
    pub exclusion_ip_address: Vec<String>,

/// 
    #[serde(rename = "ExclusionPath")]
    pub exclusion_path: Vec<String>,

/// 
    #[serde(rename = "ExclusionProcess")]
    pub exclusion_process: Vec<String>,

/// 
    #[serde(rename = "ForceUseProxyOnly")]
    pub force_use_proxy_only: Option<bool>,

/// 
    #[serde(rename = "HideExclusionsFromLocalUsers")]
    pub hide_exclusions_from_local_users: Option<bool>,

/// 
    #[serde(rename = "HighThreatDefaultAction")]
    pub high_threat_default_action: Option<u8>,

/// 
    #[serde(rename = "IntelTDTEnabled")]
    pub intel_tdtenabled: Option<bool>,

/// 
    #[serde(rename = "LowThreatDefaultAction")]
    pub low_threat_default_action: Option<u8>,

/// 
    #[serde(rename = "MAPSReporting")]
    pub mapsreporting: Option<u8>,

/// 
    #[serde(rename = "MeteredConnectionUpdates")]
    pub metered_connection_updates: Option<bool>,

/// 
    #[serde(rename = "ModerateThreatDefaultAction")]
    pub moderate_threat_default_action: Option<u8>,

/// 
    #[serde(rename = "NetworkProtectionReputationMode")]
    pub network_protection_reputation_mode: Option<u32>,

/// 
    #[serde(rename = "OobeEnableRtpAndSigUpdate")]
    pub oobe_enable_rtp_and_sig_update: Option<bool>,

/// 
    #[serde(rename = "PerformanceModeStatus")]
    pub performance_mode_status: Option<u8>,

/// 
    #[serde(rename = "PlatformUpdatesChannel")]
    pub platform_updates_channel: Option<u8>,

/// 
    #[serde(rename = "ProxyBypass")]
    pub proxy_bypass: Vec<String>,

/// 
    #[serde(rename = "ProxyPacUrl")]
    pub proxy_pac_url: Option<String>,

/// 
    #[serde(rename = "ProxyServer")]
    pub proxy_server: Option<String>,

/// 
    #[serde(rename = "PUAProtection")]
    pub puaprotection: Option<u8>,

/// 
    #[serde(rename = "QuarantinePurgeItemsAfterDelay")]
    pub quarantine_purge_items_after_delay: Option<u32>,

/// 
    #[serde(rename = "QuickScanIncludeExclusions")]
    pub quick_scan_include_exclusions: Option<u8>,

/// 
    #[serde(rename = "RandomizeScheduleTaskTimes")]
    pub randomize_schedule_task_times: Option<bool>,

/// 
    #[serde(rename = "RealTimeScanDirection")]
    pub real_time_scan_direction: Option<u8>,

/// 
    #[serde(rename = "RemediationScheduleDay")]
    pub remediation_schedule_day: Option<u8>,

/// 
    #[serde(rename = "RemediationScheduleTime")]
    pub remediation_schedule_time: Option<String>,

/// 
    #[serde(rename = "RemoteEncryptionProtectionAggressiveness")]
    pub remote_encryption_protection_aggressiveness: Option<u8>,

/// 
    #[serde(rename = "RemoteEncryptionProtectionConfiguredState")]
    pub remote_encryption_protection_configured_state: Option<u8>,

/// 
    #[serde(rename = "RemoteEncryptionProtectionExclusions")]
    pub remote_encryption_protection_exclusions: Vec<String>,

/// 
    #[serde(rename = "RemoteEncryptionProtectionMaxBlockTime")]
    pub remote_encryption_protection_max_block_time: Option<u32>,

/// 
    #[serde(rename = "RemoveScanningThreadPoolCap")]
    pub remove_scanning_thread_pool_cap: Option<bool>,

/// 
    #[serde(rename = "ReportDynamicSignatureDroppedEvent")]
    pub report_dynamic_signature_dropped_event: Option<bool>,

/// 
    #[serde(rename = "ReportingAdditionalActionTimeOut")]
    pub reporting_additional_action_time_out: Option<u32>,

/// 
    #[serde(rename = "ReportingCriticalFailureTimeOut")]
    pub reporting_critical_failure_time_out: Option<u32>,

/// 
    #[serde(rename = "ReportingNonCriticalTimeOut")]
    pub reporting_non_critical_time_out: Option<u32>,

/// 
    #[serde(rename = "ScanAvgCPULoadFactor")]
    pub scan_avg_cpuload_factor: Option<u8>,

/// 
    #[serde(rename = "ScanOnlyIfIdleEnabled")]
    pub scan_only_if_idle_enabled: Option<bool>,

/// 
    #[serde(rename = "ScanParameters")]
    pub scan_parameters: Option<u8>,

/// 
    #[serde(rename = "ScanPurgeItemsAfterDelay")]
    pub scan_purge_items_after_delay: Option<u32>,

/// 
    #[serde(rename = "ScanScheduleDay")]
    pub scan_schedule_day: Option<u8>,

/// 
    #[serde(rename = "ScanScheduleOffset")]
    pub scan_schedule_offset: Option<u32>,

/// 
    #[serde(rename = "ScanScheduleQuickScanTime")]
    pub scan_schedule_quick_scan_time: Option<String>,

/// 
    #[serde(rename = "ScanScheduleTime")]
    pub scan_schedule_time: Option<String>,

/// 
    #[serde(rename = "SchedulerRandomizationTime")]
    pub scheduler_randomization_time: Option<u32>,

/// 
    #[serde(rename = "ServiceHealthReportInterval")]
    pub service_health_report_interval: Option<u32>,

/// 
    #[serde(rename = "SevereThreatDefaultAction")]
    pub severe_threat_default_action: Option<u8>,

/// 
    #[serde(rename = "SharedSignaturesPath")]
    pub shared_signatures_path: Option<String>,

/// 
    #[serde(rename = "SharedSignaturesPathUpdateAtScheduledTimeOnly")]
    pub shared_signatures_path_update_at_scheduled_time_only: Option<bool>,

/// 
    #[serde(rename = "SignatureAuGracePeriod")]
    pub signature_au_grace_period: Option<u32>,

/// 
    #[serde(rename = "SignatureBlobFileSharesSources")]
    pub signature_blob_file_shares_sources: Option<String>,

/// 
    #[serde(rename = "SignatureBlobUpdateInterval")]
    pub signature_blob_update_interval: Option<u32>,

/// 
    #[serde(rename = "SignatureDefinitionUpdateFileSharesSources")]
    pub signature_definition_update_file_shares_sources: Option<String>,

/// 
    #[serde(rename = "SignatureDisableUpdateOnStartupWithoutEngine")]
    pub signature_disable_update_on_startup_without_engine: Option<bool>,

/// 
    #[serde(rename = "SignatureFallbackOrder")]
    pub signature_fallback_order: Option<String>,

/// 
    #[serde(rename = "SignatureFirstAuGracePeriod")]
    pub signature_first_au_grace_period: Option<u32>,

/// 
    #[serde(rename = "SignatureScheduleDay")]
    pub signature_schedule_day: Option<u8>,

/// 
    #[serde(rename = "SignatureScheduleTime")]
    pub signature_schedule_time: Option<String>,

/// 
    #[serde(rename = "SignatureUpdateCatchupInterval")]
    pub signature_update_catchup_interval: Option<u32>,

/// 
    #[serde(rename = "SignatureUpdateInterval")]
    pub signature_update_interval: Option<u32>,

/// 
    #[serde(rename = "SubmitSamplesConsent")]
    pub submit_samples_consent: Option<u8>,

/// 
    #[serde(rename = "ThreatIDDefaultAction_Actions")]
    pub threat_iddefault_action__actions: Vec<u8>,

/// 
    #[serde(rename = "ThreatIDDefaultAction_Ids")]
    pub threat_iddefault_action__ids: Vec<i64>,

/// 
    #[serde(rename = "ThrottleForScheduledScanOnly")]
    pub throttle_for_scheduled_scan_only: Option<bool>,

/// 
    #[serde(rename = "TrustLabelProtectionStatus")]
    pub trust_label_protection_status: Option<u32>,

/// 
    #[serde(rename = "UILockdown")]
    pub uilockdown: Option<bool>,

/// 
    #[serde(rename = "UnknownThreatDefaultAction")]
    pub unknown_threat_default_action: Option<u8>,
}

impl MSFT_MpPreference {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_datagram_processing_on_win_server: None,
            allow_network_protection_down_level: None,
            allow_network_protection_on_win_server: None,
            allow_switch_to_async_inspection: None,
            apply_disable_network_scanning_to_ioav: None,
            attack_surface_reduction_only_exclusions: Vec::new(),
            attack_surface_reduction_rules__actions: Vec::new(),
            attack_surface_reduction_rules__ids: Vec::new(),
            attack_surface_reduction_rules__rule_specific_exclusions: Vec::new(),
            attack_surface_reduction_rules__rule_specific_exclusions__id: Vec::new(),
            brute_force_protection_aggressiveness: None,
            brute_force_protection_configured_state: None,
            brute_force_protection_exclusions: Vec::new(),
            brute_force_protection_local_network_blocking: None,
            brute_force_protection_max_block_time: None,
            brute_force_protection_skip_learning_period: None,
            check_for_signatures_before_running_scan: None,
            cloud_block_level: None,
            cloud_extended_timeout: None,
            computer_id: None,
            controlled_folder_access_allowed_applications: Vec::new(),
            controlled_folder_access_default_protected_folders: Vec::new(),
            controlled_folder_access_protected_folders: Vec::new(),
            definition_updates_channel: None,
            disable_archive_scanning: None,
            disable_auto_exclusions: None,
            disable_behavior_monitoring: None,
            disable_block_at_first_seen: None,
            disable_cache_maintenance: None,
            disable_catchup_full_scan: None,
            disable_catchup_quick_scan: None,
            disable_core_service_ecsintegration: None,
            disable_core_service_telemetry: None,
            disable_cpu_throttle_on_idle_scans: None,
            disable_datagram_processing: None,
            disable_dns_over_tcp_parsing: None,
            disable_dns_parsing: None,
            disable_email_scanning: None,
            disable_ftp_parsing: None,
            disable_gradual_release: None,
            disable_http_parsing: None,
            disable_inbound_connection_filtering: None,
            disable_ioavprotection: None,
            disable_network_protection_perf_telemetry: None,
            disable_privacy_mode: None,
            disable_quic_parsing: None,
            disable_rdp_parsing: None,
            disable_realtime_monitoring: None,
            disable_removable_drive_scanning: None,
            disable_restore_point: None,
            disable_scanning_mapped_network_drives_for_full_scan: None,
            disable_scanning_network_files: None,
            disable_script_scanning: None,
            disable_smtp_parsing: None,
            disable_ssh_parsing: None,
            disable_tamper_protection: None,
            disable_tls_parsing: None,
            enable_controlled_folder_access: None,
            enable_convert_warn_to_block: None,
            enable_dns_sinkhole: None,
            enable_file_hash_computation: None,
            enable_full_scan_on_battery_power: None,
            enable_low_cpu_priority: None,
            enable_network_protection: None,
            enable_udp_receive_offload: None,
            enable_udp_segmentation_offload: None,
            engine_updates_channel: None,
            exclusion_extension: Vec::new(),
            exclusion_ip_address: Vec::new(),
            exclusion_path: Vec::new(),
            exclusion_process: Vec::new(),
            force_use_proxy_only: None,
            hide_exclusions_from_local_users: None,
            high_threat_default_action: None,
            intel_tdtenabled: None,
            low_threat_default_action: None,
            mapsreporting: None,
            metered_connection_updates: None,
            moderate_threat_default_action: None,
            network_protection_reputation_mode: None,
            oobe_enable_rtp_and_sig_update: None,
            performance_mode_status: None,
            platform_updates_channel: None,
            proxy_bypass: Vec::new(),
            proxy_pac_url: None,
            proxy_server: None,
            puaprotection: None,
            quarantine_purge_items_after_delay: None,
            quick_scan_include_exclusions: None,
            randomize_schedule_task_times: None,
            real_time_scan_direction: None,
            remediation_schedule_day: None,
            remediation_schedule_time: None,
            remote_encryption_protection_aggressiveness: None,
            remote_encryption_protection_configured_state: None,
            remote_encryption_protection_exclusions: Vec::new(),
            remote_encryption_protection_max_block_time: None,
            remove_scanning_thread_pool_cap: None,
            report_dynamic_signature_dropped_event: None,
            reporting_additional_action_time_out: None,
            reporting_critical_failure_time_out: None,
            reporting_non_critical_time_out: None,
            scan_avg_cpuload_factor: None,
            scan_only_if_idle_enabled: None,
            scan_parameters: None,
            scan_purge_items_after_delay: None,
            scan_schedule_day: None,
            scan_schedule_offset: None,
            scan_schedule_quick_scan_time: None,
            scan_schedule_time: None,
            scheduler_randomization_time: None,
            service_health_report_interval: None,
            severe_threat_default_action: None,
            shared_signatures_path: None,
            shared_signatures_path_update_at_scheduled_time_only: None,
            signature_au_grace_period: None,
            signature_blob_file_shares_sources: None,
            signature_blob_update_interval: None,
            signature_definition_update_file_shares_sources: None,
            signature_disable_update_on_startup_without_engine: None,
            signature_fallback_order: None,
            signature_first_au_grace_period: None,
            signature_schedule_day: None,
            signature_schedule_time: None,
            signature_update_catchup_interval: None,
            signature_update_interval: None,
            submit_samples_consent: None,
            threat_iddefault_action__actions: Vec::new(),
            threat_iddefault_action__ids: Vec::new(),
            throttle_for_scheduled_scan_only: None,
            trust_label_protection_status: None,
            uilockdown: None,
            unknown_threat_default_action: None,
        }
    }


    /// Sets the value of AllowDatagramProcessingOnWinServer
    pub fn set_allow_datagram_processing_on_win_server(&mut self, value: bool) {
        self.allow_datagram_processing_on_win_server = Some(value);
    }

    /// Gets the value of AllowDatagramProcessingOnWinServer
    pub fn get_allow_datagram_processing_on_win_server(&self) -> Option<&bool> {
        self.allow_datagram_processing_on_win_server.as_ref()
    }

    /// Sets the value of AllowNetworkProtectionDownLevel
    pub fn set_allow_network_protection_down_level(&mut self, value: bool) {
        self.allow_network_protection_down_level = Some(value);
    }

    /// Gets the value of AllowNetworkProtectionDownLevel
    pub fn get_allow_network_protection_down_level(&self) -> Option<&bool> {
        self.allow_network_protection_down_level.as_ref()
    }

    /// Sets the value of AllowNetworkProtectionOnWinServer
    pub fn set_allow_network_protection_on_win_server(&mut self, value: bool) {
        self.allow_network_protection_on_win_server = Some(value);
    }

    /// Gets the value of AllowNetworkProtectionOnWinServer
    pub fn get_allow_network_protection_on_win_server(&self) -> Option<&bool> {
        self.allow_network_protection_on_win_server.as_ref()
    }

    /// Sets the value of AllowSwitchToAsyncInspection
    pub fn set_allow_switch_to_async_inspection(&mut self, value: bool) {
        self.allow_switch_to_async_inspection = Some(value);
    }

    /// Gets the value of AllowSwitchToAsyncInspection
    pub fn get_allow_switch_to_async_inspection(&self) -> Option<&bool> {
        self.allow_switch_to_async_inspection.as_ref()
    }

    /// Sets the value of ApplyDisableNetworkScanningToIOAV
    pub fn set_apply_disable_network_scanning_to_ioav(&mut self, value: bool) {
        self.apply_disable_network_scanning_to_ioav = Some(value);
    }

    /// Gets the value of ApplyDisableNetworkScanningToIOAV
    pub fn get_apply_disable_network_scanning_to_ioav(&self) -> Option<&bool> {
        self.apply_disable_network_scanning_to_ioav.as_ref()
    }

    /// Sets the value of AttackSurfaceReductionOnlyExclusions
    pub fn set_attack_surface_reduction_only_exclusions(&mut self, value: Vec<String>) {
        self.attack_surface_reduction_only_exclusions = value;
    }

    /// Gets the value of AttackSurfaceReductionOnlyExclusions
    pub fn get_attack_surface_reduction_only_exclusions(&self) -> &Vec<String> {
        &self.attack_surface_reduction_only_exclusions
    }

    /// Sets the value of AttackSurfaceReductionRules_Actions
    pub fn set_attack_surface_reduction_rules__actions(&mut self, value: Vec<u8>) {
        self.attack_surface_reduction_rules__actions = value;
    }

    /// Gets the value of AttackSurfaceReductionRules_Actions
    pub fn get_attack_surface_reduction_rules__actions(&self) -> &Vec<u8> {
        &self.attack_surface_reduction_rules__actions
    }

    /// Sets the value of AttackSurfaceReductionRules_Ids
    pub fn set_attack_surface_reduction_rules__ids(&mut self, value: Vec<String>) {
        self.attack_surface_reduction_rules__ids = value;
    }

    /// Gets the value of AttackSurfaceReductionRules_Ids
    pub fn get_attack_surface_reduction_rules__ids(&self) -> &Vec<String> {
        &self.attack_surface_reduction_rules__ids
    }

    /// Sets the value of AttackSurfaceReductionRules_RuleSpecificExclusions
    pub fn set_attack_surface_reduction_rules__rule_specific_exclusions(&mut self, value: Vec<String>) {
        self.attack_surface_reduction_rules__rule_specific_exclusions = value;
    }

    /// Gets the value of AttackSurfaceReductionRules_RuleSpecificExclusions
    pub fn get_attack_surface_reduction_rules__rule_specific_exclusions(&self) -> &Vec<String> {
        &self.attack_surface_reduction_rules__rule_specific_exclusions
    }

    /// Sets the value of AttackSurfaceReductionRules_RuleSpecificExclusions_Id
    pub fn set_attack_surface_reduction_rules__rule_specific_exclusions__id(&mut self, value: Vec<String>) {
        self.attack_surface_reduction_rules__rule_specific_exclusions__id = value;
    }

    /// Gets the value of AttackSurfaceReductionRules_RuleSpecificExclusions_Id
    pub fn get_attack_surface_reduction_rules__rule_specific_exclusions__id(&self) -> &Vec<String> {
        &self.attack_surface_reduction_rules__rule_specific_exclusions__id
    }

    /// Sets the value of BruteForceProtectionAggressiveness
    pub fn set_brute_force_protection_aggressiveness(&mut self, value: u8) {
        self.brute_force_protection_aggressiveness = Some(value);
    }

    /// Gets the value of BruteForceProtectionAggressiveness
    pub fn get_brute_force_protection_aggressiveness(&self) -> Option<&u8> {
        self.brute_force_protection_aggressiveness.as_ref()
    }

    /// Sets the value of BruteForceProtectionConfiguredState
    pub fn set_brute_force_protection_configured_state(&mut self, value: u8) {
        self.brute_force_protection_configured_state = Some(value);
    }

    /// Gets the value of BruteForceProtectionConfiguredState
    pub fn get_brute_force_protection_configured_state(&self) -> Option<&u8> {
        self.brute_force_protection_configured_state.as_ref()
    }

    /// Sets the value of BruteForceProtectionExclusions
    pub fn set_brute_force_protection_exclusions(&mut self, value: Vec<String>) {
        self.brute_force_protection_exclusions = value;
    }

    /// Gets the value of BruteForceProtectionExclusions
    pub fn get_brute_force_protection_exclusions(&self) -> &Vec<String> {
        &self.brute_force_protection_exclusions
    }

    /// Sets the value of BruteForceProtectionLocalNetworkBlocking
    pub fn set_brute_force_protection_local_network_blocking(&mut self, value: bool) {
        self.brute_force_protection_local_network_blocking = Some(value);
    }

    /// Gets the value of BruteForceProtectionLocalNetworkBlocking
    pub fn get_brute_force_protection_local_network_blocking(&self) -> Option<&bool> {
        self.brute_force_protection_local_network_blocking.as_ref()
    }

    /// Sets the value of BruteForceProtectionMaxBlockTime
    pub fn set_brute_force_protection_max_block_time(&mut self, value: u32) {
        self.brute_force_protection_max_block_time = Some(value);
    }

    /// Gets the value of BruteForceProtectionMaxBlockTime
    pub fn get_brute_force_protection_max_block_time(&self) -> Option<&u32> {
        self.brute_force_protection_max_block_time.as_ref()
    }

    /// Sets the value of BruteForceProtectionSkipLearningPeriod
    pub fn set_brute_force_protection_skip_learning_period(&mut self, value: bool) {
        self.brute_force_protection_skip_learning_period = Some(value);
    }

    /// Gets the value of BruteForceProtectionSkipLearningPeriod
    pub fn get_brute_force_protection_skip_learning_period(&self) -> Option<&bool> {
        self.brute_force_protection_skip_learning_period.as_ref()
    }

    /// Sets the value of CheckForSignaturesBeforeRunningScan
    pub fn set_check_for_signatures_before_running_scan(&mut self, value: bool) {
        self.check_for_signatures_before_running_scan = Some(value);
    }

    /// Gets the value of CheckForSignaturesBeforeRunningScan
    pub fn get_check_for_signatures_before_running_scan(&self) -> Option<&bool> {
        self.check_for_signatures_before_running_scan.as_ref()
    }

    /// Sets the value of CloudBlockLevel
    pub fn set_cloud_block_level(&mut self, value: u8) {
        self.cloud_block_level = Some(value);
    }

    /// Gets the value of CloudBlockLevel
    pub fn get_cloud_block_level(&self) -> Option<&u8> {
        self.cloud_block_level.as_ref()
    }

    /// Sets the value of CloudExtendedTimeout
    pub fn set_cloud_extended_timeout(&mut self, value: u32) {
        self.cloud_extended_timeout = Some(value);
    }

    /// Gets the value of CloudExtendedTimeout
    pub fn get_cloud_extended_timeout(&self) -> Option<&u32> {
        self.cloud_extended_timeout.as_ref()
    }

    /// Sets the value of ComputerID
    pub fn set_computer_id(&mut self, value: String) {
        self.computer_id = Some(value);
    }

    /// Gets the value of ComputerID
    pub fn get_computer_id(&self) -> Option<&String> {
        self.computer_id.as_ref()
    }

    /// Sets the value of ControlledFolderAccessAllowedApplications
    pub fn set_controlled_folder_access_allowed_applications(&mut self, value: Vec<String>) {
        self.controlled_folder_access_allowed_applications = value;
    }

    /// Gets the value of ControlledFolderAccessAllowedApplications
    pub fn get_controlled_folder_access_allowed_applications(&self) -> &Vec<String> {
        &self.controlled_folder_access_allowed_applications
    }

    /// Sets the value of ControlledFolderAccessDefaultProtectedFolders
    pub fn set_controlled_folder_access_default_protected_folders(&mut self, value: Vec<String>) {
        self.controlled_folder_access_default_protected_folders = value;
    }

    /// Gets the value of ControlledFolderAccessDefaultProtectedFolders
    pub fn get_controlled_folder_access_default_protected_folders(&self) -> &Vec<String> {
        &self.controlled_folder_access_default_protected_folders
    }

    /// Sets the value of ControlledFolderAccessProtectedFolders
    pub fn set_controlled_folder_access_protected_folders(&mut self, value: Vec<String>) {
        self.controlled_folder_access_protected_folders = value;
    }

    /// Gets the value of ControlledFolderAccessProtectedFolders
    pub fn get_controlled_folder_access_protected_folders(&self) -> &Vec<String> {
        &self.controlled_folder_access_protected_folders
    }

    /// Sets the value of DefinitionUpdatesChannel
    pub fn set_definition_updates_channel(&mut self, value: u8) {
        self.definition_updates_channel = Some(value);
    }

    /// Gets the value of DefinitionUpdatesChannel
    pub fn get_definition_updates_channel(&self) -> Option<&u8> {
        self.definition_updates_channel.as_ref()
    }

    /// Sets the value of DisableArchiveScanning
    pub fn set_disable_archive_scanning(&mut self, value: bool) {
        self.disable_archive_scanning = Some(value);
    }

    /// Gets the value of DisableArchiveScanning
    pub fn get_disable_archive_scanning(&self) -> Option<&bool> {
        self.disable_archive_scanning.as_ref()
    }

    /// Sets the value of DisableAutoExclusions
    pub fn set_disable_auto_exclusions(&mut self, value: bool) {
        self.disable_auto_exclusions = Some(value);
    }

    /// Gets the value of DisableAutoExclusions
    pub fn get_disable_auto_exclusions(&self) -> Option<&bool> {
        self.disable_auto_exclusions.as_ref()
    }

    /// Sets the value of DisableBehaviorMonitoring
    pub fn set_disable_behavior_monitoring(&mut self, value: bool) {
        self.disable_behavior_monitoring = Some(value);
    }

    /// Gets the value of DisableBehaviorMonitoring
    pub fn get_disable_behavior_monitoring(&self) -> Option<&bool> {
        self.disable_behavior_monitoring.as_ref()
    }

    /// Sets the value of DisableBlockAtFirstSeen
    pub fn set_disable_block_at_first_seen(&mut self, value: bool) {
        self.disable_block_at_first_seen = Some(value);
    }

    /// Gets the value of DisableBlockAtFirstSeen
    pub fn get_disable_block_at_first_seen(&self) -> Option<&bool> {
        self.disable_block_at_first_seen.as_ref()
    }

    /// Sets the value of DisableCacheMaintenance
    pub fn set_disable_cache_maintenance(&mut self, value: bool) {
        self.disable_cache_maintenance = Some(value);
    }

    /// Gets the value of DisableCacheMaintenance
    pub fn get_disable_cache_maintenance(&self) -> Option<&bool> {
        self.disable_cache_maintenance.as_ref()
    }

    /// Sets the value of DisableCatchupFullScan
    pub fn set_disable_catchup_full_scan(&mut self, value: bool) {
        self.disable_catchup_full_scan = Some(value);
    }

    /// Gets the value of DisableCatchupFullScan
    pub fn get_disable_catchup_full_scan(&self) -> Option<&bool> {
        self.disable_catchup_full_scan.as_ref()
    }

    /// Sets the value of DisableCatchupQuickScan
    pub fn set_disable_catchup_quick_scan(&mut self, value: bool) {
        self.disable_catchup_quick_scan = Some(value);
    }

    /// Gets the value of DisableCatchupQuickScan
    pub fn get_disable_catchup_quick_scan(&self) -> Option<&bool> {
        self.disable_catchup_quick_scan.as_ref()
    }

    /// Sets the value of DisableCoreServiceECSIntegration
    pub fn set_disable_core_service_ecsintegration(&mut self, value: bool) {
        self.disable_core_service_ecsintegration = Some(value);
    }

    /// Gets the value of DisableCoreServiceECSIntegration
    pub fn get_disable_core_service_ecsintegration(&self) -> Option<&bool> {
        self.disable_core_service_ecsintegration.as_ref()
    }

    /// Sets the value of DisableCoreServiceTelemetry
    pub fn set_disable_core_service_telemetry(&mut self, value: bool) {
        self.disable_core_service_telemetry = Some(value);
    }

    /// Gets the value of DisableCoreServiceTelemetry
    pub fn get_disable_core_service_telemetry(&self) -> Option<&bool> {
        self.disable_core_service_telemetry.as_ref()
    }

    /// Sets the value of DisableCpuThrottleOnIdleScans
    pub fn set_disable_cpu_throttle_on_idle_scans(&mut self, value: bool) {
        self.disable_cpu_throttle_on_idle_scans = Some(value);
    }

    /// Gets the value of DisableCpuThrottleOnIdleScans
    pub fn get_disable_cpu_throttle_on_idle_scans(&self) -> Option<&bool> {
        self.disable_cpu_throttle_on_idle_scans.as_ref()
    }

    /// Sets the value of DisableDatagramProcessing
    pub fn set_disable_datagram_processing(&mut self, value: bool) {
        self.disable_datagram_processing = Some(value);
    }

    /// Gets the value of DisableDatagramProcessing
    pub fn get_disable_datagram_processing(&self) -> Option<&bool> {
        self.disable_datagram_processing.as_ref()
    }

    /// Sets the value of DisableDnsOverTcpParsing
    pub fn set_disable_dns_over_tcp_parsing(&mut self, value: bool) {
        self.disable_dns_over_tcp_parsing = Some(value);
    }

    /// Gets the value of DisableDnsOverTcpParsing
    pub fn get_disable_dns_over_tcp_parsing(&self) -> Option<&bool> {
        self.disable_dns_over_tcp_parsing.as_ref()
    }

    /// Sets the value of DisableDnsParsing
    pub fn set_disable_dns_parsing(&mut self, value: bool) {
        self.disable_dns_parsing = Some(value);
    }

    /// Gets the value of DisableDnsParsing
    pub fn get_disable_dns_parsing(&self) -> Option<&bool> {
        self.disable_dns_parsing.as_ref()
    }

    /// Sets the value of DisableEmailScanning
    pub fn set_disable_email_scanning(&mut self, value: bool) {
        self.disable_email_scanning = Some(value);
    }

    /// Gets the value of DisableEmailScanning
    pub fn get_disable_email_scanning(&self) -> Option<&bool> {
        self.disable_email_scanning.as_ref()
    }

    /// Sets the value of DisableFtpParsing
    pub fn set_disable_ftp_parsing(&mut self, value: bool) {
        self.disable_ftp_parsing = Some(value);
    }

    /// Gets the value of DisableFtpParsing
    pub fn get_disable_ftp_parsing(&self) -> Option<&bool> {
        self.disable_ftp_parsing.as_ref()
    }

    /// Sets the value of DisableGradualRelease
    pub fn set_disable_gradual_release(&mut self, value: bool) {
        self.disable_gradual_release = Some(value);
    }

    /// Gets the value of DisableGradualRelease
    pub fn get_disable_gradual_release(&self) -> Option<&bool> {
        self.disable_gradual_release.as_ref()
    }

    /// Sets the value of DisableHttpParsing
    pub fn set_disable_http_parsing(&mut self, value: bool) {
        self.disable_http_parsing = Some(value);
    }

    /// Gets the value of DisableHttpParsing
    pub fn get_disable_http_parsing(&self) -> Option<&bool> {
        self.disable_http_parsing.as_ref()
    }

    /// Sets the value of DisableInboundConnectionFiltering
    pub fn set_disable_inbound_connection_filtering(&mut self, value: bool) {
        self.disable_inbound_connection_filtering = Some(value);
    }

    /// Gets the value of DisableInboundConnectionFiltering
    pub fn get_disable_inbound_connection_filtering(&self) -> Option<&bool> {
        self.disable_inbound_connection_filtering.as_ref()
    }

    /// Sets the value of DisableIOAVProtection
    pub fn set_disable_ioavprotection(&mut self, value: bool) {
        self.disable_ioavprotection = Some(value);
    }

    /// Gets the value of DisableIOAVProtection
    pub fn get_disable_ioavprotection(&self) -> Option<&bool> {
        self.disable_ioavprotection.as_ref()
    }

    /// Sets the value of DisableNetworkProtectionPerfTelemetry
    pub fn set_disable_network_protection_perf_telemetry(&mut self, value: bool) {
        self.disable_network_protection_perf_telemetry = Some(value);
    }

    /// Gets the value of DisableNetworkProtectionPerfTelemetry
    pub fn get_disable_network_protection_perf_telemetry(&self) -> Option<&bool> {
        self.disable_network_protection_perf_telemetry.as_ref()
    }

    /// Sets the value of DisablePrivacyMode
    pub fn set_disable_privacy_mode(&mut self, value: bool) {
        self.disable_privacy_mode = Some(value);
    }

    /// Gets the value of DisablePrivacyMode
    pub fn get_disable_privacy_mode(&self) -> Option<&bool> {
        self.disable_privacy_mode.as_ref()
    }

    /// Sets the value of DisableQuicParsing
    pub fn set_disable_quic_parsing(&mut self, value: bool) {
        self.disable_quic_parsing = Some(value);
    }

    /// Gets the value of DisableQuicParsing
    pub fn get_disable_quic_parsing(&self) -> Option<&bool> {
        self.disable_quic_parsing.as_ref()
    }

    /// Sets the value of DisableRdpParsing
    pub fn set_disable_rdp_parsing(&mut self, value: bool) {
        self.disable_rdp_parsing = Some(value);
    }

    /// Gets the value of DisableRdpParsing
    pub fn get_disable_rdp_parsing(&self) -> Option<&bool> {
        self.disable_rdp_parsing.as_ref()
    }

    /// Sets the value of DisableRealtimeMonitoring
    pub fn set_disable_realtime_monitoring(&mut self, value: bool) {
        self.disable_realtime_monitoring = Some(value);
    }

    /// Gets the value of DisableRealtimeMonitoring
    pub fn get_disable_realtime_monitoring(&self) -> Option<&bool> {
        self.disable_realtime_monitoring.as_ref()
    }

    /// Sets the value of DisableRemovableDriveScanning
    pub fn set_disable_removable_drive_scanning(&mut self, value: bool) {
        self.disable_removable_drive_scanning = Some(value);
    }

    /// Gets the value of DisableRemovableDriveScanning
    pub fn get_disable_removable_drive_scanning(&self) -> Option<&bool> {
        self.disable_removable_drive_scanning.as_ref()
    }

    /// Sets the value of DisableRestorePoint
    pub fn set_disable_restore_point(&mut self, value: bool) {
        self.disable_restore_point = Some(value);
    }

    /// Gets the value of DisableRestorePoint
    pub fn get_disable_restore_point(&self) -> Option<&bool> {
        self.disable_restore_point.as_ref()
    }

    /// Sets the value of DisableScanningMappedNetworkDrivesForFullScan
    pub fn set_disable_scanning_mapped_network_drives_for_full_scan(&mut self, value: bool) {
        self.disable_scanning_mapped_network_drives_for_full_scan = Some(value);
    }

    /// Gets the value of DisableScanningMappedNetworkDrivesForFullScan
    pub fn get_disable_scanning_mapped_network_drives_for_full_scan(&self) -> Option<&bool> {
        self.disable_scanning_mapped_network_drives_for_full_scan.as_ref()
    }

    /// Sets the value of DisableScanningNetworkFiles
    pub fn set_disable_scanning_network_files(&mut self, value: bool) {
        self.disable_scanning_network_files = Some(value);
    }

    /// Gets the value of DisableScanningNetworkFiles
    pub fn get_disable_scanning_network_files(&self) -> Option<&bool> {
        self.disable_scanning_network_files.as_ref()
    }

    /// Sets the value of DisableScriptScanning
    pub fn set_disable_script_scanning(&mut self, value: bool) {
        self.disable_script_scanning = Some(value);
    }

    /// Gets the value of DisableScriptScanning
    pub fn get_disable_script_scanning(&self) -> Option<&bool> {
        self.disable_script_scanning.as_ref()
    }

    /// Sets the value of DisableSmtpParsing
    pub fn set_disable_smtp_parsing(&mut self, value: bool) {
        self.disable_smtp_parsing = Some(value);
    }

    /// Gets the value of DisableSmtpParsing
    pub fn get_disable_smtp_parsing(&self) -> Option<&bool> {
        self.disable_smtp_parsing.as_ref()
    }

    /// Sets the value of DisableSshParsing
    pub fn set_disable_ssh_parsing(&mut self, value: bool) {
        self.disable_ssh_parsing = Some(value);
    }

    /// Gets the value of DisableSshParsing
    pub fn get_disable_ssh_parsing(&self) -> Option<&bool> {
        self.disable_ssh_parsing.as_ref()
    }

    /// Sets the value of DisableTamperProtection
    pub fn set_disable_tamper_protection(&mut self, value: bool) {
        self.disable_tamper_protection = Some(value);
    }

    /// Gets the value of DisableTamperProtection
    pub fn get_disable_tamper_protection(&self) -> Option<&bool> {
        self.disable_tamper_protection.as_ref()
    }

    /// Sets the value of DisableTlsParsing
    pub fn set_disable_tls_parsing(&mut self, value: bool) {
        self.disable_tls_parsing = Some(value);
    }

    /// Gets the value of DisableTlsParsing
    pub fn get_disable_tls_parsing(&self) -> Option<&bool> {
        self.disable_tls_parsing.as_ref()
    }

    /// Sets the value of EnableControlledFolderAccess
    pub fn set_enable_controlled_folder_access(&mut self, value: u8) {
        self.enable_controlled_folder_access = Some(value);
    }

    /// Gets the value of EnableControlledFolderAccess
    pub fn get_enable_controlled_folder_access(&self) -> Option<&u8> {
        self.enable_controlled_folder_access.as_ref()
    }

    /// Sets the value of EnableConvertWarnToBlock
    pub fn set_enable_convert_warn_to_block(&mut self, value: bool) {
        self.enable_convert_warn_to_block = Some(value);
    }

    /// Gets the value of EnableConvertWarnToBlock
    pub fn get_enable_convert_warn_to_block(&self) -> Option<&bool> {
        self.enable_convert_warn_to_block.as_ref()
    }

    /// Sets the value of EnableDnsSinkhole
    pub fn set_enable_dns_sinkhole(&mut self, value: bool) {
        self.enable_dns_sinkhole = Some(value);
    }

    /// Gets the value of EnableDnsSinkhole
    pub fn get_enable_dns_sinkhole(&self) -> Option<&bool> {
        self.enable_dns_sinkhole.as_ref()
    }

    /// Sets the value of EnableFileHashComputation
    pub fn set_enable_file_hash_computation(&mut self, value: bool) {
        self.enable_file_hash_computation = Some(value);
    }

    /// Gets the value of EnableFileHashComputation
    pub fn get_enable_file_hash_computation(&self) -> Option<&bool> {
        self.enable_file_hash_computation.as_ref()
    }

    /// Sets the value of EnableFullScanOnBatteryPower
    pub fn set_enable_full_scan_on_battery_power(&mut self, value: bool) {
        self.enable_full_scan_on_battery_power = Some(value);
    }

    /// Gets the value of EnableFullScanOnBatteryPower
    pub fn get_enable_full_scan_on_battery_power(&self) -> Option<&bool> {
        self.enable_full_scan_on_battery_power.as_ref()
    }

    /// Sets the value of EnableLowCpuPriority
    pub fn set_enable_low_cpu_priority(&mut self, value: bool) {
        self.enable_low_cpu_priority = Some(value);
    }

    /// Gets the value of EnableLowCpuPriority
    pub fn get_enable_low_cpu_priority(&self) -> Option<&bool> {
        self.enable_low_cpu_priority.as_ref()
    }

    /// Sets the value of EnableNetworkProtection
    pub fn set_enable_network_protection(&mut self, value: u8) {
        self.enable_network_protection = Some(value);
    }

    /// Gets the value of EnableNetworkProtection
    pub fn get_enable_network_protection(&self) -> Option<&u8> {
        self.enable_network_protection.as_ref()
    }

    /// Sets the value of EnableUdpReceiveOffload
    pub fn set_enable_udp_receive_offload(&mut self, value: bool) {
        self.enable_udp_receive_offload = Some(value);
    }

    /// Gets the value of EnableUdpReceiveOffload
    pub fn get_enable_udp_receive_offload(&self) -> Option<&bool> {
        self.enable_udp_receive_offload.as_ref()
    }

    /// Sets the value of EnableUdpSegmentationOffload
    pub fn set_enable_udp_segmentation_offload(&mut self, value: bool) {
        self.enable_udp_segmentation_offload = Some(value);
    }

    /// Gets the value of EnableUdpSegmentationOffload
    pub fn get_enable_udp_segmentation_offload(&self) -> Option<&bool> {
        self.enable_udp_segmentation_offload.as_ref()
    }

    /// Sets the value of EngineUpdatesChannel
    pub fn set_engine_updates_channel(&mut self, value: u8) {
        self.engine_updates_channel = Some(value);
    }

    /// Gets the value of EngineUpdatesChannel
    pub fn get_engine_updates_channel(&self) -> Option<&u8> {
        self.engine_updates_channel.as_ref()
    }

    /// Sets the value of ExclusionExtension
    pub fn set_exclusion_extension(&mut self, value: Vec<String>) {
        self.exclusion_extension = value;
    }

    /// Gets the value of ExclusionExtension
    pub fn get_exclusion_extension(&self) -> &Vec<String> {
        &self.exclusion_extension
    }

    /// Sets the value of ExclusionIpAddress
    pub fn set_exclusion_ip_address(&mut self, value: Vec<String>) {
        self.exclusion_ip_address = value;
    }

    /// Gets the value of ExclusionIpAddress
    pub fn get_exclusion_ip_address(&self) -> &Vec<String> {
        &self.exclusion_ip_address
    }

    /// Sets the value of ExclusionPath
    pub fn set_exclusion_path(&mut self, value: Vec<String>) {
        self.exclusion_path = value;
    }

    /// Gets the value of ExclusionPath
    pub fn get_exclusion_path(&self) -> &Vec<String> {
        &self.exclusion_path
    }

    /// Sets the value of ExclusionProcess
    pub fn set_exclusion_process(&mut self, value: Vec<String>) {
        self.exclusion_process = value;
    }

    /// Gets the value of ExclusionProcess
    pub fn get_exclusion_process(&self) -> &Vec<String> {
        &self.exclusion_process
    }

    /// Sets the value of ForceUseProxyOnly
    pub fn set_force_use_proxy_only(&mut self, value: bool) {
        self.force_use_proxy_only = Some(value);
    }

    /// Gets the value of ForceUseProxyOnly
    pub fn get_force_use_proxy_only(&self) -> Option<&bool> {
        self.force_use_proxy_only.as_ref()
    }

    /// Sets the value of HideExclusionsFromLocalUsers
    pub fn set_hide_exclusions_from_local_users(&mut self, value: bool) {
        self.hide_exclusions_from_local_users = Some(value);
    }

    /// Gets the value of HideExclusionsFromLocalUsers
    pub fn get_hide_exclusions_from_local_users(&self) -> Option<&bool> {
        self.hide_exclusions_from_local_users.as_ref()
    }

    /// Sets the value of HighThreatDefaultAction
    pub fn set_high_threat_default_action(&mut self, value: u8) {
        self.high_threat_default_action = Some(value);
    }

    /// Gets the value of HighThreatDefaultAction
    pub fn get_high_threat_default_action(&self) -> Option<&u8> {
        self.high_threat_default_action.as_ref()
    }

    /// Sets the value of IntelTDTEnabled
    pub fn set_intel_tdtenabled(&mut self, value: bool) {
        self.intel_tdtenabled = Some(value);
    }

    /// Gets the value of IntelTDTEnabled
    pub fn get_intel_tdtenabled(&self) -> Option<&bool> {
        self.intel_tdtenabled.as_ref()
    }

    /// Sets the value of LowThreatDefaultAction
    pub fn set_low_threat_default_action(&mut self, value: u8) {
        self.low_threat_default_action = Some(value);
    }

    /// Gets the value of LowThreatDefaultAction
    pub fn get_low_threat_default_action(&self) -> Option<&u8> {
        self.low_threat_default_action.as_ref()
    }

    /// Sets the value of MAPSReporting
    pub fn set_mapsreporting(&mut self, value: u8) {
        self.mapsreporting = Some(value);
    }

    /// Gets the value of MAPSReporting
    pub fn get_mapsreporting(&self) -> Option<&u8> {
        self.mapsreporting.as_ref()
    }

    /// Sets the value of MeteredConnectionUpdates
    pub fn set_metered_connection_updates(&mut self, value: bool) {
        self.metered_connection_updates = Some(value);
    }

    /// Gets the value of MeteredConnectionUpdates
    pub fn get_metered_connection_updates(&self) -> Option<&bool> {
        self.metered_connection_updates.as_ref()
    }

    /// Sets the value of ModerateThreatDefaultAction
    pub fn set_moderate_threat_default_action(&mut self, value: u8) {
        self.moderate_threat_default_action = Some(value);
    }

    /// Gets the value of ModerateThreatDefaultAction
    pub fn get_moderate_threat_default_action(&self) -> Option<&u8> {
        self.moderate_threat_default_action.as_ref()
    }

    /// Sets the value of NetworkProtectionReputationMode
    pub fn set_network_protection_reputation_mode(&mut self, value: u32) {
        self.network_protection_reputation_mode = Some(value);
    }

    /// Gets the value of NetworkProtectionReputationMode
    pub fn get_network_protection_reputation_mode(&self) -> Option<&u32> {
        self.network_protection_reputation_mode.as_ref()
    }

    /// Sets the value of OobeEnableRtpAndSigUpdate
    pub fn set_oobe_enable_rtp_and_sig_update(&mut self, value: bool) {
        self.oobe_enable_rtp_and_sig_update = Some(value);
    }

    /// Gets the value of OobeEnableRtpAndSigUpdate
    pub fn get_oobe_enable_rtp_and_sig_update(&self) -> Option<&bool> {
        self.oobe_enable_rtp_and_sig_update.as_ref()
    }

    /// Sets the value of PerformanceModeStatus
    pub fn set_performance_mode_status(&mut self, value: u8) {
        self.performance_mode_status = Some(value);
    }

    /// Gets the value of PerformanceModeStatus
    pub fn get_performance_mode_status(&self) -> Option<&u8> {
        self.performance_mode_status.as_ref()
    }

    /// Sets the value of PlatformUpdatesChannel
    pub fn set_platform_updates_channel(&mut self, value: u8) {
        self.platform_updates_channel = Some(value);
    }

    /// Gets the value of PlatformUpdatesChannel
    pub fn get_platform_updates_channel(&self) -> Option<&u8> {
        self.platform_updates_channel.as_ref()
    }

    /// Sets the value of ProxyBypass
    pub fn set_proxy_bypass(&mut self, value: Vec<String>) {
        self.proxy_bypass = value;
    }

    /// Gets the value of ProxyBypass
    pub fn get_proxy_bypass(&self) -> &Vec<String> {
        &self.proxy_bypass
    }

    /// Sets the value of ProxyPacUrl
    pub fn set_proxy_pac_url(&mut self, value: String) {
        self.proxy_pac_url = Some(value);
    }

    /// Gets the value of ProxyPacUrl
    pub fn get_proxy_pac_url(&self) -> Option<&String> {
        self.proxy_pac_url.as_ref()
    }

    /// Sets the value of ProxyServer
    pub fn set_proxy_server(&mut self, value: String) {
        self.proxy_server = Some(value);
    }

    /// Gets the value of ProxyServer
    pub fn get_proxy_server(&self) -> Option<&String> {
        self.proxy_server.as_ref()
    }

    /// Sets the value of PUAProtection
    pub fn set_puaprotection(&mut self, value: u8) {
        self.puaprotection = Some(value);
    }

    /// Gets the value of PUAProtection
    pub fn get_puaprotection(&self) -> Option<&u8> {
        self.puaprotection.as_ref()
    }

    /// Sets the value of QuarantinePurgeItemsAfterDelay
    pub fn set_quarantine_purge_items_after_delay(&mut self, value: u32) {
        self.quarantine_purge_items_after_delay = Some(value);
    }

    /// Gets the value of QuarantinePurgeItemsAfterDelay
    pub fn get_quarantine_purge_items_after_delay(&self) -> Option<&u32> {
        self.quarantine_purge_items_after_delay.as_ref()
    }

    /// Sets the value of QuickScanIncludeExclusions
    pub fn set_quick_scan_include_exclusions(&mut self, value: u8) {
        self.quick_scan_include_exclusions = Some(value);
    }

    /// Gets the value of QuickScanIncludeExclusions
    pub fn get_quick_scan_include_exclusions(&self) -> Option<&u8> {
        self.quick_scan_include_exclusions.as_ref()
    }

    /// Sets the value of RandomizeScheduleTaskTimes
    pub fn set_randomize_schedule_task_times(&mut self, value: bool) {
        self.randomize_schedule_task_times = Some(value);
    }

    /// Gets the value of RandomizeScheduleTaskTimes
    pub fn get_randomize_schedule_task_times(&self) -> Option<&bool> {
        self.randomize_schedule_task_times.as_ref()
    }

    /// Sets the value of RealTimeScanDirection
    pub fn set_real_time_scan_direction(&mut self, value: u8) {
        self.real_time_scan_direction = Some(value);
    }

    /// Gets the value of RealTimeScanDirection
    pub fn get_real_time_scan_direction(&self) -> Option<&u8> {
        self.real_time_scan_direction.as_ref()
    }

    /// Sets the value of RemediationScheduleDay
    pub fn set_remediation_schedule_day(&mut self, value: u8) {
        self.remediation_schedule_day = Some(value);
    }

    /// Gets the value of RemediationScheduleDay
    pub fn get_remediation_schedule_day(&self) -> Option<&u8> {
        self.remediation_schedule_day.as_ref()
    }

    /// Sets the value of RemediationScheduleTime
    pub fn set_remediation_schedule_time(&mut self, value: String) {
        self.remediation_schedule_time = Some(value);
    }

    /// Gets the value of RemediationScheduleTime
    pub fn get_remediation_schedule_time(&self) -> Option<&String> {
        self.remediation_schedule_time.as_ref()
    }

    /// Sets the value of RemoteEncryptionProtectionAggressiveness
    pub fn set_remote_encryption_protection_aggressiveness(&mut self, value: u8) {
        self.remote_encryption_protection_aggressiveness = Some(value);
    }

    /// Gets the value of RemoteEncryptionProtectionAggressiveness
    pub fn get_remote_encryption_protection_aggressiveness(&self) -> Option<&u8> {
        self.remote_encryption_protection_aggressiveness.as_ref()
    }

    /// Sets the value of RemoteEncryptionProtectionConfiguredState
    pub fn set_remote_encryption_protection_configured_state(&mut self, value: u8) {
        self.remote_encryption_protection_configured_state = Some(value);
    }

    /// Gets the value of RemoteEncryptionProtectionConfiguredState
    pub fn get_remote_encryption_protection_configured_state(&self) -> Option<&u8> {
        self.remote_encryption_protection_configured_state.as_ref()
    }

    /// Sets the value of RemoteEncryptionProtectionExclusions
    pub fn set_remote_encryption_protection_exclusions(&mut self, value: Vec<String>) {
        self.remote_encryption_protection_exclusions = value;
    }

    /// Gets the value of RemoteEncryptionProtectionExclusions
    pub fn get_remote_encryption_protection_exclusions(&self) -> &Vec<String> {
        &self.remote_encryption_protection_exclusions
    }

    /// Sets the value of RemoteEncryptionProtectionMaxBlockTime
    pub fn set_remote_encryption_protection_max_block_time(&mut self, value: u32) {
        self.remote_encryption_protection_max_block_time = Some(value);
    }

    /// Gets the value of RemoteEncryptionProtectionMaxBlockTime
    pub fn get_remote_encryption_protection_max_block_time(&self) -> Option<&u32> {
        self.remote_encryption_protection_max_block_time.as_ref()
    }

    /// Sets the value of RemoveScanningThreadPoolCap
    pub fn set_remove_scanning_thread_pool_cap(&mut self, value: bool) {
        self.remove_scanning_thread_pool_cap = Some(value);
    }

    /// Gets the value of RemoveScanningThreadPoolCap
    pub fn get_remove_scanning_thread_pool_cap(&self) -> Option<&bool> {
        self.remove_scanning_thread_pool_cap.as_ref()
    }

    /// Sets the value of ReportDynamicSignatureDroppedEvent
    pub fn set_report_dynamic_signature_dropped_event(&mut self, value: bool) {
        self.report_dynamic_signature_dropped_event = Some(value);
    }

    /// Gets the value of ReportDynamicSignatureDroppedEvent
    pub fn get_report_dynamic_signature_dropped_event(&self) -> Option<&bool> {
        self.report_dynamic_signature_dropped_event.as_ref()
    }

    /// Sets the value of ReportingAdditionalActionTimeOut
    pub fn set_reporting_additional_action_time_out(&mut self, value: u32) {
        self.reporting_additional_action_time_out = Some(value);
    }

    /// Gets the value of ReportingAdditionalActionTimeOut
    pub fn get_reporting_additional_action_time_out(&self) -> Option<&u32> {
        self.reporting_additional_action_time_out.as_ref()
    }

    /// Sets the value of ReportingCriticalFailureTimeOut
    pub fn set_reporting_critical_failure_time_out(&mut self, value: u32) {
        self.reporting_critical_failure_time_out = Some(value);
    }

    /// Gets the value of ReportingCriticalFailureTimeOut
    pub fn get_reporting_critical_failure_time_out(&self) -> Option<&u32> {
        self.reporting_critical_failure_time_out.as_ref()
    }

    /// Sets the value of ReportingNonCriticalTimeOut
    pub fn set_reporting_non_critical_time_out(&mut self, value: u32) {
        self.reporting_non_critical_time_out = Some(value);
    }

    /// Gets the value of ReportingNonCriticalTimeOut
    pub fn get_reporting_non_critical_time_out(&self) -> Option<&u32> {
        self.reporting_non_critical_time_out.as_ref()
    }

    /// Sets the value of ScanAvgCPULoadFactor
    pub fn set_scan_avg_cpuload_factor(&mut self, value: u8) {
        self.scan_avg_cpuload_factor = Some(value);
    }

    /// Gets the value of ScanAvgCPULoadFactor
    pub fn get_scan_avg_cpuload_factor(&self) -> Option<&u8> {
        self.scan_avg_cpuload_factor.as_ref()
    }

    /// Sets the value of ScanOnlyIfIdleEnabled
    pub fn set_scan_only_if_idle_enabled(&mut self, value: bool) {
        self.scan_only_if_idle_enabled = Some(value);
    }

    /// Gets the value of ScanOnlyIfIdleEnabled
    pub fn get_scan_only_if_idle_enabled(&self) -> Option<&bool> {
        self.scan_only_if_idle_enabled.as_ref()
    }

    /// Sets the value of ScanParameters
    pub fn set_scan_parameters(&mut self, value: u8) {
        self.scan_parameters = Some(value);
    }

    /// Gets the value of ScanParameters
    pub fn get_scan_parameters(&self) -> Option<&u8> {
        self.scan_parameters.as_ref()
    }

    /// Sets the value of ScanPurgeItemsAfterDelay
    pub fn set_scan_purge_items_after_delay(&mut self, value: u32) {
        self.scan_purge_items_after_delay = Some(value);
    }

    /// Gets the value of ScanPurgeItemsAfterDelay
    pub fn get_scan_purge_items_after_delay(&self) -> Option<&u32> {
        self.scan_purge_items_after_delay.as_ref()
    }

    /// Sets the value of ScanScheduleDay
    pub fn set_scan_schedule_day(&mut self, value: u8) {
        self.scan_schedule_day = Some(value);
    }

    /// Gets the value of ScanScheduleDay
    pub fn get_scan_schedule_day(&self) -> Option<&u8> {
        self.scan_schedule_day.as_ref()
    }

    /// Sets the value of ScanScheduleOffset
    pub fn set_scan_schedule_offset(&mut self, value: u32) {
        self.scan_schedule_offset = Some(value);
    }

    /// Gets the value of ScanScheduleOffset
    pub fn get_scan_schedule_offset(&self) -> Option<&u32> {
        self.scan_schedule_offset.as_ref()
    }

    /// Sets the value of ScanScheduleQuickScanTime
    pub fn set_scan_schedule_quick_scan_time(&mut self, value: String) {
        self.scan_schedule_quick_scan_time = Some(value);
    }

    /// Gets the value of ScanScheduleQuickScanTime
    pub fn get_scan_schedule_quick_scan_time(&self) -> Option<&String> {
        self.scan_schedule_quick_scan_time.as_ref()
    }

    /// Sets the value of ScanScheduleTime
    pub fn set_scan_schedule_time(&mut self, value: String) {
        self.scan_schedule_time = Some(value);
    }

    /// Gets the value of ScanScheduleTime
    pub fn get_scan_schedule_time(&self) -> Option<&String> {
        self.scan_schedule_time.as_ref()
    }

    /// Sets the value of SchedulerRandomizationTime
    pub fn set_scheduler_randomization_time(&mut self, value: u32) {
        self.scheduler_randomization_time = Some(value);
    }

    /// Gets the value of SchedulerRandomizationTime
    pub fn get_scheduler_randomization_time(&self) -> Option<&u32> {
        self.scheduler_randomization_time.as_ref()
    }

    /// Sets the value of ServiceHealthReportInterval
    pub fn set_service_health_report_interval(&mut self, value: u32) {
        self.service_health_report_interval = Some(value);
    }

    /// Gets the value of ServiceHealthReportInterval
    pub fn get_service_health_report_interval(&self) -> Option<&u32> {
        self.service_health_report_interval.as_ref()
    }

    /// Sets the value of SevereThreatDefaultAction
    pub fn set_severe_threat_default_action(&mut self, value: u8) {
        self.severe_threat_default_action = Some(value);
    }

    /// Gets the value of SevereThreatDefaultAction
    pub fn get_severe_threat_default_action(&self) -> Option<&u8> {
        self.severe_threat_default_action.as_ref()
    }

    /// Sets the value of SharedSignaturesPath
    pub fn set_shared_signatures_path(&mut self, value: String) {
        self.shared_signatures_path = Some(value);
    }

    /// Gets the value of SharedSignaturesPath
    pub fn get_shared_signatures_path(&self) -> Option<&String> {
        self.shared_signatures_path.as_ref()
    }

    /// Sets the value of SharedSignaturesPathUpdateAtScheduledTimeOnly
    pub fn set_shared_signatures_path_update_at_scheduled_time_only(&mut self, value: bool) {
        self.shared_signatures_path_update_at_scheduled_time_only = Some(value);
    }

    /// Gets the value of SharedSignaturesPathUpdateAtScheduledTimeOnly
    pub fn get_shared_signatures_path_update_at_scheduled_time_only(&self) -> Option<&bool> {
        self.shared_signatures_path_update_at_scheduled_time_only.as_ref()
    }

    /// Sets the value of SignatureAuGracePeriod
    pub fn set_signature_au_grace_period(&mut self, value: u32) {
        self.signature_au_grace_period = Some(value);
    }

    /// Gets the value of SignatureAuGracePeriod
    pub fn get_signature_au_grace_period(&self) -> Option<&u32> {
        self.signature_au_grace_period.as_ref()
    }

    /// Sets the value of SignatureBlobFileSharesSources
    pub fn set_signature_blob_file_shares_sources(&mut self, value: String) {
        self.signature_blob_file_shares_sources = Some(value);
    }

    /// Gets the value of SignatureBlobFileSharesSources
    pub fn get_signature_blob_file_shares_sources(&self) -> Option<&String> {
        self.signature_blob_file_shares_sources.as_ref()
    }

    /// Sets the value of SignatureBlobUpdateInterval
    pub fn set_signature_blob_update_interval(&mut self, value: u32) {
        self.signature_blob_update_interval = Some(value);
    }

    /// Gets the value of SignatureBlobUpdateInterval
    pub fn get_signature_blob_update_interval(&self) -> Option<&u32> {
        self.signature_blob_update_interval.as_ref()
    }

    /// Sets the value of SignatureDefinitionUpdateFileSharesSources
    pub fn set_signature_definition_update_file_shares_sources(&mut self, value: String) {
        self.signature_definition_update_file_shares_sources = Some(value);
    }

    /// Gets the value of SignatureDefinitionUpdateFileSharesSources
    pub fn get_signature_definition_update_file_shares_sources(&self) -> Option<&String> {
        self.signature_definition_update_file_shares_sources.as_ref()
    }

    /// Sets the value of SignatureDisableUpdateOnStartupWithoutEngine
    pub fn set_signature_disable_update_on_startup_without_engine(&mut self, value: bool) {
        self.signature_disable_update_on_startup_without_engine = Some(value);
    }

    /// Gets the value of SignatureDisableUpdateOnStartupWithoutEngine
    pub fn get_signature_disable_update_on_startup_without_engine(&self) -> Option<&bool> {
        self.signature_disable_update_on_startup_without_engine.as_ref()
    }

    /// Sets the value of SignatureFallbackOrder
    pub fn set_signature_fallback_order(&mut self, value: String) {
        self.signature_fallback_order = Some(value);
    }

    /// Gets the value of SignatureFallbackOrder
    pub fn get_signature_fallback_order(&self) -> Option<&String> {
        self.signature_fallback_order.as_ref()
    }

    /// Sets the value of SignatureFirstAuGracePeriod
    pub fn set_signature_first_au_grace_period(&mut self, value: u32) {
        self.signature_first_au_grace_period = Some(value);
    }

    /// Gets the value of SignatureFirstAuGracePeriod
    pub fn get_signature_first_au_grace_period(&self) -> Option<&u32> {
        self.signature_first_au_grace_period.as_ref()
    }

    /// Sets the value of SignatureScheduleDay
    pub fn set_signature_schedule_day(&mut self, value: u8) {
        self.signature_schedule_day = Some(value);
    }

    /// Gets the value of SignatureScheduleDay
    pub fn get_signature_schedule_day(&self) -> Option<&u8> {
        self.signature_schedule_day.as_ref()
    }

    /// Sets the value of SignatureScheduleTime
    pub fn set_signature_schedule_time(&mut self, value: String) {
        self.signature_schedule_time = Some(value);
    }

    /// Gets the value of SignatureScheduleTime
    pub fn get_signature_schedule_time(&self) -> Option<&String> {
        self.signature_schedule_time.as_ref()
    }

    /// Sets the value of SignatureUpdateCatchupInterval
    pub fn set_signature_update_catchup_interval(&mut self, value: u32) {
        self.signature_update_catchup_interval = Some(value);
    }

    /// Gets the value of SignatureUpdateCatchupInterval
    pub fn get_signature_update_catchup_interval(&self) -> Option<&u32> {
        self.signature_update_catchup_interval.as_ref()
    }

    /// Sets the value of SignatureUpdateInterval
    pub fn set_signature_update_interval(&mut self, value: u32) {
        self.signature_update_interval = Some(value);
    }

    /// Gets the value of SignatureUpdateInterval
    pub fn get_signature_update_interval(&self) -> Option<&u32> {
        self.signature_update_interval.as_ref()
    }

    /// Sets the value of SubmitSamplesConsent
    pub fn set_submit_samples_consent(&mut self, value: u8) {
        self.submit_samples_consent = Some(value);
    }

    /// Gets the value of SubmitSamplesConsent
    pub fn get_submit_samples_consent(&self) -> Option<&u8> {
        self.submit_samples_consent.as_ref()
    }

    /// Sets the value of ThreatIDDefaultAction_Actions
    pub fn set_threat_iddefault_action__actions(&mut self, value: Vec<u8>) {
        self.threat_iddefault_action__actions = value;
    }

    /// Gets the value of ThreatIDDefaultAction_Actions
    pub fn get_threat_iddefault_action__actions(&self) -> &Vec<u8> {
        &self.threat_iddefault_action__actions
    }

    /// Sets the value of ThreatIDDefaultAction_Ids
    pub fn set_threat_iddefault_action__ids(&mut self, value: Vec<i64>) {
        self.threat_iddefault_action__ids = value;
    }

    /// Gets the value of ThreatIDDefaultAction_Ids
    pub fn get_threat_iddefault_action__ids(&self) -> &Vec<i64> {
        &self.threat_iddefault_action__ids
    }

    /// Sets the value of ThrottleForScheduledScanOnly
    pub fn set_throttle_for_scheduled_scan_only(&mut self, value: bool) {
        self.throttle_for_scheduled_scan_only = Some(value);
    }

    /// Gets the value of ThrottleForScheduledScanOnly
    pub fn get_throttle_for_scheduled_scan_only(&self) -> Option<&bool> {
        self.throttle_for_scheduled_scan_only.as_ref()
    }

    /// Sets the value of TrustLabelProtectionStatus
    pub fn set_trust_label_protection_status(&mut self, value: u32) {
        self.trust_label_protection_status = Some(value);
    }

    /// Gets the value of TrustLabelProtectionStatus
    pub fn get_trust_label_protection_status(&self) -> Option<&u32> {
        self.trust_label_protection_status.as_ref()
    }

    /// Sets the value of UILockdown
    pub fn set_uilockdown(&mut self, value: bool) {
        self.uilockdown = Some(value);
    }

    /// Gets the value of UILockdown
    pub fn get_uilockdown(&self) -> Option<&bool> {
        self.uilockdown.as_ref()
    }

    /// Sets the value of UnknownThreatDefaultAction
    pub fn set_unknown_threat_default_action(&mut self, value: u8) {
        self.unknown_threat_default_action = Some(value);
    }

    /// Gets the value of UnknownThreatDefaultAction
    pub fn get_unknown_threat_default_action(&self) -> Option<&u8> {
        self.unknown_threat_default_action.as_ref()
    }

/// 

    /// * `allow_datagram_processing_on_win_server` -  (bool)
    /// * `allow_network_protection_down_level` -  (bool)
    /// * `allow_network_protection_on_win_server` -  (bool)
    /// * `allow_switch_to_async_inspection` -  (bool)
    /// * `apply_disable_network_scanning_to_ioav` -  (bool)
    /// * `attack_surface_reduction_only_exclusions` -  (String[])
    /// * `attack_surface_reduction_rules__actions` -  (u8[])
    /// * `attack_surface_reduction_rules__ids` -  (String[])
    /// * `attack_surface_reduction_rules__rule_specific_exclusions` -  (String[])
    /// * `attack_surface_reduction_rules__rule_specific_exclusions__id` -  (String[])
    /// * `brute_force_protection_aggressiveness` -  (u8)
    /// * `brute_force_protection_configured_state` -  (u8)
    /// * `brute_force_protection_exclusions` -  (String[])
    /// * `brute_force_protection_local_network_blocking` -  (bool)
    /// * `brute_force_protection_max_block_time` -  (u32)
    /// * `brute_force_protection_skip_learning_period` -  (bool)
    /// * `check_for_signatures_before_running_scan` -  (bool)
    /// * `cloud_block_level` -  (u8)
    /// * `cloud_extended_timeout` -  (u32)
    /// * `controlled_folder_access_allowed_applications` -  (String[])
    /// * `controlled_folder_access_protected_folders` -  (String[])
    /// * `definition_updates_channel` -  (u8)
    /// * `disable_archive_scanning` -  (bool)
    /// * `disable_auto_exclusions` -  (bool)
    /// * `disable_behavior_monitoring` -  (bool)
    /// * `disable_block_at_first_seen` -  (bool)
    /// * `disable_cache_maintenance` -  (bool)
    /// * `disable_catchup_full_scan` -  (bool)
    /// * `disable_catchup_quick_scan` -  (bool)
    /// * `disable_core_service_ecsintegration` -  (bool)
    /// * `disable_core_service_telemetry` -  (bool)
    /// * `disable_cpu_throttle_on_idle_scans` -  (bool)
    /// * `disable_datagram_processing` -  (bool)
    /// * `disable_dns_over_tcp_parsing` -  (bool)
    /// * `disable_dns_parsing` -  (bool)
    /// * `disable_email_scanning` -  (bool)
    /// * `disable_ftp_parsing` -  (bool)
    /// * `disable_gradual_release` -  (bool)
    /// * `disable_http_parsing` -  (bool)
    /// * `disable_inbound_connection_filtering` -  (bool)
    /// * `disable_intrusion_prevention_system` -  (bool)
    /// * `disable_ioavprotection` -  (bool)
    /// * `disable_network_protection_perf_telemetry` -  (bool)
    /// * `disable_privacy_mode` -  (bool)
    /// * `disable_quic_parsing` -  (bool)
    /// * `disable_rdp_parsing` -  (bool)
    /// * `disable_realtime_monitoring` -  (bool)
    /// * `disable_removable_drive_scanning` -  (bool)
    /// * `disable_restore_point` -  (bool)
    /// * `disable_scanning_mapped_network_drives_for_full_scan` -  (bool)
    /// * `disable_scanning_network_files` -  (bool)
    /// * `disable_script_scanning` -  (bool)
    /// * `disable_smtp_parsing` -  (bool)
    /// * `disable_ssh_parsing` -  (bool)
    /// * `disable_tamper_protection` -  (bool)
    /// * `disable_tdtfeature` -  (bool)
    /// * `disable_tls_parsing` -  (bool)
    /// * `enable_controlled_folder_access` -  (u8)
    /// * `enable_convert_warn_to_block` -  (bool)
    /// * `enable_dns_sinkhole` -  (bool)
    /// * `enable_file_hash_computation` -  (bool)
    /// * `enable_full_scan_on_battery_power` -  (bool)
    /// * `enable_low_cpu_priority` -  (bool)
    /// * `enable_network_protection` -  (u8)
    /// * `enable_udp_receive_offload` -  (bool)
    /// * `enable_udp_segmentation_offload` -  (bool)
    /// * `engine_updates_channel` -  (u8)
    /// * `exclusion_extension` -  (String[])
    /// * `exclusion_ip_address` -  (String[])
    /// * `exclusion_path` -  (String[])
    /// * `exclusion_process` -  (String[])
    /// * `force` -  (bool)
    /// * `force_use_proxy_only` -  (bool)
    /// * `high_threat_default_action` -  (u8)
    /// * `intel_tdtenabled` -  (bool)
    /// * `low_threat_default_action` -  (u8)
    /// * `mapsreporting` -  (u8)
    /// * `metered_connection_updates` -  (bool)
    /// * `moderate_threat_default_action` -  (u8)
    /// * `network_protection_reputation_mode` -  (u32)
    /// * `oobe_enable_rtp_and_sig_update` -  (bool)
    /// * `performance_mode_status` -  (u8)
    /// * `platform_updates_channel` -  (u8)
    /// * `proxy_bypass` -  (String[])
    /// * `proxy_pac_url` -  (String)
    /// * `proxy_server` -  (String)
    /// * `puaprotection` -  (u8)
    /// * `quarantine_purge_items_after_delay` -  (u32)
    /// * `quick_scan_include_exclusions` -  (u8)
    /// * `randomize_schedule_task_times` -  (bool)
    /// * `real_time_scan_direction` -  (u8)
    /// * `remediation_schedule_day` -  (u8)
    /// * `remediation_schedule_time` -  (String)
    /// * `remote_encryption_protection_aggressiveness` -  (u8)
    /// * `remote_encryption_protection_configured_state` -  (u8)
    /// * `remote_encryption_protection_exclusions` -  (String[])
    /// * `remote_encryption_protection_max_block_time` -  (u32)
    /// * `remove_scanning_thread_pool_cap` -  (bool)
    /// * `report_dynamic_signature_dropped_event` -  (bool)
    /// * `reporting_additional_action_time_out` -  (u32)
    /// * `reporting_critical_failure_time_out` -  (u32)
    /// * `reporting_non_critical_time_out` -  (u32)
    /// * `scan_avg_cpuload_factor` -  (u8)
    /// * `scan_only_if_idle_enabled` -  (bool)
    /// * `scan_parameters` -  (u8)
    /// * `scan_purge_items_after_delay` -  (u32)
    /// * `scan_schedule_day` -  (u8)
    /// * `scan_schedule_offset` -  (u32)
    /// * `scan_schedule_quick_scan_time` -  (String)
    /// * `scan_schedule_time` -  (String)
    /// * `scheduler_randomization_time` -  (u32)
    /// * `service_health_report_interval` -  (u32)
    /// * `severe_threat_default_action` -  (u8)
    /// * `shared_signatures_path` -  (String)
    /// * `shared_signatures_path_update_at_scheduled_time_only` -  (bool)
    /// * `signature_au_grace_period` -  (u32)
    /// * `signature_blob_file_shares_sources` -  (String)
    /// * `signature_blob_update_interval` -  (u32)
    /// * `signature_definition_update_file_shares_sources` -  (String)
    /// * `signature_disable_update_on_startup_without_engine` -  (bool)
    /// * `signature_fallback_order` -  (String)
    /// * `signature_first_au_grace_period` -  (u32)
    /// * `signature_schedule_day` -  (u8)
    /// * `signature_schedule_time` -  (String)
    /// * `signature_update_catchup_interval` -  (u32)
    /// * `signature_update_interval` -  (u32)
    /// * `submit_samples_consent` -  (u8)
    /// * `threat_iddefault_action__actions` -  (u8[])
    /// * `threat_iddefault_action__ids` -  (i64[])
    /// * `throttle_for_scheduled_scan_only` -  (bool)
    /// * `trust_label_protection_status` -  (u32)
    /// * `uilockdown` -  (bool)
    /// * `unknown_threat_default_action` -  (u8)

    /// * `return_value` -  (u32)
    pub fn set(&self, disable_auto_exclusions: bool, exclusion_path: &Vec<String>, exclusion_extension: &Vec<String>, exclusion_process: &Vec<String>, exclusion_ip_address: &Vec<String>, quarantine_purge_items_after_delay: u32, real_time_scan_direction: u8, remediation_schedule_day: u8, remediation_schedule_time: &String, remote_encryption_protection_configured_state: u8, remote_encryption_protection_max_block_time: u32, remote_encryption_protection_aggressiveness: u8, remote_encryption_protection_exclusions: &Vec<String>, brute_force_protection_configured_state: u8, brute_force_protection_max_block_time: u32, brute_force_protection_aggressiveness: u8, brute_force_protection_exclusions: &Vec<String>, brute_force_protection_local_network_blocking: bool, brute_force_protection_skip_learning_period: bool, reporting_additional_action_time_out: u32, reporting_critical_failure_time_out: u32, reporting_non_critical_time_out: u32, service_health_report_interval: u32, report_dynamic_signature_dropped_event: bool, scan_avg_cpuload_factor: u8, check_for_signatures_before_running_scan: bool, scan_purge_items_after_delay: u32, scan_only_if_idle_enabled: bool, scan_parameters: u8, scan_schedule_day: u8, scan_schedule_quick_scan_time: &String, scan_schedule_time: &String, throttle_for_scheduled_scan_only: bool, signature_first_au_grace_period: u32, signature_au_grace_period: u32, signature_definition_update_file_shares_sources: &String, signature_disable_update_on_startup_without_engine: bool, signature_fallback_order: &String, signature_schedule_day: u8, signature_schedule_time: &String, signature_update_catchup_interval: u32, signature_blob_file_shares_sources: &String, signature_update_interval: u32, signature_blob_update_interval: u32, mapsreporting: u8, submit_samples_consent: u8, disable_privacy_mode: bool, randomize_schedule_task_times: bool, scheduler_randomization_time: u32, disable_behavior_monitoring: bool, disable_intrusion_prevention_system: bool, disable_ioavprotection: bool, disable_realtime_monitoring: bool, disable_script_scanning: bool, disable_archive_scanning: bool, disable_catchup_full_scan: bool, disable_catchup_quick_scan: bool, disable_email_scanning: bool, disable_removable_drive_scanning: bool, disable_restore_point: bool, disable_scanning_mapped_network_drives_for_full_scan: bool, disable_scanning_network_files: bool, apply_disable_network_scanning_to_ioav: bool, uilockdown: bool, threat_iddefault_action__ids: &Vec<i64>, threat_iddefault_action__actions: &Vec<u8>, unknown_threat_default_action: u8, low_threat_default_action: u8, moderate_threat_default_action: u8, high_threat_default_action: u8, severe_threat_default_action: u8, puaprotection: u8, disable_block_at_first_seen: bool, cloud_block_level: u8, cloud_extended_timeout: u32, enable_network_protection: u8, enable_controlled_folder_access: u8, attack_surface_reduction_only_exclusions: &Vec<String>, attack_surface_reduction_rules__ids: &Vec<String>, attack_surface_reduction_rules__actions: &Vec<u8>, controlled_folder_access_allowed_applications: &Vec<String>, controlled_folder_access_protected_folders: &Vec<String>, shared_signatures_path: &String, shared_signatures_path_update_at_scheduled_time_only: bool, enable_low_cpu_priority: bool, enable_file_hash_computation: bool, metered_connection_updates: bool, allow_network_protection_on_win_server: bool, disable_datagram_processing: bool, enable_convert_warn_to_block: bool, disable_cpu_throttle_on_idle_scans: bool, enable_full_scan_on_battery_power: bool, proxy_pac_url: &String, proxy_server: &String, proxy_bypass: &Vec<String>, force_use_proxy_only: bool, disable_tls_parsing: bool, disable_http_parsing: bool, disable_dns_parsing: bool, disable_dns_over_tcp_parsing: bool, disable_ssh_parsing: bool, platform_updates_channel: u8, engine_updates_channel: u8, definition_updates_channel: u8, disable_gradual_release: bool, allow_network_protection_down_level: bool, allow_datagram_processing_on_win_server: bool, enable_dns_sinkhole: bool, disable_inbound_connection_filtering: bool, disable_rdp_parsing: bool, disable_network_protection_perf_telemetry: bool, trust_label_protection_status: u32, disable_ftp_parsing: bool, allow_switch_to_async_inspection: bool, scan_schedule_offset: u32, disable_tdtfeature: bool, disable_tamper_protection: bool, disable_smtp_parsing: bool, disable_quic_parsing: bool, network_protection_reputation_mode: u32, intel_tdtenabled: bool, attack_surface_reduction_rules__rule_specific_exclusions__id: &Vec<String>, attack_surface_reduction_rules__rule_specific_exclusions: &Vec<String>, oobe_enable_rtp_and_sig_update: bool, performance_mode_status: u8, quick_scan_include_exclusions: u8, remove_scanning_thread_pool_cap: bool, disable_cache_maintenance: bool, disable_core_service_ecsintegration: bool, disable_core_service_telemetry: bool, enable_udp_segmentation_offload: bool, enable_udp_receive_offload: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisableAutoExclusions".to_string(), value: disable_auto_exclusions.into() });
        args.push(MethodParameter { name: "ExclusionPath".to_string(), value: exclusion_path.into() });
        args.push(MethodParameter { name: "ExclusionExtension".to_string(), value: exclusion_extension.into() });
        args.push(MethodParameter { name: "ExclusionProcess".to_string(), value: exclusion_process.into() });
        args.push(MethodParameter { name: "ExclusionIpAddress".to_string(), value: exclusion_ip_address.into() });
        args.push(MethodParameter { name: "QuarantinePurgeItemsAfterDelay".to_string(), value: quarantine_purge_items_after_delay.into() });
        args.push(MethodParameter { name: "RealTimeScanDirection".to_string(), value: real_time_scan_direction.into() });
        args.push(MethodParameter { name: "RemediationScheduleDay".to_string(), value: remediation_schedule_day.into() });
        args.push(MethodParameter { name: "RemediationScheduleTime".to_string(), value: remediation_schedule_time.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionConfiguredState".to_string(), value: remote_encryption_protection_configured_state.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionMaxBlockTime".to_string(), value: remote_encryption_protection_max_block_time.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionAggressiveness".to_string(), value: remote_encryption_protection_aggressiveness.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionExclusions".to_string(), value: remote_encryption_protection_exclusions.into() });
        args.push(MethodParameter { name: "BruteForceProtectionConfiguredState".to_string(), value: brute_force_protection_configured_state.into() });
        args.push(MethodParameter { name: "BruteForceProtectionMaxBlockTime".to_string(), value: brute_force_protection_max_block_time.into() });
        args.push(MethodParameter { name: "BruteForceProtectionAggressiveness".to_string(), value: brute_force_protection_aggressiveness.into() });
        args.push(MethodParameter { name: "BruteForceProtectionExclusions".to_string(), value: brute_force_protection_exclusions.into() });
        args.push(MethodParameter { name: "BruteForceProtectionLocalNetworkBlocking".to_string(), value: brute_force_protection_local_network_blocking.into() });
        args.push(MethodParameter { name: "BruteForceProtectionSkipLearningPeriod".to_string(), value: brute_force_protection_skip_learning_period.into() });
        args.push(MethodParameter { name: "ReportingAdditionalActionTimeOut".to_string(), value: reporting_additional_action_time_out.into() });
        args.push(MethodParameter { name: "ReportingCriticalFailureTimeOut".to_string(), value: reporting_critical_failure_time_out.into() });
        args.push(MethodParameter { name: "ReportingNonCriticalTimeOut".to_string(), value: reporting_non_critical_time_out.into() });
        args.push(MethodParameter { name: "ServiceHealthReportInterval".to_string(), value: service_health_report_interval.into() });
        args.push(MethodParameter { name: "ReportDynamicSignatureDroppedEvent".to_string(), value: report_dynamic_signature_dropped_event.into() });
        args.push(MethodParameter { name: "ScanAvgCPULoadFactor".to_string(), value: scan_avg_cpuload_factor.into() });
        args.push(MethodParameter { name: "CheckForSignaturesBeforeRunningScan".to_string(), value: check_for_signatures_before_running_scan.into() });
        args.push(MethodParameter { name: "ScanPurgeItemsAfterDelay".to_string(), value: scan_purge_items_after_delay.into() });
        args.push(MethodParameter { name: "ScanOnlyIfIdleEnabled".to_string(), value: scan_only_if_idle_enabled.into() });
        args.push(MethodParameter { name: "ScanParameters".to_string(), value: scan_parameters.into() });
        args.push(MethodParameter { name: "ScanScheduleDay".to_string(), value: scan_schedule_day.into() });
        args.push(MethodParameter { name: "ScanScheduleQuickScanTime".to_string(), value: scan_schedule_quick_scan_time.into() });
        args.push(MethodParameter { name: "ScanScheduleTime".to_string(), value: scan_schedule_time.into() });
        args.push(MethodParameter { name: "ThrottleForScheduledScanOnly".to_string(), value: throttle_for_scheduled_scan_only.into() });
        args.push(MethodParameter { name: "SignatureFirstAuGracePeriod".to_string(), value: signature_first_au_grace_period.into() });
        args.push(MethodParameter { name: "SignatureAuGracePeriod".to_string(), value: signature_au_grace_period.into() });
        args.push(MethodParameter { name: "SignatureDefinitionUpdateFileSharesSources".to_string(), value: signature_definition_update_file_shares_sources.into() });
        args.push(MethodParameter { name: "SignatureDisableUpdateOnStartupWithoutEngine".to_string(), value: signature_disable_update_on_startup_without_engine.into() });
        args.push(MethodParameter { name: "SignatureFallbackOrder".to_string(), value: signature_fallback_order.into() });
        args.push(MethodParameter { name: "SignatureScheduleDay".to_string(), value: signature_schedule_day.into() });
        args.push(MethodParameter { name: "SignatureScheduleTime".to_string(), value: signature_schedule_time.into() });
        args.push(MethodParameter { name: "SignatureUpdateCatchupInterval".to_string(), value: signature_update_catchup_interval.into() });
        args.push(MethodParameter { name: "SignatureBlobFileSharesSources".to_string(), value: signature_blob_file_shares_sources.into() });
        args.push(MethodParameter { name: "SignatureUpdateInterval".to_string(), value: signature_update_interval.into() });
        args.push(MethodParameter { name: "SignatureBlobUpdateInterval".to_string(), value: signature_blob_update_interval.into() });
        args.push(MethodParameter { name: "MAPSReporting".to_string(), value: mapsreporting.into() });
        args.push(MethodParameter { name: "SubmitSamplesConsent".to_string(), value: submit_samples_consent.into() });
        args.push(MethodParameter { name: "DisablePrivacyMode".to_string(), value: disable_privacy_mode.into() });
        args.push(MethodParameter { name: "RandomizeScheduleTaskTimes".to_string(), value: randomize_schedule_task_times.into() });
        args.push(MethodParameter { name: "SchedulerRandomizationTime".to_string(), value: scheduler_randomization_time.into() });
        args.push(MethodParameter { name: "DisableBehaviorMonitoring".to_string(), value: disable_behavior_monitoring.into() });
        args.push(MethodParameter { name: "DisableIntrusionPreventionSystem".to_string(), value: disable_intrusion_prevention_system.into() });
        args.push(MethodParameter { name: "DisableIOAVProtection".to_string(), value: disable_ioavprotection.into() });
        args.push(MethodParameter { name: "DisableRealtimeMonitoring".to_string(), value: disable_realtime_monitoring.into() });
        args.push(MethodParameter { name: "DisableScriptScanning".to_string(), value: disable_script_scanning.into() });
        args.push(MethodParameter { name: "DisableArchiveScanning".to_string(), value: disable_archive_scanning.into() });
        args.push(MethodParameter { name: "DisableCatchupFullScan".to_string(), value: disable_catchup_full_scan.into() });
        args.push(MethodParameter { name: "DisableCatchupQuickScan".to_string(), value: disable_catchup_quick_scan.into() });
        args.push(MethodParameter { name: "DisableEmailScanning".to_string(), value: disable_email_scanning.into() });
        args.push(MethodParameter { name: "DisableRemovableDriveScanning".to_string(), value: disable_removable_drive_scanning.into() });
        args.push(MethodParameter { name: "DisableRestorePoint".to_string(), value: disable_restore_point.into() });
        args.push(MethodParameter { name: "DisableScanningMappedNetworkDrivesForFullScan".to_string(), value: disable_scanning_mapped_network_drives_for_full_scan.into() });
        args.push(MethodParameter { name: "DisableScanningNetworkFiles".to_string(), value: disable_scanning_network_files.into() });
        args.push(MethodParameter { name: "ApplyDisableNetworkScanningToIOAV".to_string(), value: apply_disable_network_scanning_to_ioav.into() });
        args.push(MethodParameter { name: "UILockdown".to_string(), value: uilockdown.into() });
        args.push(MethodParameter { name: "ThreatIDDefaultAction_Ids".to_string(), value: threat_iddefault_action__ids.into() });
        args.push(MethodParameter { name: "ThreatIDDefaultAction_Actions".to_string(), value: threat_iddefault_action__actions.into() });
        args.push(MethodParameter { name: "UnknownThreatDefaultAction".to_string(), value: unknown_threat_default_action.into() });
        args.push(MethodParameter { name: "LowThreatDefaultAction".to_string(), value: low_threat_default_action.into() });
        args.push(MethodParameter { name: "ModerateThreatDefaultAction".to_string(), value: moderate_threat_default_action.into() });
        args.push(MethodParameter { name: "HighThreatDefaultAction".to_string(), value: high_threat_default_action.into() });
        args.push(MethodParameter { name: "SevereThreatDefaultAction".to_string(), value: severe_threat_default_action.into() });
        args.push(MethodParameter { name: "PUAProtection".to_string(), value: puaprotection.into() });
        args.push(MethodParameter { name: "DisableBlockAtFirstSeen".to_string(), value: disable_block_at_first_seen.into() });
        args.push(MethodParameter { name: "CloudBlockLevel".to_string(), value: cloud_block_level.into() });
        args.push(MethodParameter { name: "CloudExtendedTimeout".to_string(), value: cloud_extended_timeout.into() });
        args.push(MethodParameter { name: "EnableNetworkProtection".to_string(), value: enable_network_protection.into() });
        args.push(MethodParameter { name: "EnableControlledFolderAccess".to_string(), value: enable_controlled_folder_access.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionOnlyExclusions".to_string(), value: attack_surface_reduction_only_exclusions.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_Ids".to_string(), value: attack_surface_reduction_rules__ids.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_Actions".to_string(), value: attack_surface_reduction_rules__actions.into() });
        args.push(MethodParameter { name: "ControlledFolderAccessAllowedApplications".to_string(), value: controlled_folder_access_allowed_applications.into() });
        args.push(MethodParameter { name: "ControlledFolderAccessProtectedFolders".to_string(), value: controlled_folder_access_protected_folders.into() });
        args.push(MethodParameter { name: "SharedSignaturesPath".to_string(), value: shared_signatures_path.into() });
        args.push(MethodParameter { name: "SharedSignaturesPathUpdateAtScheduledTimeOnly".to_string(), value: shared_signatures_path_update_at_scheduled_time_only.into() });
        args.push(MethodParameter { name: "EnableLowCpuPriority".to_string(), value: enable_low_cpu_priority.into() });
        args.push(MethodParameter { name: "EnableFileHashComputation".to_string(), value: enable_file_hash_computation.into() });
        args.push(MethodParameter { name: "MeteredConnectionUpdates".to_string(), value: metered_connection_updates.into() });
        args.push(MethodParameter { name: "AllowNetworkProtectionOnWinServer".to_string(), value: allow_network_protection_on_win_server.into() });
        args.push(MethodParameter { name: "DisableDatagramProcessing".to_string(), value: disable_datagram_processing.into() });
        args.push(MethodParameter { name: "EnableConvertWarnToBlock".to_string(), value: enable_convert_warn_to_block.into() });
        args.push(MethodParameter { name: "DisableCpuThrottleOnIdleScans".to_string(), value: disable_cpu_throttle_on_idle_scans.into() });
        args.push(MethodParameter { name: "EnableFullScanOnBatteryPower".to_string(), value: enable_full_scan_on_battery_power.into() });
        args.push(MethodParameter { name: "ProxyPacUrl".to_string(), value: proxy_pac_url.into() });
        args.push(MethodParameter { name: "ProxyServer".to_string(), value: proxy_server.into() });
        args.push(MethodParameter { name: "ProxyBypass".to_string(), value: proxy_bypass.into() });
        args.push(MethodParameter { name: "ForceUseProxyOnly".to_string(), value: force_use_proxy_only.into() });
        args.push(MethodParameter { name: "DisableTlsParsing".to_string(), value: disable_tls_parsing.into() });
        args.push(MethodParameter { name: "DisableHttpParsing".to_string(), value: disable_http_parsing.into() });
        args.push(MethodParameter { name: "DisableDnsParsing".to_string(), value: disable_dns_parsing.into() });
        args.push(MethodParameter { name: "DisableDnsOverTcpParsing".to_string(), value: disable_dns_over_tcp_parsing.into() });
        args.push(MethodParameter { name: "DisableSshParsing".to_string(), value: disable_ssh_parsing.into() });
        args.push(MethodParameter { name: "PlatformUpdatesChannel".to_string(), value: platform_updates_channel.into() });
        args.push(MethodParameter { name: "EngineUpdatesChannel".to_string(), value: engine_updates_channel.into() });
        args.push(MethodParameter { name: "DefinitionUpdatesChannel".to_string(), value: definition_updates_channel.into() });
        args.push(MethodParameter { name: "DisableGradualRelease".to_string(), value: disable_gradual_release.into() });
        args.push(MethodParameter { name: "AllowNetworkProtectionDownLevel".to_string(), value: allow_network_protection_down_level.into() });
        args.push(MethodParameter { name: "AllowDatagramProcessingOnWinServer".to_string(), value: allow_datagram_processing_on_win_server.into() });
        args.push(MethodParameter { name: "EnableDnsSinkhole".to_string(), value: enable_dns_sinkhole.into() });
        args.push(MethodParameter { name: "DisableInboundConnectionFiltering".to_string(), value: disable_inbound_connection_filtering.into() });
        args.push(MethodParameter { name: "DisableRdpParsing".to_string(), value: disable_rdp_parsing.into() });
        args.push(MethodParameter { name: "DisableNetworkProtectionPerfTelemetry".to_string(), value: disable_network_protection_perf_telemetry.into() });
        args.push(MethodParameter { name: "TrustLabelProtectionStatus".to_string(), value: trust_label_protection_status.into() });
        args.push(MethodParameter { name: "DisableFtpParsing".to_string(), value: disable_ftp_parsing.into() });
        args.push(MethodParameter { name: "AllowSwitchToAsyncInspection".to_string(), value: allow_switch_to_async_inspection.into() });
        args.push(MethodParameter { name: "ScanScheduleOffset".to_string(), value: scan_schedule_offset.into() });
        args.push(MethodParameter { name: "DisableTDTFeature".to_string(), value: disable_tdtfeature.into() });
        args.push(MethodParameter { name: "DisableTamperProtection".to_string(), value: disable_tamper_protection.into() });
        args.push(MethodParameter { name: "DisableSmtpParsing".to_string(), value: disable_smtp_parsing.into() });
        args.push(MethodParameter { name: "DisableQuicParsing".to_string(), value: disable_quic_parsing.into() });
        args.push(MethodParameter { name: "NetworkProtectionReputationMode".to_string(), value: network_protection_reputation_mode.into() });
        args.push(MethodParameter { name: "IntelTDTEnabled".to_string(), value: intel_tdtenabled.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_RuleSpecificExclusions_Id".to_string(), value: attack_surface_reduction_rules__rule_specific_exclusions__id.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_RuleSpecificExclusions".to_string(), value: attack_surface_reduction_rules__rule_specific_exclusions.into() });
        args.push(MethodParameter { name: "OobeEnableRtpAndSigUpdate".to_string(), value: oobe_enable_rtp_and_sig_update.into() });
        args.push(MethodParameter { name: "PerformanceModeStatus".to_string(), value: performance_mode_status.into() });
        args.push(MethodParameter { name: "QuickScanIncludeExclusions".to_string(), value: quick_scan_include_exclusions.into() });
        args.push(MethodParameter { name: "RemoveScanningThreadPoolCap".to_string(), value: remove_scanning_thread_pool_cap.into() });
        args.push(MethodParameter { name: "DisableCacheMaintenance".to_string(), value: disable_cache_maintenance.into() });
        args.push(MethodParameter { name: "DisableCoreServiceECSIntegration".to_string(), value: disable_core_service_ecsintegration.into() });
        args.push(MethodParameter { name: "DisableCoreServiceTelemetry".to_string(), value: disable_core_service_telemetry.into() });
        args.push(MethodParameter { name: "EnableUdpSegmentationOffload".to_string(), value: enable_udp_segmentation_offload.into() });
        args.push(MethodParameter { name: "EnableUdpReceiveOffload".to_string(), value: enable_udp_receive_offload.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Set", &args)

    }


/// 

    /// * `allow_datagram_processing_on_win_server` -  (bool)
    /// * `allow_network_protection_down_level` -  (bool)
    /// * `allow_network_protection_on_win_server` -  (bool)
    /// * `allow_switch_to_async_inspection` -  (bool)
    /// * `apply_disable_network_scanning_to_ioav` -  (bool)
    /// * `attack_surface_reduction_only_exclusions` -  (String[])
    /// * `attack_surface_reduction_rules__actions` -  (u8[])
    /// * `attack_surface_reduction_rules__ids` -  (String[])
    /// * `attack_surface_reduction_rules__rule_specific_exclusions` -  (String)
    /// * `attack_surface_reduction_rules__rule_specific_exclusions__id` -  (String)
    /// * `brute_force_protection_aggressiveness` -  (bool)
    /// * `brute_force_protection_configured_state` -  (bool)
    /// * `brute_force_protection_exclusions` -  (String[])
    /// * `brute_force_protection_local_network_blocking` -  (bool)
    /// * `brute_force_protection_max_block_time` -  (bool)
    /// * `brute_force_protection_skip_learning_period` -  (bool)
    /// * `check_for_signatures_before_running_scan` -  (bool)
    /// * `cloud_block_level` -  (bool)
    /// * `cloud_extended_timeout` -  (bool)
    /// * `controlled_folder_access_allowed_applications` -  (String[])
    /// * `controlled_folder_access_protected_folders` -  (String[])
    /// * `definition_updates_channel` -  (bool)
    /// * `disable_archive_scanning` -  (bool)
    /// * `disable_auto_exclusions` -  (bool)
    /// * `disable_behavior_monitoring` -  (bool)
    /// * `disable_block_at_first_seen` -  (bool)
    /// * `disable_cache_maintenance` -  (bool)
    /// * `disable_catchup_full_scan` -  (bool)
    /// * `disable_catchup_quick_scan` -  (bool)
    /// * `disable_core_service_ecsintegration` -  (bool)
    /// * `disable_core_service_telemetry` -  (bool)
    /// * `disable_cpu_throttle_on_idle_scans` -  (bool)
    /// * `disable_datagram_processing` -  (bool)
    /// * `disable_dns_over_tcp_parsing` -  (bool)
    /// * `disable_dns_parsing` -  (bool)
    /// * `disable_email_scanning` -  (bool)
    /// * `disable_ftp_parsing` -  (bool)
    /// * `disable_gradual_release` -  (bool)
    /// * `disable_http_parsing` -  (bool)
    /// * `disable_inbound_connection_filtering` -  (bool)
    /// * `disable_intrusion_prevention_system` -  (bool)
    /// * `disable_ioavprotection` -  (bool)
    /// * `disable_network_protection_perf_telemetry` -  (bool)
    /// * `disable_privacy_mode` -  (bool)
    /// * `disable_quic_parsing` -  (bool)
    /// * `disable_rdp_parsing` -  (bool)
    /// * `disable_realtime_monitoring` -  (bool)
    /// * `disable_removable_drive_scanning` -  (bool)
    /// * `disable_restore_point` -  (bool)
    /// * `disable_scanning_mapped_network_drives_for_full_scan` -  (bool)
    /// * `disable_scanning_network_files` -  (bool)
    /// * `disable_script_scanning` -  (bool)
    /// * `disable_smtp_parsing` -  (bool)
    /// * `disable_ssh_parsing` -  (bool)
    /// * `disable_tdtfeature` -  (bool)
    /// * `disable_tls_parsing` -  (bool)
    /// * `enable_controlled_folder_access` -  (bool)
    /// * `enable_convert_warn_to_block` -  (bool)
    /// * `enable_dns_sinkhole` -  (bool)
    /// * `enable_file_hash_computation` -  (bool)
    /// * `enable_full_scan_on_battery_power` -  (bool)
    /// * `enable_low_cpu_priority` -  (bool)
    /// * `enable_network_protection` -  (bool)
    /// * `enable_udp_receive_offload` -  (bool)
    /// * `enable_udp_segmentation_offload` -  (bool)
    /// * `engine_updates_channel` -  (bool)
    /// * `exclusion_extension` -  (String[])
    /// * `exclusion_ip_address` -  (String[])
    /// * `exclusion_path` -  (String[])
    /// * `exclusion_process` -  (String[])
    /// * `force` -  (bool)
    /// * `force_use_proxy_only` -  (bool)
    /// * `high_threat_default_action` -  (bool)
    /// * `intel_tdtenabled` -  (bool)
    /// * `low_threat_default_action` -  (bool)
    /// * `mapsreporting` -  (bool)
    /// * `metered_connection_updates` -  (bool)
    /// * `moderate_threat_default_action` -  (bool)
    /// * `network_protection_reputation_mode` -  (bool)
    /// * `oobe_enable_rtp_and_sig_update` -  (bool)
    /// * `performance_mode_status` -  (bool)
    /// * `platform_updates_channel` -  (bool)
    /// * `proxy_bypass` -  (bool)
    /// * `proxy_pac_url` -  (bool)
    /// * `proxy_server` -  (bool)
    /// * `puaprotection` -  (bool)
    /// * `quarantine_purge_items_after_delay` -  (bool)
    /// * `quick_scan_include_exclusions` -  (bool)
    /// * `randomize_schedule_task_times` -  (bool)
    /// * `real_time_scan_direction` -  (bool)
    /// * `remediation_schedule_day` -  (bool)
    /// * `remediation_schedule_time` -  (bool)
    /// * `remote_encryption_protection_aggressiveness` -  (bool)
    /// * `remote_encryption_protection_configured_state` -  (bool)
    /// * `remote_encryption_protection_exclusions` -  (String[])
    /// * `remote_encryption_protection_max_block_time` -  (bool)
    /// * `remove_scanning_thread_pool_cap` -  (bool)
    /// * `report_dynamic_signature_dropped_event` -  (bool)
    /// * `reporting_additional_action_time_out` -  (bool)
    /// * `reporting_critical_failure_time_out` -  (bool)
    /// * `reporting_non_critical_time_out` -  (bool)
    /// * `scan_avg_cpuload_factor` -  (bool)
    /// * `scan_only_if_idle_enabled` -  (bool)
    /// * `scan_parameters` -  (bool)
    /// * `scan_purge_items_after_delay` -  (bool)
    /// * `scan_schedule_day` -  (bool)
    /// * `scan_schedule_offset` -  (bool)
    /// * `scan_schedule_quick_scan_time` -  (bool)
    /// * `scan_schedule_time` -  (bool)
    /// * `scheduler_randomization_time` -  (bool)
    /// * `service_health_report_interval` -  (bool)
    /// * `severe_threat_default_action` -  (bool)
    /// * `shared_signatures_path` -  (bool)
    /// * `shared_signatures_path_update_at_scheduled_time_only` -  (bool)
    /// * `signature_au_grace_period` -  (bool)
    /// * `signature_blob_file_shares_sources` -  (bool)
    /// * `signature_blob_update_interval` -  (bool)
    /// * `signature_definition_update_file_shares_sources` -  (bool)
    /// * `signature_disable_update_on_startup_without_engine` -  (bool)
    /// * `signature_fallback_order` -  (bool)
    /// * `signature_first_au_grace_period` -  (bool)
    /// * `signature_schedule_day` -  (bool)
    /// * `signature_schedule_time` -  (bool)
    /// * `signature_update_catchup_interval` -  (bool)
    /// * `signature_update_interval` -  (bool)
    /// * `submit_samples_consent` -  (bool)
    /// * `threat_iddefault_action__actions` -  (u8[])
    /// * `threat_iddefault_action__ids` -  (i64[])
    /// * `throttle_for_scheduled_scan_only` -  (bool)
    /// * `trust_label_protection_status` -  (bool)
    /// * `uilockdown` -  (bool)
    /// * `unknown_threat_default_action` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove(&self, disable_auto_exclusions: bool, exclusion_path: &Vec<String>, exclusion_extension: &Vec<String>, exclusion_process: &Vec<String>, exclusion_ip_address: &Vec<String>, quarantine_purge_items_after_delay: bool, real_time_scan_direction: bool, remediation_schedule_day: bool, remediation_schedule_time: bool, remote_encryption_protection_configured_state: bool, remote_encryption_protection_max_block_time: bool, remote_encryption_protection_aggressiveness: bool, remote_encryption_protection_exclusions: &Vec<String>, brute_force_protection_configured_state: bool, brute_force_protection_max_block_time: bool, brute_force_protection_aggressiveness: bool, brute_force_protection_exclusions: &Vec<String>, brute_force_protection_local_network_blocking: bool, brute_force_protection_skip_learning_period: bool, reporting_additional_action_time_out: bool, reporting_critical_failure_time_out: bool, reporting_non_critical_time_out: bool, service_health_report_interval: bool, report_dynamic_signature_dropped_event: bool, scan_avg_cpuload_factor: bool, check_for_signatures_before_running_scan: bool, scan_purge_items_after_delay: bool, scan_only_if_idle_enabled: bool, scan_parameters: bool, scan_schedule_day: bool, scan_schedule_quick_scan_time: bool, scan_schedule_time: bool, throttle_for_scheduled_scan_only: bool, signature_first_au_grace_period: bool, signature_au_grace_period: bool, signature_definition_update_file_shares_sources: bool, signature_disable_update_on_startup_without_engine: bool, signature_fallback_order: bool, signature_schedule_day: bool, signature_schedule_time: bool, signature_update_catchup_interval: bool, signature_blob_file_shares_sources: bool, signature_update_interval: bool, signature_blob_update_interval: bool, mapsreporting: bool, submit_samples_consent: bool, disable_privacy_mode: bool, randomize_schedule_task_times: bool, scheduler_randomization_time: bool, disable_behavior_monitoring: bool, disable_intrusion_prevention_system: bool, disable_ioavprotection: bool, disable_realtime_monitoring: bool, disable_script_scanning: bool, disable_archive_scanning: bool, disable_catchup_full_scan: bool, disable_catchup_quick_scan: bool, disable_email_scanning: bool, disable_removable_drive_scanning: bool, disable_restore_point: bool, disable_scanning_mapped_network_drives_for_full_scan: bool, disable_scanning_network_files: bool, apply_disable_network_scanning_to_ioav: bool, uilockdown: bool, threat_iddefault_action__ids: &Vec<i64>, threat_iddefault_action__actions: &Vec<u8>, unknown_threat_default_action: bool, low_threat_default_action: bool, moderate_threat_default_action: bool, high_threat_default_action: bool, severe_threat_default_action: bool, puaprotection: bool, disable_block_at_first_seen: bool, cloud_block_level: bool, cloud_extended_timeout: bool, enable_network_protection: bool, enable_controlled_folder_access: bool, attack_surface_reduction_only_exclusions: &Vec<String>, attack_surface_reduction_rules__ids: &Vec<String>, attack_surface_reduction_rules__actions: &Vec<u8>, controlled_folder_access_allowed_applications: &Vec<String>, controlled_folder_access_protected_folders: &Vec<String>, shared_signatures_path: bool, shared_signatures_path_update_at_scheduled_time_only: bool, enable_low_cpu_priority: bool, enable_file_hash_computation: bool, metered_connection_updates: bool, allow_network_protection_on_win_server: bool, disable_datagram_processing: bool, enable_convert_warn_to_block: bool, disable_cpu_throttle_on_idle_scans: bool, enable_full_scan_on_battery_power: bool, proxy_pac_url: bool, proxy_server: bool, proxy_bypass: bool, force_use_proxy_only: bool, disable_tls_parsing: bool, disable_http_parsing: bool, disable_dns_parsing: bool, disable_dns_over_tcp_parsing: bool, disable_ssh_parsing: bool, platform_updates_channel: bool, engine_updates_channel: bool, definition_updates_channel: bool, disable_gradual_release: bool, allow_network_protection_down_level: bool, allow_datagram_processing_on_win_server: bool, enable_dns_sinkhole: bool, disable_inbound_connection_filtering: bool, disable_rdp_parsing: bool, disable_network_protection_perf_telemetry: bool, trust_label_protection_status: bool, disable_ftp_parsing: bool, allow_switch_to_async_inspection: bool, scan_schedule_offset: bool, disable_tdtfeature: bool, disable_smtp_parsing: bool, disable_quic_parsing: bool, network_protection_reputation_mode: bool, intel_tdtenabled: bool, attack_surface_reduction_rules__rule_specific_exclusions__id: &String, attack_surface_reduction_rules__rule_specific_exclusions: &String, oobe_enable_rtp_and_sig_update: bool, performance_mode_status: bool, quick_scan_include_exclusions: bool, remove_scanning_thread_pool_cap: bool, disable_cache_maintenance: bool, disable_core_service_ecsintegration: bool, disable_core_service_telemetry: bool, enable_udp_segmentation_offload: bool, enable_udp_receive_offload: bool, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DisableAutoExclusions".to_string(), value: disable_auto_exclusions.into() });
        args.push(MethodParameter { name: "ExclusionPath".to_string(), value: exclusion_path.into() });
        args.push(MethodParameter { name: "ExclusionExtension".to_string(), value: exclusion_extension.into() });
        args.push(MethodParameter { name: "ExclusionProcess".to_string(), value: exclusion_process.into() });
        args.push(MethodParameter { name: "ExclusionIpAddress".to_string(), value: exclusion_ip_address.into() });
        args.push(MethodParameter { name: "QuarantinePurgeItemsAfterDelay".to_string(), value: quarantine_purge_items_after_delay.into() });
        args.push(MethodParameter { name: "RealTimeScanDirection".to_string(), value: real_time_scan_direction.into() });
        args.push(MethodParameter { name: "RemediationScheduleDay".to_string(), value: remediation_schedule_day.into() });
        args.push(MethodParameter { name: "RemediationScheduleTime".to_string(), value: remediation_schedule_time.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionConfiguredState".to_string(), value: remote_encryption_protection_configured_state.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionMaxBlockTime".to_string(), value: remote_encryption_protection_max_block_time.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionAggressiveness".to_string(), value: remote_encryption_protection_aggressiveness.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionExclusions".to_string(), value: remote_encryption_protection_exclusions.into() });
        args.push(MethodParameter { name: "BruteForceProtectionConfiguredState".to_string(), value: brute_force_protection_configured_state.into() });
        args.push(MethodParameter { name: "BruteForceProtectionMaxBlockTime".to_string(), value: brute_force_protection_max_block_time.into() });
        args.push(MethodParameter { name: "BruteForceProtectionAggressiveness".to_string(), value: brute_force_protection_aggressiveness.into() });
        args.push(MethodParameter { name: "BruteForceProtectionExclusions".to_string(), value: brute_force_protection_exclusions.into() });
        args.push(MethodParameter { name: "BruteForceProtectionLocalNetworkBlocking".to_string(), value: brute_force_protection_local_network_blocking.into() });
        args.push(MethodParameter { name: "BruteForceProtectionSkipLearningPeriod".to_string(), value: brute_force_protection_skip_learning_period.into() });
        args.push(MethodParameter { name: "ReportingAdditionalActionTimeOut".to_string(), value: reporting_additional_action_time_out.into() });
        args.push(MethodParameter { name: "ReportingCriticalFailureTimeOut".to_string(), value: reporting_critical_failure_time_out.into() });
        args.push(MethodParameter { name: "ReportingNonCriticalTimeOut".to_string(), value: reporting_non_critical_time_out.into() });
        args.push(MethodParameter { name: "ServiceHealthReportInterval".to_string(), value: service_health_report_interval.into() });
        args.push(MethodParameter { name: "ReportDynamicSignatureDroppedEvent".to_string(), value: report_dynamic_signature_dropped_event.into() });
        args.push(MethodParameter { name: "ScanAvgCPULoadFactor".to_string(), value: scan_avg_cpuload_factor.into() });
        args.push(MethodParameter { name: "CheckForSignaturesBeforeRunningScan".to_string(), value: check_for_signatures_before_running_scan.into() });
        args.push(MethodParameter { name: "ScanPurgeItemsAfterDelay".to_string(), value: scan_purge_items_after_delay.into() });
        args.push(MethodParameter { name: "ScanOnlyIfIdleEnabled".to_string(), value: scan_only_if_idle_enabled.into() });
        args.push(MethodParameter { name: "ScanParameters".to_string(), value: scan_parameters.into() });
        args.push(MethodParameter { name: "ScanScheduleDay".to_string(), value: scan_schedule_day.into() });
        args.push(MethodParameter { name: "ScanScheduleQuickScanTime".to_string(), value: scan_schedule_quick_scan_time.into() });
        args.push(MethodParameter { name: "ScanScheduleTime".to_string(), value: scan_schedule_time.into() });
        args.push(MethodParameter { name: "ThrottleForScheduledScanOnly".to_string(), value: throttle_for_scheduled_scan_only.into() });
        args.push(MethodParameter { name: "SignatureFirstAuGracePeriod".to_string(), value: signature_first_au_grace_period.into() });
        args.push(MethodParameter { name: "SignatureAuGracePeriod".to_string(), value: signature_au_grace_period.into() });
        args.push(MethodParameter { name: "SignatureDefinitionUpdateFileSharesSources".to_string(), value: signature_definition_update_file_shares_sources.into() });
        args.push(MethodParameter { name: "SignatureDisableUpdateOnStartupWithoutEngine".to_string(), value: signature_disable_update_on_startup_without_engine.into() });
        args.push(MethodParameter { name: "SignatureFallbackOrder".to_string(), value: signature_fallback_order.into() });
        args.push(MethodParameter { name: "SignatureScheduleDay".to_string(), value: signature_schedule_day.into() });
        args.push(MethodParameter { name: "SignatureScheduleTime".to_string(), value: signature_schedule_time.into() });
        args.push(MethodParameter { name: "SignatureUpdateCatchupInterval".to_string(), value: signature_update_catchup_interval.into() });
        args.push(MethodParameter { name: "SignatureBlobFileSharesSources".to_string(), value: signature_blob_file_shares_sources.into() });
        args.push(MethodParameter { name: "SignatureUpdateInterval".to_string(), value: signature_update_interval.into() });
        args.push(MethodParameter { name: "SignatureBlobUpdateInterval".to_string(), value: signature_blob_update_interval.into() });
        args.push(MethodParameter { name: "MAPSReporting".to_string(), value: mapsreporting.into() });
        args.push(MethodParameter { name: "SubmitSamplesConsent".to_string(), value: submit_samples_consent.into() });
        args.push(MethodParameter { name: "DisablePrivacyMode".to_string(), value: disable_privacy_mode.into() });
        args.push(MethodParameter { name: "RandomizeScheduleTaskTimes".to_string(), value: randomize_schedule_task_times.into() });
        args.push(MethodParameter { name: "SchedulerRandomizationTime".to_string(), value: scheduler_randomization_time.into() });
        args.push(MethodParameter { name: "DisableBehaviorMonitoring".to_string(), value: disable_behavior_monitoring.into() });
        args.push(MethodParameter { name: "DisableIntrusionPreventionSystem".to_string(), value: disable_intrusion_prevention_system.into() });
        args.push(MethodParameter { name: "DisableIOAVProtection".to_string(), value: disable_ioavprotection.into() });
        args.push(MethodParameter { name: "DisableRealtimeMonitoring".to_string(), value: disable_realtime_monitoring.into() });
        args.push(MethodParameter { name: "DisableScriptScanning".to_string(), value: disable_script_scanning.into() });
        args.push(MethodParameter { name: "DisableArchiveScanning".to_string(), value: disable_archive_scanning.into() });
        args.push(MethodParameter { name: "DisableCatchupFullScan".to_string(), value: disable_catchup_full_scan.into() });
        args.push(MethodParameter { name: "DisableCatchupQuickScan".to_string(), value: disable_catchup_quick_scan.into() });
        args.push(MethodParameter { name: "DisableEmailScanning".to_string(), value: disable_email_scanning.into() });
        args.push(MethodParameter { name: "DisableRemovableDriveScanning".to_string(), value: disable_removable_drive_scanning.into() });
        args.push(MethodParameter { name: "DisableRestorePoint".to_string(), value: disable_restore_point.into() });
        args.push(MethodParameter { name: "DisableScanningMappedNetworkDrivesForFullScan".to_string(), value: disable_scanning_mapped_network_drives_for_full_scan.into() });
        args.push(MethodParameter { name: "DisableScanningNetworkFiles".to_string(), value: disable_scanning_network_files.into() });
        args.push(MethodParameter { name: "ApplyDisableNetworkScanningToIOAV".to_string(), value: apply_disable_network_scanning_to_ioav.into() });
        args.push(MethodParameter { name: "UILockdown".to_string(), value: uilockdown.into() });
        args.push(MethodParameter { name: "ThreatIDDefaultAction_Ids".to_string(), value: threat_iddefault_action__ids.into() });
        args.push(MethodParameter { name: "ThreatIDDefaultAction_Actions".to_string(), value: threat_iddefault_action__actions.into() });
        args.push(MethodParameter { name: "UnknownThreatDefaultAction".to_string(), value: unknown_threat_default_action.into() });
        args.push(MethodParameter { name: "LowThreatDefaultAction".to_string(), value: low_threat_default_action.into() });
        args.push(MethodParameter { name: "ModerateThreatDefaultAction".to_string(), value: moderate_threat_default_action.into() });
        args.push(MethodParameter { name: "HighThreatDefaultAction".to_string(), value: high_threat_default_action.into() });
        args.push(MethodParameter { name: "SevereThreatDefaultAction".to_string(), value: severe_threat_default_action.into() });
        args.push(MethodParameter { name: "PUAProtection".to_string(), value: puaprotection.into() });
        args.push(MethodParameter { name: "DisableBlockAtFirstSeen".to_string(), value: disable_block_at_first_seen.into() });
        args.push(MethodParameter { name: "CloudBlockLevel".to_string(), value: cloud_block_level.into() });
        args.push(MethodParameter { name: "CloudExtendedTimeout".to_string(), value: cloud_extended_timeout.into() });
        args.push(MethodParameter { name: "EnableNetworkProtection".to_string(), value: enable_network_protection.into() });
        args.push(MethodParameter { name: "EnableControlledFolderAccess".to_string(), value: enable_controlled_folder_access.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionOnlyExclusions".to_string(), value: attack_surface_reduction_only_exclusions.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_Ids".to_string(), value: attack_surface_reduction_rules__ids.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_Actions".to_string(), value: attack_surface_reduction_rules__actions.into() });
        args.push(MethodParameter { name: "ControlledFolderAccessAllowedApplications".to_string(), value: controlled_folder_access_allowed_applications.into() });
        args.push(MethodParameter { name: "ControlledFolderAccessProtectedFolders".to_string(), value: controlled_folder_access_protected_folders.into() });
        args.push(MethodParameter { name: "SharedSignaturesPath".to_string(), value: shared_signatures_path.into() });
        args.push(MethodParameter { name: "SharedSignaturesPathUpdateAtScheduledTimeOnly".to_string(), value: shared_signatures_path_update_at_scheduled_time_only.into() });
        args.push(MethodParameter { name: "EnableLowCpuPriority".to_string(), value: enable_low_cpu_priority.into() });
        args.push(MethodParameter { name: "EnableFileHashComputation".to_string(), value: enable_file_hash_computation.into() });
        args.push(MethodParameter { name: "MeteredConnectionUpdates".to_string(), value: metered_connection_updates.into() });
        args.push(MethodParameter { name: "AllowNetworkProtectionOnWinServer".to_string(), value: allow_network_protection_on_win_server.into() });
        args.push(MethodParameter { name: "DisableDatagramProcessing".to_string(), value: disable_datagram_processing.into() });
        args.push(MethodParameter { name: "EnableConvertWarnToBlock".to_string(), value: enable_convert_warn_to_block.into() });
        args.push(MethodParameter { name: "DisableCpuThrottleOnIdleScans".to_string(), value: disable_cpu_throttle_on_idle_scans.into() });
        args.push(MethodParameter { name: "EnableFullScanOnBatteryPower".to_string(), value: enable_full_scan_on_battery_power.into() });
        args.push(MethodParameter { name: "ProxyPacUrl".to_string(), value: proxy_pac_url.into() });
        args.push(MethodParameter { name: "ProxyServer".to_string(), value: proxy_server.into() });
        args.push(MethodParameter { name: "ProxyBypass".to_string(), value: proxy_bypass.into() });
        args.push(MethodParameter { name: "ForceUseProxyOnly".to_string(), value: force_use_proxy_only.into() });
        args.push(MethodParameter { name: "DisableTlsParsing".to_string(), value: disable_tls_parsing.into() });
        args.push(MethodParameter { name: "DisableHttpParsing".to_string(), value: disable_http_parsing.into() });
        args.push(MethodParameter { name: "DisableDnsParsing".to_string(), value: disable_dns_parsing.into() });
        args.push(MethodParameter { name: "DisableDnsOverTcpParsing".to_string(), value: disable_dns_over_tcp_parsing.into() });
        args.push(MethodParameter { name: "DisableSshParsing".to_string(), value: disable_ssh_parsing.into() });
        args.push(MethodParameter { name: "PlatformUpdatesChannel".to_string(), value: platform_updates_channel.into() });
        args.push(MethodParameter { name: "EngineUpdatesChannel".to_string(), value: engine_updates_channel.into() });
        args.push(MethodParameter { name: "DefinitionUpdatesChannel".to_string(), value: definition_updates_channel.into() });
        args.push(MethodParameter { name: "DisableGradualRelease".to_string(), value: disable_gradual_release.into() });
        args.push(MethodParameter { name: "AllowNetworkProtectionDownLevel".to_string(), value: allow_network_protection_down_level.into() });
        args.push(MethodParameter { name: "AllowDatagramProcessingOnWinServer".to_string(), value: allow_datagram_processing_on_win_server.into() });
        args.push(MethodParameter { name: "EnableDnsSinkhole".to_string(), value: enable_dns_sinkhole.into() });
        args.push(MethodParameter { name: "DisableInboundConnectionFiltering".to_string(), value: disable_inbound_connection_filtering.into() });
        args.push(MethodParameter { name: "DisableRdpParsing".to_string(), value: disable_rdp_parsing.into() });
        args.push(MethodParameter { name: "DisableNetworkProtectionPerfTelemetry".to_string(), value: disable_network_protection_perf_telemetry.into() });
        args.push(MethodParameter { name: "TrustLabelProtectionStatus".to_string(), value: trust_label_protection_status.into() });
        args.push(MethodParameter { name: "DisableFtpParsing".to_string(), value: disable_ftp_parsing.into() });
        args.push(MethodParameter { name: "AllowSwitchToAsyncInspection".to_string(), value: allow_switch_to_async_inspection.into() });
        args.push(MethodParameter { name: "ScanScheduleOffset".to_string(), value: scan_schedule_offset.into() });
        args.push(MethodParameter { name: "DisableTDTFeature".to_string(), value: disable_tdtfeature.into() });
        args.push(MethodParameter { name: "DisableSmtpParsing".to_string(), value: disable_smtp_parsing.into() });
        args.push(MethodParameter { name: "DisableQuicParsing".to_string(), value: disable_quic_parsing.into() });
        args.push(MethodParameter { name: "NetworkProtectionReputationMode".to_string(), value: network_protection_reputation_mode.into() });
        args.push(MethodParameter { name: "IntelTDTEnabled".to_string(), value: intel_tdtenabled.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_RuleSpecificExclusions_Id".to_string(), value: attack_surface_reduction_rules__rule_specific_exclusions__id.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_RuleSpecificExclusions".to_string(), value: attack_surface_reduction_rules__rule_specific_exclusions.into() });
        args.push(MethodParameter { name: "OobeEnableRtpAndSigUpdate".to_string(), value: oobe_enable_rtp_and_sig_update.into() });
        args.push(MethodParameter { name: "PerformanceModeStatus".to_string(), value: performance_mode_status.into() });
        args.push(MethodParameter { name: "QuickScanIncludeExclusions".to_string(), value: quick_scan_include_exclusions.into() });
        args.push(MethodParameter { name: "RemoveScanningThreadPoolCap".to_string(), value: remove_scanning_thread_pool_cap.into() });
        args.push(MethodParameter { name: "DisableCacheMaintenance".to_string(), value: disable_cache_maintenance.into() });
        args.push(MethodParameter { name: "DisableCoreServiceECSIntegration".to_string(), value: disable_core_service_ecsintegration.into() });
        args.push(MethodParameter { name: "DisableCoreServiceTelemetry".to_string(), value: disable_core_service_telemetry.into() });
        args.push(MethodParameter { name: "EnableUdpSegmentationOffload".to_string(), value: enable_udp_segmentation_offload.into() });
        args.push(MethodParameter { name: "EnableUdpReceiveOffload".to_string(), value: enable_udp_receive_offload.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Remove", &args)

    }


/// 

    /// * `attack_surface_reduction_only_exclusions` -  (String[])
    /// * `attack_surface_reduction_rules__actions` -  (u8[])
    /// * `attack_surface_reduction_rules__ids` -  (String[])
    /// * `attack_surface_reduction_rules__rule_specific_exclusions` -  (String[])
    /// * `attack_surface_reduction_rules__rule_specific_exclusions__id` -  (String[])
    /// * `brute_force_protection_exclusions` -  (String[])
    /// * `controlled_folder_access_allowed_applications` -  (String[])
    /// * `controlled_folder_access_protected_folders` -  (String[])
    /// * `exclusion_extension` -  (String[])
    /// * `exclusion_ip_address` -  (String[])
    /// * `exclusion_path` -  (String[])
    /// * `exclusion_process` -  (String[])
    /// * `force` -  (bool)
    /// * `remote_encryption_protection_exclusions` -  (String[])
    /// * `shared_signatures_path` -  (String)
    /// * `threat_iddefault_action__actions` -  (u8[])
    /// * `threat_iddefault_action__ids` -  (i64[])

    /// * `return_value` -  (u32)
    pub fn add(&self, exclusion_path: &Vec<String>, exclusion_extension: &Vec<String>, exclusion_process: &Vec<String>, exclusion_ip_address: &Vec<String>, threat_iddefault_action__ids: &Vec<i64>, threat_iddefault_action__actions: &Vec<u8>, attack_surface_reduction_only_exclusions: &Vec<String>, attack_surface_reduction_rules__ids: &Vec<String>, attack_surface_reduction_rules__actions: &Vec<u8>, controlled_folder_access_allowed_applications: &Vec<String>, controlled_folder_access_protected_folders: &Vec<String>, shared_signatures_path: &String, attack_surface_reduction_rules__rule_specific_exclusions__id: &Vec<String>, attack_surface_reduction_rules__rule_specific_exclusions: &Vec<String>, remote_encryption_protection_exclusions: &Vec<String>, brute_force_protection_exclusions: &Vec<String>, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ExclusionPath".to_string(), value: exclusion_path.into() });
        args.push(MethodParameter { name: "ExclusionExtension".to_string(), value: exclusion_extension.into() });
        args.push(MethodParameter { name: "ExclusionProcess".to_string(), value: exclusion_process.into() });
        args.push(MethodParameter { name: "ExclusionIpAddress".to_string(), value: exclusion_ip_address.into() });
        args.push(MethodParameter { name: "ThreatIDDefaultAction_Ids".to_string(), value: threat_iddefault_action__ids.into() });
        args.push(MethodParameter { name: "ThreatIDDefaultAction_Actions".to_string(), value: threat_iddefault_action__actions.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionOnlyExclusions".to_string(), value: attack_surface_reduction_only_exclusions.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_Ids".to_string(), value: attack_surface_reduction_rules__ids.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_Actions".to_string(), value: attack_surface_reduction_rules__actions.into() });
        args.push(MethodParameter { name: "ControlledFolderAccessAllowedApplications".to_string(), value: controlled_folder_access_allowed_applications.into() });
        args.push(MethodParameter { name: "ControlledFolderAccessProtectedFolders".to_string(), value: controlled_folder_access_protected_folders.into() });
        args.push(MethodParameter { name: "SharedSignaturesPath".to_string(), value: shared_signatures_path.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_RuleSpecificExclusions_Id".to_string(), value: attack_surface_reduction_rules__rule_specific_exclusions__id.into() });
        args.push(MethodParameter { name: "AttackSurfaceReductionRules_RuleSpecificExclusions".to_string(), value: attack_surface_reduction_rules__rule_specific_exclusions.into() });
        args.push(MethodParameter { name: "RemoteEncryptionProtectionExclusions".to_string(), value: remote_encryption_protection_exclusions.into() });
        args.push(MethodParameter { name: "BruteForceProtectionExclusions".to_string(), value: brute_force_protection_exclusions.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Add", &args)

    }

}

