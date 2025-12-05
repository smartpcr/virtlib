// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.ProtectionManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MpComputerStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MpComputerStatus {
    #[serde(flatten)]
    pub base: BaseStatus,

/// 
    #[serde(rename = "AMEngineVersion")]
    pub amengine_version: Option<String>,

/// 
    #[serde(rename = "AMProductVersion")]
    pub amproduct_version: Option<String>,

/// 
    #[serde(rename = "AMRunningMode")]
    pub amrunning_mode: Option<String>,

/// 
    #[serde(rename = "AMServiceEnabled")]
    pub amservice_enabled: Option<bool>,

/// 
    #[serde(rename = "AMServiceVersion")]
    pub amservice_version: Option<String>,

/// 
    #[serde(rename = "AntispywareEnabled")]
    pub antispyware_enabled: Option<bool>,

/// 
    #[serde(rename = "AntispywareSignatureAge")]
    pub antispyware_signature_age: Option<u32>,

/// 
    #[serde(rename = "AntispywareSignatureLastUpdated")]
    pub antispyware_signature_last_updated: Option<String>,

/// 
    #[serde(rename = "AntispywareSignatureVersion")]
    pub antispyware_signature_version: Option<String>,

/// 
    #[serde(rename = "AntivirusEnabled")]
    pub antivirus_enabled: Option<bool>,

/// 
    #[serde(rename = "AntivirusSignatureAge")]
    pub antivirus_signature_age: Option<u32>,

/// 
    #[serde(rename = "AntivirusSignatureLastUpdated")]
    pub antivirus_signature_last_updated: Option<String>,

/// 
    #[serde(rename = "AntivirusSignatureVersion")]
    pub antivirus_signature_version: Option<String>,

/// 
    #[serde(rename = "BehaviorMonitorEnabled")]
    pub behavior_monitor_enabled: Option<bool>,

/// 
    #[serde(rename = "ComputerID")]
    pub computer_id: Option<String>,

/// 
    #[serde(rename = "ComputerState")]
    pub computer_state: Option<u32>,

/// 
    #[serde(rename = "DefenderSignaturesOutOfDate")]
    pub defender_signatures_out_of_date: Option<bool>,

/// 
    #[serde(rename = "DeviceControlDefaultEnforcement")]
    pub device_control_default_enforcement: Option<String>,

/// 
    #[serde(rename = "DeviceControlPoliciesLastUpdated")]
    pub device_control_policies_last_updated: Option<String>,

/// 
    #[serde(rename = "DeviceControlState")]
    pub device_control_state: Option<String>,

/// 
    #[serde(rename = "FullScanAge")]
    pub full_scan_age: Option<u32>,

/// 
    #[serde(rename = "FullScanEndTime")]
    pub full_scan_end_time: Option<String>,

/// 
    #[serde(rename = "FullScanOverdue")]
    pub full_scan_overdue: Option<bool>,

/// 
    #[serde(rename = "FullScanRequired")]
    pub full_scan_required: Option<bool>,

/// 
    #[serde(rename = "FullScanSignatureVersion")]
    pub full_scan_signature_version: Option<String>,

/// 
    #[serde(rename = "FullScanStartTime")]
    pub full_scan_start_time: Option<String>,

/// 
    #[serde(rename = "InitializationProgress")]
    pub initialization_progress: Option<String>,

/// 
    #[serde(rename = "IoavProtectionEnabled")]
    pub ioav_protection_enabled: Option<bool>,

/// 
    #[serde(rename = "IsTamperProtected")]
    pub is_tamper_protected: Option<bool>,

/// 
    #[serde(rename = "IsVirtualMachine")]
    pub is_virtual_machine: Option<bool>,

/// 
    #[serde(rename = "LastFullScanSource")]
    pub last_full_scan_source: Option<u8>,

/// 
    #[serde(rename = "LastQuickScanSource")]
    pub last_quick_scan_source: Option<u8>,

/// 
    #[serde(rename = "NISEnabled")]
    pub nisenabled: Option<bool>,

/// 
    #[serde(rename = "NISEngineVersion")]
    pub nisengine_version: Option<String>,

/// 
    #[serde(rename = "NISSignatureAge")]
    pub nissignature_age: Option<u32>,

/// 
    #[serde(rename = "NISSignatureLastUpdated")]
    pub nissignature_last_updated: Option<String>,

/// 
    #[serde(rename = "NISSignatureVersion")]
    pub nissignature_version: Option<String>,

/// 
    #[serde(rename = "OnAccessProtectionEnabled")]
    pub on_access_protection_enabled: Option<bool>,

/// 
    #[serde(rename = "ProductStatus")]
    pub product_status: Option<u32>,

/// 
    #[serde(rename = "QuickScanAge")]
    pub quick_scan_age: Option<u32>,

/// 
    #[serde(rename = "QuickScanEndTime")]
    pub quick_scan_end_time: Option<String>,

/// 
    #[serde(rename = "QuickScanOverdue")]
    pub quick_scan_overdue: Option<bool>,

/// 
    #[serde(rename = "QuickScanSignatureVersion")]
    pub quick_scan_signature_version: Option<String>,

/// 
    #[serde(rename = "QuickScanStartTime")]
    pub quick_scan_start_time: Option<String>,

/// 
    #[serde(rename = "RealTimeProtectionEnabled")]
    pub real_time_protection_enabled: Option<bool>,

/// 
    #[serde(rename = "RealTimeScanDirection")]
    pub real_time_scan_direction: Option<u8>,

/// 
    #[serde(rename = "RebootRequired")]
    pub reboot_required: Option<bool>,

/// 
    #[serde(rename = "SmartAppControlExpiration")]
    pub smart_app_control_expiration: Option<String>,

/// 
    #[serde(rename = "SmartAppControlState")]
    pub smart_app_control_state: Option<String>,

/// 
    #[serde(rename = "TamperProtectionSource")]
    pub tamper_protection_source: Option<String>,

/// 
    #[serde(rename = "TDTCapable")]
    pub tdtcapable: Option<String>,

/// 
    #[serde(rename = "TDTMode")]
    pub tdtmode: Option<String>,

/// 
    #[serde(rename = "TDTSiloType")]
    pub tdtsilo_type: Option<String>,

/// 
    #[serde(rename = "TDTStatus")]
    pub tdtstatus: Option<String>,

/// 
    #[serde(rename = "TDTTelemetry")]
    pub tdttelemetry: Option<String>,

/// 
    #[serde(rename = "TroubleShootingDailyMaxQuota")]
    pub trouble_shooting_daily_max_quota: Option<String>,

/// 
    #[serde(rename = "TroubleShootingDailyQuotaLeft")]
    pub trouble_shooting_daily_quota_left: Option<String>,

/// 
    #[serde(rename = "TroubleShootingEndTime")]
    pub trouble_shooting_end_time: Option<String>,

/// 
    #[serde(rename = "TroubleShootingExpirationLeft")]
    pub trouble_shooting_expiration_left: Option<String>,

/// 
    #[serde(rename = "TroubleShootingMode")]
    pub trouble_shooting_mode: Option<String>,

/// 
    #[serde(rename = "TroubleShootingModeSource")]
    pub trouble_shooting_mode_source: Option<String>,

/// 
    #[serde(rename = "TroubleShootingQuotaResetTime")]
    pub trouble_shooting_quota_reset_time: Option<String>,

/// 
    #[serde(rename = "TroubleShootingStartTime")]
    pub trouble_shooting_start_time: Option<String>,
}

impl MSFT_MpComputerStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BaseStatus::new(),
            amengine_version: None,
            amproduct_version: None,
            amrunning_mode: None,
            amservice_enabled: None,
            amservice_version: None,
            antispyware_enabled: None,
            antispyware_signature_age: None,
            antispyware_signature_last_updated: None,
            antispyware_signature_version: None,
            antivirus_enabled: None,
            antivirus_signature_age: None,
            antivirus_signature_last_updated: None,
            antivirus_signature_version: None,
            behavior_monitor_enabled: None,
            computer_id: None,
            computer_state: None,
            defender_signatures_out_of_date: None,
            device_control_default_enforcement: None,
            device_control_policies_last_updated: None,
            device_control_state: None,
            full_scan_age: None,
            full_scan_end_time: None,
            full_scan_overdue: None,
            full_scan_required: None,
            full_scan_signature_version: None,
            full_scan_start_time: None,
            initialization_progress: None,
            ioav_protection_enabled: None,
            is_tamper_protected: None,
            is_virtual_machine: None,
            last_full_scan_source: None,
            last_quick_scan_source: None,
            nisenabled: None,
            nisengine_version: None,
            nissignature_age: None,
            nissignature_last_updated: None,
            nissignature_version: None,
            on_access_protection_enabled: None,
            product_status: None,
            quick_scan_age: None,
            quick_scan_end_time: None,
            quick_scan_overdue: None,
            quick_scan_signature_version: None,
            quick_scan_start_time: None,
            real_time_protection_enabled: None,
            real_time_scan_direction: None,
            reboot_required: None,
            smart_app_control_expiration: None,
            smart_app_control_state: None,
            tamper_protection_source: None,
            tdtcapable: None,
            tdtmode: None,
            tdtsilo_type: None,
            tdtstatus: None,
            tdttelemetry: None,
            trouble_shooting_daily_max_quota: None,
            trouble_shooting_daily_quota_left: None,
            trouble_shooting_end_time: None,
            trouble_shooting_expiration_left: None,
            trouble_shooting_mode: None,
            trouble_shooting_mode_source: None,
            trouble_shooting_quota_reset_time: None,
            trouble_shooting_start_time: None,
        }
    }


    /// Sets the value of AMEngineVersion
    pub fn set_amengine_version(&mut self, value: String) {
        self.amengine_version = Some(value);
    }

    /// Gets the value of AMEngineVersion
    pub fn get_amengine_version(&self) -> Option<&String> {
        self.amengine_version.as_ref()
    }

    /// Sets the value of AMProductVersion
    pub fn set_amproduct_version(&mut self, value: String) {
        self.amproduct_version = Some(value);
    }

    /// Gets the value of AMProductVersion
    pub fn get_amproduct_version(&self) -> Option<&String> {
        self.amproduct_version.as_ref()
    }

    /// Sets the value of AMRunningMode
    pub fn set_amrunning_mode(&mut self, value: String) {
        self.amrunning_mode = Some(value);
    }

    /// Gets the value of AMRunningMode
    pub fn get_amrunning_mode(&self) -> Option<&String> {
        self.amrunning_mode.as_ref()
    }

    /// Sets the value of AMServiceEnabled
    pub fn set_amservice_enabled(&mut self, value: bool) {
        self.amservice_enabled = Some(value);
    }

    /// Gets the value of AMServiceEnabled
    pub fn get_amservice_enabled(&self) -> Option<&bool> {
        self.amservice_enabled.as_ref()
    }

    /// Sets the value of AMServiceVersion
    pub fn set_amservice_version(&mut self, value: String) {
        self.amservice_version = Some(value);
    }

    /// Gets the value of AMServiceVersion
    pub fn get_amservice_version(&self) -> Option<&String> {
        self.amservice_version.as_ref()
    }

    /// Sets the value of AntispywareEnabled
    pub fn set_antispyware_enabled(&mut self, value: bool) {
        self.antispyware_enabled = Some(value);
    }

    /// Gets the value of AntispywareEnabled
    pub fn get_antispyware_enabled(&self) -> Option<&bool> {
        self.antispyware_enabled.as_ref()
    }

    /// Sets the value of AntispywareSignatureAge
    pub fn set_antispyware_signature_age(&mut self, value: u32) {
        self.antispyware_signature_age = Some(value);
    }

    /// Gets the value of AntispywareSignatureAge
    pub fn get_antispyware_signature_age(&self) -> Option<&u32> {
        self.antispyware_signature_age.as_ref()
    }

    /// Sets the value of AntispywareSignatureLastUpdated
    pub fn set_antispyware_signature_last_updated(&mut self, value: String) {
        self.antispyware_signature_last_updated = Some(value);
    }

    /// Gets the value of AntispywareSignatureLastUpdated
    pub fn get_antispyware_signature_last_updated(&self) -> Option<&String> {
        self.antispyware_signature_last_updated.as_ref()
    }

    /// Sets the value of AntispywareSignatureVersion
    pub fn set_antispyware_signature_version(&mut self, value: String) {
        self.antispyware_signature_version = Some(value);
    }

    /// Gets the value of AntispywareSignatureVersion
    pub fn get_antispyware_signature_version(&self) -> Option<&String> {
        self.antispyware_signature_version.as_ref()
    }

    /// Sets the value of AntivirusEnabled
    pub fn set_antivirus_enabled(&mut self, value: bool) {
        self.antivirus_enabled = Some(value);
    }

    /// Gets the value of AntivirusEnabled
    pub fn get_antivirus_enabled(&self) -> Option<&bool> {
        self.antivirus_enabled.as_ref()
    }

    /// Sets the value of AntivirusSignatureAge
    pub fn set_antivirus_signature_age(&mut self, value: u32) {
        self.antivirus_signature_age = Some(value);
    }

    /// Gets the value of AntivirusSignatureAge
    pub fn get_antivirus_signature_age(&self) -> Option<&u32> {
        self.antivirus_signature_age.as_ref()
    }

    /// Sets the value of AntivirusSignatureLastUpdated
    pub fn set_antivirus_signature_last_updated(&mut self, value: String) {
        self.antivirus_signature_last_updated = Some(value);
    }

    /// Gets the value of AntivirusSignatureLastUpdated
    pub fn get_antivirus_signature_last_updated(&self) -> Option<&String> {
        self.antivirus_signature_last_updated.as_ref()
    }

    /// Sets the value of AntivirusSignatureVersion
    pub fn set_antivirus_signature_version(&mut self, value: String) {
        self.antivirus_signature_version = Some(value);
    }

    /// Gets the value of AntivirusSignatureVersion
    pub fn get_antivirus_signature_version(&self) -> Option<&String> {
        self.antivirus_signature_version.as_ref()
    }

    /// Sets the value of BehaviorMonitorEnabled
    pub fn set_behavior_monitor_enabled(&mut self, value: bool) {
        self.behavior_monitor_enabled = Some(value);
    }

    /// Gets the value of BehaviorMonitorEnabled
    pub fn get_behavior_monitor_enabled(&self) -> Option<&bool> {
        self.behavior_monitor_enabled.as_ref()
    }

    /// Sets the value of ComputerID
    pub fn set_computer_id(&mut self, value: String) {
        self.computer_id = Some(value);
    }

    /// Gets the value of ComputerID
    pub fn get_computer_id(&self) -> Option<&String> {
        self.computer_id.as_ref()
    }

    /// Sets the value of ComputerState
    pub fn set_computer_state(&mut self, value: u32) {
        self.computer_state = Some(value);
    }

    /// Gets the value of ComputerState
    pub fn get_computer_state(&self) -> Option<&u32> {
        self.computer_state.as_ref()
    }

    /// Sets the value of DefenderSignaturesOutOfDate
    pub fn set_defender_signatures_out_of_date(&mut self, value: bool) {
        self.defender_signatures_out_of_date = Some(value);
    }

    /// Gets the value of DefenderSignaturesOutOfDate
    pub fn get_defender_signatures_out_of_date(&self) -> Option<&bool> {
        self.defender_signatures_out_of_date.as_ref()
    }

    /// Sets the value of DeviceControlDefaultEnforcement
    pub fn set_device_control_default_enforcement(&mut self, value: String) {
        self.device_control_default_enforcement = Some(value);
    }

    /// Gets the value of DeviceControlDefaultEnforcement
    pub fn get_device_control_default_enforcement(&self) -> Option<&String> {
        self.device_control_default_enforcement.as_ref()
    }

    /// Sets the value of DeviceControlPoliciesLastUpdated
    pub fn set_device_control_policies_last_updated(&mut self, value: String) {
        self.device_control_policies_last_updated = Some(value);
    }

    /// Gets the value of DeviceControlPoliciesLastUpdated
    pub fn get_device_control_policies_last_updated(&self) -> Option<&String> {
        self.device_control_policies_last_updated.as_ref()
    }

    /// Sets the value of DeviceControlState
    pub fn set_device_control_state(&mut self, value: String) {
        self.device_control_state = Some(value);
    }

    /// Gets the value of DeviceControlState
    pub fn get_device_control_state(&self) -> Option<&String> {
        self.device_control_state.as_ref()
    }

    /// Sets the value of FullScanAge
    pub fn set_full_scan_age(&mut self, value: u32) {
        self.full_scan_age = Some(value);
    }

    /// Gets the value of FullScanAge
    pub fn get_full_scan_age(&self) -> Option<&u32> {
        self.full_scan_age.as_ref()
    }

    /// Sets the value of FullScanEndTime
    pub fn set_full_scan_end_time(&mut self, value: String) {
        self.full_scan_end_time = Some(value);
    }

    /// Gets the value of FullScanEndTime
    pub fn get_full_scan_end_time(&self) -> Option<&String> {
        self.full_scan_end_time.as_ref()
    }

    /// Sets the value of FullScanOverdue
    pub fn set_full_scan_overdue(&mut self, value: bool) {
        self.full_scan_overdue = Some(value);
    }

    /// Gets the value of FullScanOverdue
    pub fn get_full_scan_overdue(&self) -> Option<&bool> {
        self.full_scan_overdue.as_ref()
    }

    /// Sets the value of FullScanRequired
    pub fn set_full_scan_required(&mut self, value: bool) {
        self.full_scan_required = Some(value);
    }

    /// Gets the value of FullScanRequired
    pub fn get_full_scan_required(&self) -> Option<&bool> {
        self.full_scan_required.as_ref()
    }

    /// Sets the value of FullScanSignatureVersion
    pub fn set_full_scan_signature_version(&mut self, value: String) {
        self.full_scan_signature_version = Some(value);
    }

    /// Gets the value of FullScanSignatureVersion
    pub fn get_full_scan_signature_version(&self) -> Option<&String> {
        self.full_scan_signature_version.as_ref()
    }

    /// Sets the value of FullScanStartTime
    pub fn set_full_scan_start_time(&mut self, value: String) {
        self.full_scan_start_time = Some(value);
    }

    /// Gets the value of FullScanStartTime
    pub fn get_full_scan_start_time(&self) -> Option<&String> {
        self.full_scan_start_time.as_ref()
    }

    /// Sets the value of InitializationProgress
    pub fn set_initialization_progress(&mut self, value: String) {
        self.initialization_progress = Some(value);
    }

    /// Gets the value of InitializationProgress
    pub fn get_initialization_progress(&self) -> Option<&String> {
        self.initialization_progress.as_ref()
    }

    /// Sets the value of IoavProtectionEnabled
    pub fn set_ioav_protection_enabled(&mut self, value: bool) {
        self.ioav_protection_enabled = Some(value);
    }

    /// Gets the value of IoavProtectionEnabled
    pub fn get_ioav_protection_enabled(&self) -> Option<&bool> {
        self.ioav_protection_enabled.as_ref()
    }

    /// Sets the value of IsTamperProtected
    pub fn set_is_tamper_protected(&mut self, value: bool) {
        self.is_tamper_protected = Some(value);
    }

    /// Gets the value of IsTamperProtected
    pub fn get_is_tamper_protected(&self) -> Option<&bool> {
        self.is_tamper_protected.as_ref()
    }

    /// Sets the value of IsVirtualMachine
    pub fn set_is_virtual_machine(&mut self, value: bool) {
        self.is_virtual_machine = Some(value);
    }

    /// Gets the value of IsVirtualMachine
    pub fn get_is_virtual_machine(&self) -> Option<&bool> {
        self.is_virtual_machine.as_ref()
    }

    /// Sets the value of LastFullScanSource
    pub fn set_last_full_scan_source(&mut self, value: u8) {
        self.last_full_scan_source = Some(value);
    }

    /// Gets the value of LastFullScanSource
    pub fn get_last_full_scan_source(&self) -> Option<&u8> {
        self.last_full_scan_source.as_ref()
    }

    /// Sets the value of LastQuickScanSource
    pub fn set_last_quick_scan_source(&mut self, value: u8) {
        self.last_quick_scan_source = Some(value);
    }

    /// Gets the value of LastQuickScanSource
    pub fn get_last_quick_scan_source(&self) -> Option<&u8> {
        self.last_quick_scan_source.as_ref()
    }

    /// Sets the value of NISEnabled
    pub fn set_nisenabled(&mut self, value: bool) {
        self.nisenabled = Some(value);
    }

    /// Gets the value of NISEnabled
    pub fn get_nisenabled(&self) -> Option<&bool> {
        self.nisenabled.as_ref()
    }

    /// Sets the value of NISEngineVersion
    pub fn set_nisengine_version(&mut self, value: String) {
        self.nisengine_version = Some(value);
    }

    /// Gets the value of NISEngineVersion
    pub fn get_nisengine_version(&self) -> Option<&String> {
        self.nisengine_version.as_ref()
    }

    /// Sets the value of NISSignatureAge
    pub fn set_nissignature_age(&mut self, value: u32) {
        self.nissignature_age = Some(value);
    }

    /// Gets the value of NISSignatureAge
    pub fn get_nissignature_age(&self) -> Option<&u32> {
        self.nissignature_age.as_ref()
    }

    /// Sets the value of NISSignatureLastUpdated
    pub fn set_nissignature_last_updated(&mut self, value: String) {
        self.nissignature_last_updated = Some(value);
    }

    /// Gets the value of NISSignatureLastUpdated
    pub fn get_nissignature_last_updated(&self) -> Option<&String> {
        self.nissignature_last_updated.as_ref()
    }

    /// Sets the value of NISSignatureVersion
    pub fn set_nissignature_version(&mut self, value: String) {
        self.nissignature_version = Some(value);
    }

    /// Gets the value of NISSignatureVersion
    pub fn get_nissignature_version(&self) -> Option<&String> {
        self.nissignature_version.as_ref()
    }

    /// Sets the value of OnAccessProtectionEnabled
    pub fn set_on_access_protection_enabled(&mut self, value: bool) {
        self.on_access_protection_enabled = Some(value);
    }

    /// Gets the value of OnAccessProtectionEnabled
    pub fn get_on_access_protection_enabled(&self) -> Option<&bool> {
        self.on_access_protection_enabled.as_ref()
    }

    /// Sets the value of ProductStatus
    pub fn set_product_status(&mut self, value: u32) {
        self.product_status = Some(value);
    }

    /// Gets the value of ProductStatus
    pub fn get_product_status(&self) -> Option<&u32> {
        self.product_status.as_ref()
    }

    /// Sets the value of QuickScanAge
    pub fn set_quick_scan_age(&mut self, value: u32) {
        self.quick_scan_age = Some(value);
    }

    /// Gets the value of QuickScanAge
    pub fn get_quick_scan_age(&self) -> Option<&u32> {
        self.quick_scan_age.as_ref()
    }

    /// Sets the value of QuickScanEndTime
    pub fn set_quick_scan_end_time(&mut self, value: String) {
        self.quick_scan_end_time = Some(value);
    }

    /// Gets the value of QuickScanEndTime
    pub fn get_quick_scan_end_time(&self) -> Option<&String> {
        self.quick_scan_end_time.as_ref()
    }

    /// Sets the value of QuickScanOverdue
    pub fn set_quick_scan_overdue(&mut self, value: bool) {
        self.quick_scan_overdue = Some(value);
    }

    /// Gets the value of QuickScanOverdue
    pub fn get_quick_scan_overdue(&self) -> Option<&bool> {
        self.quick_scan_overdue.as_ref()
    }

    /// Sets the value of QuickScanSignatureVersion
    pub fn set_quick_scan_signature_version(&mut self, value: String) {
        self.quick_scan_signature_version = Some(value);
    }

    /// Gets the value of QuickScanSignatureVersion
    pub fn get_quick_scan_signature_version(&self) -> Option<&String> {
        self.quick_scan_signature_version.as_ref()
    }

    /// Sets the value of QuickScanStartTime
    pub fn set_quick_scan_start_time(&mut self, value: String) {
        self.quick_scan_start_time = Some(value);
    }

    /// Gets the value of QuickScanStartTime
    pub fn get_quick_scan_start_time(&self) -> Option<&String> {
        self.quick_scan_start_time.as_ref()
    }

    /// Sets the value of RealTimeProtectionEnabled
    pub fn set_real_time_protection_enabled(&mut self, value: bool) {
        self.real_time_protection_enabled = Some(value);
    }

    /// Gets the value of RealTimeProtectionEnabled
    pub fn get_real_time_protection_enabled(&self) -> Option<&bool> {
        self.real_time_protection_enabled.as_ref()
    }

    /// Sets the value of RealTimeScanDirection
    pub fn set_real_time_scan_direction(&mut self, value: u8) {
        self.real_time_scan_direction = Some(value);
    }

    /// Gets the value of RealTimeScanDirection
    pub fn get_real_time_scan_direction(&self) -> Option<&u8> {
        self.real_time_scan_direction.as_ref()
    }

    /// Sets the value of RebootRequired
    pub fn set_reboot_required(&mut self, value: bool) {
        self.reboot_required = Some(value);
    }

    /// Gets the value of RebootRequired
    pub fn get_reboot_required(&self) -> Option<&bool> {
        self.reboot_required.as_ref()
    }

    /// Sets the value of SmartAppControlExpiration
    pub fn set_smart_app_control_expiration(&mut self, value: String) {
        self.smart_app_control_expiration = Some(value);
    }

    /// Gets the value of SmartAppControlExpiration
    pub fn get_smart_app_control_expiration(&self) -> Option<&String> {
        self.smart_app_control_expiration.as_ref()
    }

    /// Sets the value of SmartAppControlState
    pub fn set_smart_app_control_state(&mut self, value: String) {
        self.smart_app_control_state = Some(value);
    }

    /// Gets the value of SmartAppControlState
    pub fn get_smart_app_control_state(&self) -> Option<&String> {
        self.smart_app_control_state.as_ref()
    }

    /// Sets the value of TamperProtectionSource
    pub fn set_tamper_protection_source(&mut self, value: String) {
        self.tamper_protection_source = Some(value);
    }

    /// Gets the value of TamperProtectionSource
    pub fn get_tamper_protection_source(&self) -> Option<&String> {
        self.tamper_protection_source.as_ref()
    }

    /// Sets the value of TDTCapable
    pub fn set_tdtcapable(&mut self, value: String) {
        self.tdtcapable = Some(value);
    }

    /// Gets the value of TDTCapable
    pub fn get_tdtcapable(&self) -> Option<&String> {
        self.tdtcapable.as_ref()
    }

    /// Sets the value of TDTMode
    pub fn set_tdtmode(&mut self, value: String) {
        self.tdtmode = Some(value);
    }

    /// Gets the value of TDTMode
    pub fn get_tdtmode(&self) -> Option<&String> {
        self.tdtmode.as_ref()
    }

    /// Sets the value of TDTSiloType
    pub fn set_tdtsilo_type(&mut self, value: String) {
        self.tdtsilo_type = Some(value);
    }

    /// Gets the value of TDTSiloType
    pub fn get_tdtsilo_type(&self) -> Option<&String> {
        self.tdtsilo_type.as_ref()
    }

    /// Sets the value of TDTStatus
    pub fn set_tdtstatus(&mut self, value: String) {
        self.tdtstatus = Some(value);
    }

    /// Gets the value of TDTStatus
    pub fn get_tdtstatus(&self) -> Option<&String> {
        self.tdtstatus.as_ref()
    }

    /// Sets the value of TDTTelemetry
    pub fn set_tdttelemetry(&mut self, value: String) {
        self.tdttelemetry = Some(value);
    }

    /// Gets the value of TDTTelemetry
    pub fn get_tdttelemetry(&self) -> Option<&String> {
        self.tdttelemetry.as_ref()
    }

    /// Sets the value of TroubleShootingDailyMaxQuota
    pub fn set_trouble_shooting_daily_max_quota(&mut self, value: String) {
        self.trouble_shooting_daily_max_quota = Some(value);
    }

    /// Gets the value of TroubleShootingDailyMaxQuota
    pub fn get_trouble_shooting_daily_max_quota(&self) -> Option<&String> {
        self.trouble_shooting_daily_max_quota.as_ref()
    }

    /// Sets the value of TroubleShootingDailyQuotaLeft
    pub fn set_trouble_shooting_daily_quota_left(&mut self, value: String) {
        self.trouble_shooting_daily_quota_left = Some(value);
    }

    /// Gets the value of TroubleShootingDailyQuotaLeft
    pub fn get_trouble_shooting_daily_quota_left(&self) -> Option<&String> {
        self.trouble_shooting_daily_quota_left.as_ref()
    }

    /// Sets the value of TroubleShootingEndTime
    pub fn set_trouble_shooting_end_time(&mut self, value: String) {
        self.trouble_shooting_end_time = Some(value);
    }

    /// Gets the value of TroubleShootingEndTime
    pub fn get_trouble_shooting_end_time(&self) -> Option<&String> {
        self.trouble_shooting_end_time.as_ref()
    }

    /// Sets the value of TroubleShootingExpirationLeft
    pub fn set_trouble_shooting_expiration_left(&mut self, value: String) {
        self.trouble_shooting_expiration_left = Some(value);
    }

    /// Gets the value of TroubleShootingExpirationLeft
    pub fn get_trouble_shooting_expiration_left(&self) -> Option<&String> {
        self.trouble_shooting_expiration_left.as_ref()
    }

    /// Sets the value of TroubleShootingMode
    pub fn set_trouble_shooting_mode(&mut self, value: String) {
        self.trouble_shooting_mode = Some(value);
    }

    /// Gets the value of TroubleShootingMode
    pub fn get_trouble_shooting_mode(&self) -> Option<&String> {
        self.trouble_shooting_mode.as_ref()
    }

    /// Sets the value of TroubleShootingModeSource
    pub fn set_trouble_shooting_mode_source(&mut self, value: String) {
        self.trouble_shooting_mode_source = Some(value);
    }

    /// Gets the value of TroubleShootingModeSource
    pub fn get_trouble_shooting_mode_source(&self) -> Option<&String> {
        self.trouble_shooting_mode_source.as_ref()
    }

    /// Sets the value of TroubleShootingQuotaResetTime
    pub fn set_trouble_shooting_quota_reset_time(&mut self, value: String) {
        self.trouble_shooting_quota_reset_time = Some(value);
    }

    /// Gets the value of TroubleShootingQuotaResetTime
    pub fn get_trouble_shooting_quota_reset_time(&self) -> Option<&String> {
        self.trouble_shooting_quota_reset_time.as_ref()
    }

    /// Sets the value of TroubleShootingStartTime
    pub fn set_trouble_shooting_start_time(&mut self, value: String) {
        self.trouble_shooting_start_time = Some(value);
    }

    /// Gets the value of TroubleShootingStartTime
    pub fn get_trouble_shooting_start_time(&self) -> Option<&String> {
        self.trouble_shooting_start_time.as_ref()
    }
}

