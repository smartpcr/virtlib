// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_ApplicationManagementPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_ApplicationManagementPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "AllowX86OnIA64")]
    pub allow_x86_on_ia64: Option<bool>,

/// 
    #[serde(rename = "ApplicationId")]
    pub application_id: Option<String>,

/// 
    #[serde(rename = "ApplyCause")]
    pub apply_cause: Option<u32>,

/// 
    #[serde(rename = "AssignmentType")]
    pub assignment_type: Option<u32>,

/// 
    #[serde(rename = "Categories")]
    pub categories: Vec<String>,

/// 
    #[serde(rename = "DemandInstallable")]
    pub demand_installable: Option<bool>,

/// 
    #[serde(rename = "DeploymentLastModifyTime")]
    pub deployment_last_modify_time: Option<String>,

/// 
    #[serde(rename = "DeploymentType")]
    pub deployment_type: Option<u32>,

/// 
    #[serde(rename = "DisplayInARP")]
    pub display_in_arp: Option<bool>,

/// 
    #[serde(rename = "Eligibility")]
    pub eligibility: Option<u32>,

/// 
    #[serde(rename = "EntryType")]
    pub entry_type: Option<u32>,

/// 
    #[serde(rename = "IgnoreLanguage")]
    pub ignore_language: Option<bool>,

/// 
    #[serde(rename = "InstallationUI")]
    pub installation_ui: Option<u32>,

/// 
    #[serde(rename = "LanguageId")]
    pub language_id: Option<u32>,

/// 
    #[serde(rename = "LanguageMatch")]
    pub language_match: Option<u32>,

/// 
    #[serde(rename = "LossOfScopeAction")]
    pub loss_of_scope_action: Option<u32>,

/// 
    #[serde(rename = "MachineArchitectures")]
    pub machine_architectures: Vec<u32>,

/// 
    #[serde(rename = "OnDemandClsid")]
    pub on_demand_clsid: Option<String>,

/// 
    #[serde(rename = "OnDemandFileExtension")]
    pub on_demand_file_extension: Option<String>,

/// 
    #[serde(rename = "OnDemandProgId")]
    pub on_demand_prog_id: Option<String>,

/// 
    #[serde(rename = "PackageLocation")]
    pub package_location: Option<String>,

/// 
    #[serde(rename = "PackageType")]
    pub package_type: Option<u32>,

/// 
    #[serde(rename = "PrecedenceReason")]
    pub precedence_reason: Option<u32>,

/// 
    #[serde(rename = "ProductId")]
    pub product_id: Option<String>,

/// 
    #[serde(rename = "Publisher")]
    pub publisher: Option<String>,

/// 
    #[serde(rename = "RedeployCount")]
    pub redeploy_count: Option<u32>,

/// 
    #[serde(rename = "RemovalCause")]
    pub removal_cause: Option<u32>,

/// 
    #[serde(rename = "RemovalType")]
    pub removal_type: Option<u32>,

/// 
    #[serde(rename = "RemovingApplication")]
    pub removing_application: Option<String>,

/// 
    #[serde(rename = "ReplaceableApplications")]
    pub replaceable_applications: Vec<String>,

/// 
    #[serde(rename = "ScriptFile")]
    pub script_file: Option<String>,

/// 
    #[serde(rename = "SecurityDescriptor")]
    pub security_descriptor: Vec<u8>,

/// 
    #[serde(rename = "SupportURL")]
    pub support_url: Option<String>,

/// 
    #[serde(rename = "Transforms")]
    pub transforms: Vec<String>,

/// 
    #[serde(rename = "UninstallUnmanaged")]
    pub uninstall_unmanaged: Option<bool>,

/// 
    #[serde(rename = "UpgradeableApplications")]
    pub upgradeable_applications: Vec<String>,

/// 
    #[serde(rename = "UpgradeSettingsMandatory")]
    pub upgrade_settings_mandatory: Option<bool>,

/// 
    #[serde(rename = "VersionNumberHi")]
    pub version_number_hi: Option<u32>,

/// 
    #[serde(rename = "VersionNumberLo")]
    pub version_number_lo: Option<u32>,
}

impl RSOP_ApplicationManagementPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            allow_x86_on_ia64: None,
            application_id: None,
            apply_cause: None,
            assignment_type: None,
            categories: Vec::new(),
            demand_installable: None,
            deployment_last_modify_time: None,
            deployment_type: None,
            display_in_arp: None,
            eligibility: None,
            entry_type: None,
            ignore_language: None,
            installation_ui: None,
            language_id: None,
            language_match: None,
            loss_of_scope_action: None,
            machine_architectures: Vec::new(),
            on_demand_clsid: None,
            on_demand_file_extension: None,
            on_demand_prog_id: None,
            package_location: None,
            package_type: None,
            precedence_reason: None,
            product_id: None,
            publisher: None,
            redeploy_count: None,
            removal_cause: None,
            removal_type: None,
            removing_application: None,
            replaceable_applications: Vec::new(),
            script_file: None,
            security_descriptor: Vec::new(),
            support_url: None,
            transforms: Vec::new(),
            uninstall_unmanaged: None,
            upgradeable_applications: Vec::new(),
            upgrade_settings_mandatory: None,
            version_number_hi: None,
            version_number_lo: None,
        }
    }


    /// Sets the value of AllowX86OnIA64
    pub fn set_allow_x86_on_ia64(&mut self, value: bool) {
        self.allow_x86_on_ia64 = Some(value);
    }

    /// Gets the value of AllowX86OnIA64
    pub fn get_allow_x86_on_ia64(&self) -> Option<&bool> {
        self.allow_x86_on_ia64.as_ref()
    }

    /// Sets the value of ApplicationId
    pub fn set_application_id(&mut self, value: String) {
        self.application_id = Some(value);
    }

    /// Gets the value of ApplicationId
    pub fn get_application_id(&self) -> Option<&String> {
        self.application_id.as_ref()
    }

    /// Sets the value of ApplyCause
    pub fn set_apply_cause(&mut self, value: u32) {
        self.apply_cause = Some(value);
    }

    /// Gets the value of ApplyCause
    pub fn get_apply_cause(&self) -> Option<&u32> {
        self.apply_cause.as_ref()
    }

    /// Sets the value of AssignmentType
    pub fn set_assignment_type(&mut self, value: u32) {
        self.assignment_type = Some(value);
    }

    /// Gets the value of AssignmentType
    pub fn get_assignment_type(&self) -> Option<&u32> {
        self.assignment_type.as_ref()
    }

    /// Sets the value of Categories
    pub fn set_categories(&mut self, value: Vec<String>) {
        self.categories = value;
    }

    /// Gets the value of Categories
    pub fn get_categories(&self) -> &Vec<String> {
        &self.categories
    }

    /// Sets the value of DemandInstallable
    pub fn set_demand_installable(&mut self, value: bool) {
        self.demand_installable = Some(value);
    }

    /// Gets the value of DemandInstallable
    pub fn get_demand_installable(&self) -> Option<&bool> {
        self.demand_installable.as_ref()
    }

    /// Sets the value of DeploymentLastModifyTime
    pub fn set_deployment_last_modify_time(&mut self, value: String) {
        self.deployment_last_modify_time = Some(value);
    }

    /// Gets the value of DeploymentLastModifyTime
    pub fn get_deployment_last_modify_time(&self) -> Option<&String> {
        self.deployment_last_modify_time.as_ref()
    }

    /// Sets the value of DeploymentType
    pub fn set_deployment_type(&mut self, value: u32) {
        self.deployment_type = Some(value);
    }

    /// Gets the value of DeploymentType
    pub fn get_deployment_type(&self) -> Option<&u32> {
        self.deployment_type.as_ref()
    }

    /// Sets the value of DisplayInARP
    pub fn set_display_in_arp(&mut self, value: bool) {
        self.display_in_arp = Some(value);
    }

    /// Gets the value of DisplayInARP
    pub fn get_display_in_arp(&self) -> Option<&bool> {
        self.display_in_arp.as_ref()
    }

    /// Sets the value of Eligibility
    pub fn set_eligibility(&mut self, value: u32) {
        self.eligibility = Some(value);
    }

    /// Gets the value of Eligibility
    pub fn get_eligibility(&self) -> Option<&u32> {
        self.eligibility.as_ref()
    }

    /// Sets the value of EntryType
    pub fn set_entry_type(&mut self, value: u32) {
        self.entry_type = Some(value);
    }

    /// Gets the value of EntryType
    pub fn get_entry_type(&self) -> Option<&u32> {
        self.entry_type.as_ref()
    }

    /// Sets the value of IgnoreLanguage
    pub fn set_ignore_language(&mut self, value: bool) {
        self.ignore_language = Some(value);
    }

    /// Gets the value of IgnoreLanguage
    pub fn get_ignore_language(&self) -> Option<&bool> {
        self.ignore_language.as_ref()
    }

    /// Sets the value of InstallationUI
    pub fn set_installation_ui(&mut self, value: u32) {
        self.installation_ui = Some(value);
    }

    /// Gets the value of InstallationUI
    pub fn get_installation_ui(&self) -> Option<&u32> {
        self.installation_ui.as_ref()
    }

    /// Sets the value of LanguageId
    pub fn set_language_id(&mut self, value: u32) {
        self.language_id = Some(value);
    }

    /// Gets the value of LanguageId
    pub fn get_language_id(&self) -> Option<&u32> {
        self.language_id.as_ref()
    }

    /// Sets the value of LanguageMatch
    pub fn set_language_match(&mut self, value: u32) {
        self.language_match = Some(value);
    }

    /// Gets the value of LanguageMatch
    pub fn get_language_match(&self) -> Option<&u32> {
        self.language_match.as_ref()
    }

    /// Sets the value of LossOfScopeAction
    pub fn set_loss_of_scope_action(&mut self, value: u32) {
        self.loss_of_scope_action = Some(value);
    }

    /// Gets the value of LossOfScopeAction
    pub fn get_loss_of_scope_action(&self) -> Option<&u32> {
        self.loss_of_scope_action.as_ref()
    }

    /// Sets the value of MachineArchitectures
    pub fn set_machine_architectures(&mut self, value: Vec<u32>) {
        self.machine_architectures = value;
    }

    /// Gets the value of MachineArchitectures
    pub fn get_machine_architectures(&self) -> &Vec<u32> {
        &self.machine_architectures
    }

    /// Sets the value of OnDemandClsid
    pub fn set_on_demand_clsid(&mut self, value: String) {
        self.on_demand_clsid = Some(value);
    }

    /// Gets the value of OnDemandClsid
    pub fn get_on_demand_clsid(&self) -> Option<&String> {
        self.on_demand_clsid.as_ref()
    }

    /// Sets the value of OnDemandFileExtension
    pub fn set_on_demand_file_extension(&mut self, value: String) {
        self.on_demand_file_extension = Some(value);
    }

    /// Gets the value of OnDemandFileExtension
    pub fn get_on_demand_file_extension(&self) -> Option<&String> {
        self.on_demand_file_extension.as_ref()
    }

    /// Sets the value of OnDemandProgId
    pub fn set_on_demand_prog_id(&mut self, value: String) {
        self.on_demand_prog_id = Some(value);
    }

    /// Gets the value of OnDemandProgId
    pub fn get_on_demand_prog_id(&self) -> Option<&String> {
        self.on_demand_prog_id.as_ref()
    }

    /// Sets the value of PackageLocation
    pub fn set_package_location(&mut self, value: String) {
        self.package_location = Some(value);
    }

    /// Gets the value of PackageLocation
    pub fn get_package_location(&self) -> Option<&String> {
        self.package_location.as_ref()
    }

    /// Sets the value of PackageType
    pub fn set_package_type(&mut self, value: u32) {
        self.package_type = Some(value);
    }

    /// Gets the value of PackageType
    pub fn get_package_type(&self) -> Option<&u32> {
        self.package_type.as_ref()
    }

    /// Sets the value of PrecedenceReason
    pub fn set_precedence_reason(&mut self, value: u32) {
        self.precedence_reason = Some(value);
    }

    /// Gets the value of PrecedenceReason
    pub fn get_precedence_reason(&self) -> Option<&u32> {
        self.precedence_reason.as_ref()
    }

    /// Sets the value of ProductId
    pub fn set_product_id(&mut self, value: String) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductId
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }

    /// Sets the value of Publisher
    pub fn set_publisher(&mut self, value: String) {
        self.publisher = Some(value);
    }

    /// Gets the value of Publisher
    pub fn get_publisher(&self) -> Option<&String> {
        self.publisher.as_ref()
    }

    /// Sets the value of RedeployCount
    pub fn set_redeploy_count(&mut self, value: u32) {
        self.redeploy_count = Some(value);
    }

    /// Gets the value of RedeployCount
    pub fn get_redeploy_count(&self) -> Option<&u32> {
        self.redeploy_count.as_ref()
    }

    /// Sets the value of RemovalCause
    pub fn set_removal_cause(&mut self, value: u32) {
        self.removal_cause = Some(value);
    }

    /// Gets the value of RemovalCause
    pub fn get_removal_cause(&self) -> Option<&u32> {
        self.removal_cause.as_ref()
    }

    /// Sets the value of RemovalType
    pub fn set_removal_type(&mut self, value: u32) {
        self.removal_type = Some(value);
    }

    /// Gets the value of RemovalType
    pub fn get_removal_type(&self) -> Option<&u32> {
        self.removal_type.as_ref()
    }

    /// Sets the value of RemovingApplication
    pub fn set_removing_application(&mut self, value: String) {
        self.removing_application = Some(value);
    }

    /// Gets the value of RemovingApplication
    pub fn get_removing_application(&self) -> Option<&String> {
        self.removing_application.as_ref()
    }

    /// Sets the value of ReplaceableApplications
    pub fn set_replaceable_applications(&mut self, value: Vec<String>) {
        self.replaceable_applications = value;
    }

    /// Gets the value of ReplaceableApplications
    pub fn get_replaceable_applications(&self) -> &Vec<String> {
        &self.replaceable_applications
    }

    /// Sets the value of ScriptFile
    pub fn set_script_file(&mut self, value: String) {
        self.script_file = Some(value);
    }

    /// Gets the value of ScriptFile
    pub fn get_script_file(&self) -> Option<&String> {
        self.script_file.as_ref()
    }

    /// Sets the value of SecurityDescriptor
    pub fn set_security_descriptor(&mut self, value: Vec<u8>) {
        self.security_descriptor = value;
    }

    /// Gets the value of SecurityDescriptor
    pub fn get_security_descriptor(&self) -> &Vec<u8> {
        &self.security_descriptor
    }

    /// Sets the value of SupportURL
    pub fn set_support_url(&mut self, value: String) {
        self.support_url = Some(value);
    }

    /// Gets the value of SupportURL
    pub fn get_support_url(&self) -> Option<&String> {
        self.support_url.as_ref()
    }

    /// Sets the value of Transforms
    pub fn set_transforms(&mut self, value: Vec<String>) {
        self.transforms = value;
    }

    /// Gets the value of Transforms
    pub fn get_transforms(&self) -> &Vec<String> {
        &self.transforms
    }

    /// Sets the value of UninstallUnmanaged
    pub fn set_uninstall_unmanaged(&mut self, value: bool) {
        self.uninstall_unmanaged = Some(value);
    }

    /// Gets the value of UninstallUnmanaged
    pub fn get_uninstall_unmanaged(&self) -> Option<&bool> {
        self.uninstall_unmanaged.as_ref()
    }

    /// Sets the value of UpgradeableApplications
    pub fn set_upgradeable_applications(&mut self, value: Vec<String>) {
        self.upgradeable_applications = value;
    }

    /// Gets the value of UpgradeableApplications
    pub fn get_upgradeable_applications(&self) -> &Vec<String> {
        &self.upgradeable_applications
    }

    /// Sets the value of UpgradeSettingsMandatory
    pub fn set_upgrade_settings_mandatory(&mut self, value: bool) {
        self.upgrade_settings_mandatory = Some(value);
    }

    /// Gets the value of UpgradeSettingsMandatory
    pub fn get_upgrade_settings_mandatory(&self) -> Option<&bool> {
        self.upgrade_settings_mandatory.as_ref()
    }

    /// Sets the value of VersionNumberHi
    pub fn set_version_number_hi(&mut self, value: u32) {
        self.version_number_hi = Some(value);
    }

    /// Gets the value of VersionNumberHi
    pub fn get_version_number_hi(&self) -> Option<&u32> {
        self.version_number_hi.as_ref()
    }

    /// Sets the value of VersionNumberLo
    pub fn set_version_number_lo(&mut self, value: u32) {
        self.version_number_lo = Some(value);
    }

    /// Gets the value of VersionNumberLo
    pub fn get_version_number_lo(&self) -> Option<&u32> {
        self.version_number_lo.as_ref()
    }
}

