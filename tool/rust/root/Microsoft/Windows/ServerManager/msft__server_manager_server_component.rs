// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerManagerServerComponent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerManagerServerComponent {

/// 
    #[serde(rename = "BestPracticeModels")]
    pub best_practice_models: Vec<String>,

/// 
    #[serde(rename = "ConfigurationStatus")]
    pub configuration_status: Option<u8>,

/// 
    #[serde(rename = "Deploys")]
    pub deploys: Vec<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Descriptor")]
    pub descriptor: Option<serde_json::Value>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "EventQuery")]
    pub event_query: Option<String>,

/// 
    #[serde(rename = "FeatureType")]
    pub feature_type: Option<u8>,

/// 
    #[serde(rename = "Installed")]
    pub installed: Option<u8>,

/// 
    #[serde(rename = "InstallWithParentByDefault")]
    pub install_with_parent_by_default: Option<bool>,

/// 
    #[serde(rename = "MajorVersion")]
    pub major_version: Option<i32>,

/// 
    #[serde(rename = "MinorVersion")]
    pub minor_version: Option<i32>,

/// 
    #[serde(rename = "MutualExclusions")]
    pub mutual_exclusions: Vec<String>,

/// 
    #[serde(rename = "NonAncestorDependencies")]
    pub non_ancestor_dependencies: Vec<String>,

/// 
    #[serde(rename = "NumericId")]
    pub numeric_id: Option<i32>,

/// 
    #[serde(rename = "OptionalCompanions")]
    pub optional_companions: Vec<MSFT_OptionalCompanion>,

/// 
    #[serde(rename = "ParentName")]
    pub parent_name: Option<String>,

/// 
    #[serde(rename = "PostInstallDescription")]
    pub post_install_description: Option<String>,

/// 
    #[serde(rename = "PostUninstallDescription")]
    pub post_uninstall_description: Option<String>,

/// 
    #[serde(rename = "SubFeatures")]
    pub sub_features: Vec<String>,

/// 
    #[serde(rename = "SystemServices")]
    pub system_services: Vec<MSFT_ServiceToMonitor>,

/// 
    #[serde(rename = "UniqueName")]
    pub unique_name: Option<String>,
}

impl MSFT_ServerManagerServerComponent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            best_practice_models: Vec::new(),
            configuration_status: None,
            deploys: Vec::new(),
            description: None,
            descriptor: None,
            display_name: None,
            event_query: None,
            feature_type: None,
            installed: None,
            install_with_parent_by_default: None,
            major_version: None,
            minor_version: None,
            mutual_exclusions: Vec::new(),
            non_ancestor_dependencies: Vec::new(),
            numeric_id: None,
            optional_companions: Vec::new(),
            parent_name: None,
            post_install_description: None,
            post_uninstall_description: None,
            sub_features: Vec::new(),
            system_services: Vec::new(),
            unique_name: None,
        }
    }


    /// Sets the value of BestPracticeModels
    pub fn set_best_practice_models(&mut self, value: Vec<String>) {
        self.best_practice_models = value;
    }

    /// Gets the value of BestPracticeModels
    pub fn get_best_practice_models(&self) -> &Vec<String> {
        &self.best_practice_models
    }

    /// Sets the value of ConfigurationStatus
    pub fn set_configuration_status(&mut self, value: u8) {
        self.configuration_status = Some(value);
    }

    /// Gets the value of ConfigurationStatus
    pub fn get_configuration_status(&self) -> Option<&u8> {
        self.configuration_status.as_ref()
    }

    /// Sets the value of Deploys
    pub fn set_deploys(&mut self, value: Vec<String>) {
        self.deploys = value;
    }

    /// Gets the value of Deploys
    pub fn get_deploys(&self) -> &Vec<String> {
        &self.deploys
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Descriptor
    pub fn set_descriptor(&mut self, value: serde_json::Value) {
        self.descriptor = Some(value);
    }

    /// Gets the value of Descriptor
    pub fn get_descriptor(&self) -> Option<&serde_json::Value> {
        self.descriptor.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of EventQuery
    pub fn set_event_query(&mut self, value: String) {
        self.event_query = Some(value);
    }

    /// Gets the value of EventQuery
    pub fn get_event_query(&self) -> Option<&String> {
        self.event_query.as_ref()
    }

    /// Sets the value of FeatureType
    pub fn set_feature_type(&mut self, value: u8) {
        self.feature_type = Some(value);
    }

    /// Gets the value of FeatureType
    pub fn get_feature_type(&self) -> Option<&u8> {
        self.feature_type.as_ref()
    }

    /// Sets the value of Installed
    pub fn set_installed(&mut self, value: u8) {
        self.installed = Some(value);
    }

    /// Gets the value of Installed
    pub fn get_installed(&self) -> Option<&u8> {
        self.installed.as_ref()
    }

    /// Sets the value of InstallWithParentByDefault
    pub fn set_install_with_parent_by_default(&mut self, value: bool) {
        self.install_with_parent_by_default = Some(value);
    }

    /// Gets the value of InstallWithParentByDefault
    pub fn get_install_with_parent_by_default(&self) -> Option<&bool> {
        self.install_with_parent_by_default.as_ref()
    }

    /// Sets the value of MajorVersion
    pub fn set_major_version(&mut self, value: i32) {
        self.major_version = Some(value);
    }

    /// Gets the value of MajorVersion
    pub fn get_major_version(&self) -> Option<&i32> {
        self.major_version.as_ref()
    }

    /// Sets the value of MinorVersion
    pub fn set_minor_version(&mut self, value: i32) {
        self.minor_version = Some(value);
    }

    /// Gets the value of MinorVersion
    pub fn get_minor_version(&self) -> Option<&i32> {
        self.minor_version.as_ref()
    }

    /// Sets the value of MutualExclusions
    pub fn set_mutual_exclusions(&mut self, value: Vec<String>) {
        self.mutual_exclusions = value;
    }

    /// Gets the value of MutualExclusions
    pub fn get_mutual_exclusions(&self) -> &Vec<String> {
        &self.mutual_exclusions
    }

    /// Sets the value of NonAncestorDependencies
    pub fn set_non_ancestor_dependencies(&mut self, value: Vec<String>) {
        self.non_ancestor_dependencies = value;
    }

    /// Gets the value of NonAncestorDependencies
    pub fn get_non_ancestor_dependencies(&self) -> &Vec<String> {
        &self.non_ancestor_dependencies
    }

    /// Sets the value of NumericId
    pub fn set_numeric_id(&mut self, value: i32) {
        self.numeric_id = Some(value);
    }

    /// Gets the value of NumericId
    pub fn get_numeric_id(&self) -> Option<&i32> {
        self.numeric_id.as_ref()
    }

    /// Sets the value of OptionalCompanions
    pub fn set_optional_companions(&mut self, value: Vec<MSFT_OptionalCompanion>) {
        self.optional_companions = value;
    }

    /// Gets the value of OptionalCompanions
    pub fn get_optional_companions(&self) -> &Vec<MSFT_OptionalCompanion> {
        &self.optional_companions
    }

    /// Sets the value of ParentName
    pub fn set_parent_name(&mut self, value: String) {
        self.parent_name = Some(value);
    }

    /// Gets the value of ParentName
    pub fn get_parent_name(&self) -> Option<&String> {
        self.parent_name.as_ref()
    }

    /// Sets the value of PostInstallDescription
    pub fn set_post_install_description(&mut self, value: String) {
        self.post_install_description = Some(value);
    }

    /// Gets the value of PostInstallDescription
    pub fn get_post_install_description(&self) -> Option<&String> {
        self.post_install_description.as_ref()
    }

    /// Sets the value of PostUninstallDescription
    pub fn set_post_uninstall_description(&mut self, value: String) {
        self.post_uninstall_description = Some(value);
    }

    /// Gets the value of PostUninstallDescription
    pub fn get_post_uninstall_description(&self) -> Option<&String> {
        self.post_uninstall_description.as_ref()
    }

    /// Sets the value of SubFeatures
    pub fn set_sub_features(&mut self, value: Vec<String>) {
        self.sub_features = value;
    }

    /// Gets the value of SubFeatures
    pub fn get_sub_features(&self) -> &Vec<String> {
        &self.sub_features
    }

    /// Sets the value of SystemServices
    pub fn set_system_services(&mut self, value: Vec<MSFT_ServiceToMonitor>) {
        self.system_services = value;
    }

    /// Gets the value of SystemServices
    pub fn get_system_services(&self) -> &Vec<MSFT_ServiceToMonitor> {
        &self.system_services
    }

    /// Sets the value of UniqueName
    pub fn set_unique_name(&mut self, value: String) {
        self.unique_name = Some(value);
    }

    /// Gets the value of UniqueName
    pub fn get_unique_name(&self) -> Option<&String> {
        self.unique_name.as_ref()
    }
}

