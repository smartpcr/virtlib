// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCMetaConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCMetaConfiguration {

/// 
    #[serde(rename = "ActionAfterReboot")]
    pub action_after_reboot: Option<String>,

/// 
    #[serde(rename = "AgentId")]
    pub agent_id: Option<String>,

/// 
    #[serde(rename = "AllowModuleOverwrite")]
    pub allow_module_overwrite: Option<bool>,

/// 
    #[serde(rename = "CertificateID")]
    pub certificate_id: Option<String>,

/// 
    #[serde(rename = "ConfigurationDownloadManagers")]
    pub configuration_download_managers: Vec<OMI_ConfigurationDownloadManager>,

/// 
    #[serde(rename = "ConfigurationID")]
    pub configuration_id: Option<String>,

/// 
    #[serde(rename = "ConfigurationMode")]
    pub configuration_mode: Option<String>,

/// 
    #[serde(rename = "ConfigurationModeFrequencyMins")]
    pub configuration_mode_frequency_mins: Option<u32>,

/// 
    #[serde(rename = "Credential")]
    pub credential: Option<MSFT_Credential>,

/// 
    #[serde(rename = "DebugMode")]
    pub debug_mode: Vec<String>,

/// 
    #[serde(rename = "DownloadManagerCustomData")]
    pub download_manager_custom_data: Vec<MSFT_KeyValuePair>,

/// 
    #[serde(rename = "DownloadManagerName")]
    pub download_manager_name: Option<String>,

/// 
    #[serde(rename = "LCMCompatibleVersions")]
    pub lcmcompatible_versions: Vec<String>,

/// 
    #[serde(rename = "LCMState")]
    pub lcmstate: Option<String>,

/// 
    #[serde(rename = "LCMStateDetail")]
    pub lcmstate_detail: Option<String>,

/// 
    #[serde(rename = "LCMVersion")]
    pub lcmversion: Option<String>,

/// 
    #[serde(rename = "MaximumDownloadSizeMB")]
    pub maximum_download_size_mb: Option<u32>,

/// 
    #[serde(rename = "PartialConfigurations")]
    pub partial_configurations: Vec<MSFT_PartialConfiguration>,

/// 
    #[serde(rename = "RebootNodeIfNeeded")]
    pub reboot_node_if_needed: Option<bool>,

/// 
    #[serde(rename = "RefreshFrequencyMins")]
    pub refresh_frequency_mins: Option<u32>,

/// 
    #[serde(rename = "RefreshMode")]
    pub refresh_mode: Option<String>,

/// 
    #[serde(rename = "ReportManagers")]
    pub report_managers: Vec<OMI_ReportManager>,

/// 
    #[serde(rename = "ResourceModuleManagers")]
    pub resource_module_managers: Vec<OMI_ResourceModuleManager>,

/// 
    #[serde(rename = "SignatureValidationPolicy")]
    pub signature_validation_policy: Option<String>,

/// 
    #[serde(rename = "SignatureValidations")]
    pub signature_validations: Vec<MSFT_SignatureValidation>,

/// 
    #[serde(rename = "StatusRetentionTimeInDays")]
    pub status_retention_time_in_days: Option<u32>,
}

impl MSFT_DSCMetaConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            action_after_reboot: None,
            agent_id: None,
            allow_module_overwrite: None,
            certificate_id: None,
            configuration_download_managers: Vec::new(),
            configuration_id: None,
            configuration_mode: None,
            configuration_mode_frequency_mins: None,
            credential: None,
            debug_mode: Vec::new(),
            download_manager_custom_data: Vec::new(),
            download_manager_name: None,
            lcmcompatible_versions: Vec::new(),
            lcmstate: None,
            lcmstate_detail: None,
            lcmversion: None,
            maximum_download_size_mb: None,
            partial_configurations: Vec::new(),
            reboot_node_if_needed: None,
            refresh_frequency_mins: None,
            refresh_mode: None,
            report_managers: Vec::new(),
            resource_module_managers: Vec::new(),
            signature_validation_policy: None,
            signature_validations: Vec::new(),
            status_retention_time_in_days: None,
        }
    }


    /// Sets the value of ActionAfterReboot
    pub fn set_action_after_reboot(&mut self, value: String) {
        self.action_after_reboot = Some(value);
    }

    /// Gets the value of ActionAfterReboot
    pub fn get_action_after_reboot(&self) -> Option<&String> {
        self.action_after_reboot.as_ref()
    }

    /// Sets the value of AgentId
    pub fn set_agent_id(&mut self, value: String) {
        self.agent_id = Some(value);
    }

    /// Gets the value of AgentId
    pub fn get_agent_id(&self) -> Option<&String> {
        self.agent_id.as_ref()
    }

    /// Sets the value of AllowModuleOverwrite
    pub fn set_allow_module_overwrite(&mut self, value: bool) {
        self.allow_module_overwrite = Some(value);
    }

    /// Gets the value of AllowModuleOverwrite
    pub fn get_allow_module_overwrite(&self) -> Option<&bool> {
        self.allow_module_overwrite.as_ref()
    }

    /// Sets the value of CertificateID
    pub fn set_certificate_id(&mut self, value: String) {
        self.certificate_id = Some(value);
    }

    /// Gets the value of CertificateID
    pub fn get_certificate_id(&self) -> Option<&String> {
        self.certificate_id.as_ref()
    }

    /// Sets the value of ConfigurationDownloadManagers
    pub fn set_configuration_download_managers(&mut self, value: Vec<OMI_ConfigurationDownloadManager>) {
        self.configuration_download_managers = value;
    }

    /// Gets the value of ConfigurationDownloadManagers
    pub fn get_configuration_download_managers(&self) -> &Vec<OMI_ConfigurationDownloadManager> {
        &self.configuration_download_managers
    }

    /// Sets the value of ConfigurationID
    pub fn set_configuration_id(&mut self, value: String) {
        self.configuration_id = Some(value);
    }

    /// Gets the value of ConfigurationID
    pub fn get_configuration_id(&self) -> Option<&String> {
        self.configuration_id.as_ref()
    }

    /// Sets the value of ConfigurationMode
    pub fn set_configuration_mode(&mut self, value: String) {
        self.configuration_mode = Some(value);
    }

    /// Gets the value of ConfigurationMode
    pub fn get_configuration_mode(&self) -> Option<&String> {
        self.configuration_mode.as_ref()
    }

    /// Sets the value of ConfigurationModeFrequencyMins
    pub fn set_configuration_mode_frequency_mins(&mut self, value: u32) {
        self.configuration_mode_frequency_mins = Some(value);
    }

    /// Gets the value of ConfigurationModeFrequencyMins
    pub fn get_configuration_mode_frequency_mins(&self) -> Option<&u32> {
        self.configuration_mode_frequency_mins.as_ref()
    }

    /// Sets the value of Credential
    pub fn set_credential(&mut self, value: MSFT_Credential) {
        self.credential = Some(value);
    }

    /// Gets the value of Credential
    pub fn get_credential(&self) -> Option<&MSFT_Credential> {
        self.credential.as_ref()
    }

    /// Sets the value of DebugMode
    pub fn set_debug_mode(&mut self, value: Vec<String>) {
        self.debug_mode = value;
    }

    /// Gets the value of DebugMode
    pub fn get_debug_mode(&self) -> &Vec<String> {
        &self.debug_mode
    }

    /// Sets the value of DownloadManagerCustomData
    pub fn set_download_manager_custom_data(&mut self, value: Vec<MSFT_KeyValuePair>) {
        self.download_manager_custom_data = value;
    }

    /// Gets the value of DownloadManagerCustomData
    pub fn get_download_manager_custom_data(&self) -> &Vec<MSFT_KeyValuePair> {
        &self.download_manager_custom_data
    }

    /// Sets the value of DownloadManagerName
    pub fn set_download_manager_name(&mut self, value: String) {
        self.download_manager_name = Some(value);
    }

    /// Gets the value of DownloadManagerName
    pub fn get_download_manager_name(&self) -> Option<&String> {
        self.download_manager_name.as_ref()
    }

    /// Sets the value of LCMCompatibleVersions
    pub fn set_lcmcompatible_versions(&mut self, value: Vec<String>) {
        self.lcmcompatible_versions = value;
    }

    /// Gets the value of LCMCompatibleVersions
    pub fn get_lcmcompatible_versions(&self) -> &Vec<String> {
        &self.lcmcompatible_versions
    }

    /// Sets the value of LCMState
    pub fn set_lcmstate(&mut self, value: String) {
        self.lcmstate = Some(value);
    }

    /// Gets the value of LCMState
    pub fn get_lcmstate(&self) -> Option<&String> {
        self.lcmstate.as_ref()
    }

    /// Sets the value of LCMStateDetail
    pub fn set_lcmstate_detail(&mut self, value: String) {
        self.lcmstate_detail = Some(value);
    }

    /// Gets the value of LCMStateDetail
    pub fn get_lcmstate_detail(&self) -> Option<&String> {
        self.lcmstate_detail.as_ref()
    }

    /// Sets the value of LCMVersion
    pub fn set_lcmversion(&mut self, value: String) {
        self.lcmversion = Some(value);
    }

    /// Gets the value of LCMVersion
    pub fn get_lcmversion(&self) -> Option<&String> {
        self.lcmversion.as_ref()
    }

    /// Sets the value of MaximumDownloadSizeMB
    pub fn set_maximum_download_size_mb(&mut self, value: u32) {
        self.maximum_download_size_mb = Some(value);
    }

    /// Gets the value of MaximumDownloadSizeMB
    pub fn get_maximum_download_size_mb(&self) -> Option<&u32> {
        self.maximum_download_size_mb.as_ref()
    }

    /// Sets the value of PartialConfigurations
    pub fn set_partial_configurations(&mut self, value: Vec<MSFT_PartialConfiguration>) {
        self.partial_configurations = value;
    }

    /// Gets the value of PartialConfigurations
    pub fn get_partial_configurations(&self) -> &Vec<MSFT_PartialConfiguration> {
        &self.partial_configurations
    }

    /// Sets the value of RebootNodeIfNeeded
    pub fn set_reboot_node_if_needed(&mut self, value: bool) {
        self.reboot_node_if_needed = Some(value);
    }

    /// Gets the value of RebootNodeIfNeeded
    pub fn get_reboot_node_if_needed(&self) -> Option<&bool> {
        self.reboot_node_if_needed.as_ref()
    }

    /// Sets the value of RefreshFrequencyMins
    pub fn set_refresh_frequency_mins(&mut self, value: u32) {
        self.refresh_frequency_mins = Some(value);
    }

    /// Gets the value of RefreshFrequencyMins
    pub fn get_refresh_frequency_mins(&self) -> Option<&u32> {
        self.refresh_frequency_mins.as_ref()
    }

    /// Sets the value of RefreshMode
    pub fn set_refresh_mode(&mut self, value: String) {
        self.refresh_mode = Some(value);
    }

    /// Gets the value of RefreshMode
    pub fn get_refresh_mode(&self) -> Option<&String> {
        self.refresh_mode.as_ref()
    }

    /// Sets the value of ReportManagers
    pub fn set_report_managers(&mut self, value: Vec<OMI_ReportManager>) {
        self.report_managers = value;
    }

    /// Gets the value of ReportManagers
    pub fn get_report_managers(&self) -> &Vec<OMI_ReportManager> {
        &self.report_managers
    }

    /// Sets the value of ResourceModuleManagers
    pub fn set_resource_module_managers(&mut self, value: Vec<OMI_ResourceModuleManager>) {
        self.resource_module_managers = value;
    }

    /// Gets the value of ResourceModuleManagers
    pub fn get_resource_module_managers(&self) -> &Vec<OMI_ResourceModuleManager> {
        &self.resource_module_managers
    }

    /// Sets the value of SignatureValidationPolicy
    pub fn set_signature_validation_policy(&mut self, value: String) {
        self.signature_validation_policy = Some(value);
    }

    /// Gets the value of SignatureValidationPolicy
    pub fn get_signature_validation_policy(&self) -> Option<&String> {
        self.signature_validation_policy.as_ref()
    }

    /// Sets the value of SignatureValidations
    pub fn set_signature_validations(&mut self, value: Vec<MSFT_SignatureValidation>) {
        self.signature_validations = value;
    }

    /// Gets the value of SignatureValidations
    pub fn get_signature_validations(&self) -> &Vec<MSFT_SignatureValidation> {
        &self.signature_validations
    }

    /// Sets the value of StatusRetentionTimeInDays
    pub fn set_status_retention_time_in_days(&mut self, value: u32) {
        self.status_retention_time_in_days = Some(value);
    }

    /// Gets the value of StatusRetentionTimeInDays
    pub fn get_status_retention_time_in_days(&self) -> Option<&u32> {
        self.status_retention_time_in_days.as_ref()
    }
}

