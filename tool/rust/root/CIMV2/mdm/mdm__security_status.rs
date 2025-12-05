// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_SecurityStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_SecurityStatus {

/// 
    #[serde(rename = "AntiVirusSignatureStatus")]
    pub anti_virus_signature_status: Option<u32>,

/// 
    #[serde(rename = "AntiVirusStatus")]
    pub anti_virus_status: Option<u32>,

/// 
    #[serde(rename = "ApplicationContentUriRules")]
    pub application_content_uri_rules: Option<String>,

/// 
    #[serde(rename = "AutoUpdateStatus")]
    pub auto_update_status: Option<u32>,

/// 
    #[serde(rename = "FirewallStatus")]
    pub firewall_status: Option<u32>,

/// 
    #[serde(rename = "IsMicrosoftAccountOptional")]
    pub is_microsoft_account_optional: Option<bool>,

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,

/// 
    #[serde(rename = "MaintenanceScheduleAllowWakeup")]
    pub maintenance_schedule_allow_wakeup: Option<bool>,

/// 
    #[serde(rename = "MaintenanceScheduleDelayPattern")]
    pub maintenance_schedule_delay_pattern: Option<String>,

/// 
    #[serde(rename = "MaintenanceScheduleStartHour")]
    pub maintenance_schedule_start_hour: Option<u32>,

/// 
    #[serde(rename = "RequireEncryption")]
    pub require_encryption: Option<bool>,
}

impl MDM_SecurityStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            anti_virus_signature_status: None,
            anti_virus_status: None,
            application_content_uri_rules: None,
            auto_update_status: None,
            firewall_status: None,
            is_microsoft_account_optional: None,
            key: None,
            maintenance_schedule_allow_wakeup: None,
            maintenance_schedule_delay_pattern: None,
            maintenance_schedule_start_hour: None,
            require_encryption: None,
        }
    }


    /// Sets the value of AntiVirusSignatureStatus
    pub fn set_anti_virus_signature_status(&mut self, value: u32) {
        self.anti_virus_signature_status = Some(value);
    }

    /// Gets the value of AntiVirusSignatureStatus
    pub fn get_anti_virus_signature_status(&self) -> Option<&u32> {
        self.anti_virus_signature_status.as_ref()
    }

    /// Sets the value of AntiVirusStatus
    pub fn set_anti_virus_status(&mut self, value: u32) {
        self.anti_virus_status = Some(value);
    }

    /// Gets the value of AntiVirusStatus
    pub fn get_anti_virus_status(&self) -> Option<&u32> {
        self.anti_virus_status.as_ref()
    }

    /// Sets the value of ApplicationContentUriRules
    pub fn set_application_content_uri_rules(&mut self, value: String) {
        self.application_content_uri_rules = Some(value);
    }

    /// Gets the value of ApplicationContentUriRules
    pub fn get_application_content_uri_rules(&self) -> Option<&String> {
        self.application_content_uri_rules.as_ref()
    }

    /// Sets the value of AutoUpdateStatus
    pub fn set_auto_update_status(&mut self, value: u32) {
        self.auto_update_status = Some(value);
    }

    /// Gets the value of AutoUpdateStatus
    pub fn get_auto_update_status(&self) -> Option<&u32> {
        self.auto_update_status.as_ref()
    }

    /// Sets the value of FirewallStatus
    pub fn set_firewall_status(&mut self, value: u32) {
        self.firewall_status = Some(value);
    }

    /// Gets the value of FirewallStatus
    pub fn get_firewall_status(&self) -> Option<&u32> {
        self.firewall_status.as_ref()
    }

    /// Sets the value of IsMicrosoftAccountOptional
    pub fn set_is_microsoft_account_optional(&mut self, value: bool) {
        self.is_microsoft_account_optional = Some(value);
    }

    /// Gets the value of IsMicrosoftAccountOptional
    pub fn get_is_microsoft_account_optional(&self) -> Option<&bool> {
        self.is_microsoft_account_optional.as_ref()
    }

    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

    /// Sets the value of MaintenanceScheduleAllowWakeup
    pub fn set_maintenance_schedule_allow_wakeup(&mut self, value: bool) {
        self.maintenance_schedule_allow_wakeup = Some(value);
    }

    /// Gets the value of MaintenanceScheduleAllowWakeup
    pub fn get_maintenance_schedule_allow_wakeup(&self) -> Option<&bool> {
        self.maintenance_schedule_allow_wakeup.as_ref()
    }

    /// Sets the value of MaintenanceScheduleDelayPattern
    pub fn set_maintenance_schedule_delay_pattern(&mut self, value: String) {
        self.maintenance_schedule_delay_pattern = Some(value);
    }

    /// Gets the value of MaintenanceScheduleDelayPattern
    pub fn get_maintenance_schedule_delay_pattern(&self) -> Option<&String> {
        self.maintenance_schedule_delay_pattern.as_ref()
    }

    /// Sets the value of MaintenanceScheduleStartHour
    pub fn set_maintenance_schedule_start_hour(&mut self, value: u32) {
        self.maintenance_schedule_start_hour = Some(value);
    }

    /// Gets the value of MaintenanceScheduleStartHour
    pub fn get_maintenance_schedule_start_hour(&self) -> Option<&u32> {
        self.maintenance_schedule_start_hour.as_ref()
    }

    /// Sets the value of RequireEncryption
    pub fn set_require_encryption(&mut self, value: bool) {
        self.require_encryption = Some(value);
    }

    /// Gets the value of RequireEncryption
    pub fn get_require_encryption(&self) -> Option<&bool> {
        self.require_encryption.as_ref()
    }
}

