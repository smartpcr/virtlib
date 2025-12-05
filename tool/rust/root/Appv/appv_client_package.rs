// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Appv
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AppvClientPackage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppvClientPackage {

/// 
    #[serde(rename = "Assets")]
    pub assets: Vec<String>,

/// 
    #[serde(rename = "DeploymentMachineData")]
    pub deployment_machine_data: Option<String>,

/// 
    #[serde(rename = "DeploymentUserData")]
    pub deployment_user_data: Option<String>,

/// 
    #[serde(rename = "GlobalPending")]
    pub global_pending: Option<bool>,

/// 
    #[serde(rename = "HasAssetIntelligence")]
    pub has_asset_intelligence: Option<bool>,

/// 
    #[serde(rename = "InUse")]
    pub in_use: Option<bool>,

/// 
    #[serde(rename = "IsPublishedGlobally")]
    pub is_published_globally: Option<bool>,

/// 
    #[serde(rename = "IsPublishedToUser")]
    pub is_published_to_user: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PackageId")]
    pub package_id: Option<String>,

/// 
    #[serde(rename = "PackageSize")]
    pub package_size: Option<u64>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "PercentLoaded")]
    pub percent_loaded: Option<u16>,

/// 
    #[serde(rename = "UserConfigurationData")]
    pub user_configuration_data: Option<String>,

/// 
    #[serde(rename = "UserPending")]
    pub user_pending: Option<bool>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,

/// 
    #[serde(rename = "VersionId")]
    pub version_id: Option<String>,
}

impl AppvClientPackage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            assets: Vec::new(),
            deployment_machine_data: None,
            deployment_user_data: None,
            global_pending: None,
            has_asset_intelligence: None,
            in_use: None,
            is_published_globally: None,
            is_published_to_user: None,
            name: None,
            package_id: None,
            package_size: None,
            path: None,
            percent_loaded: None,
            user_configuration_data: None,
            user_pending: None,
            version: None,
            version_id: None,
        }
    }


    /// Sets the value of Assets
    pub fn set_assets(&mut self, value: Vec<String>) {
        self.assets = value;
    }

    /// Gets the value of Assets
    pub fn get_assets(&self) -> &Vec<String> {
        &self.assets
    }

    /// Sets the value of DeploymentMachineData
    pub fn set_deployment_machine_data(&mut self, value: String) {
        self.deployment_machine_data = Some(value);
    }

    /// Gets the value of DeploymentMachineData
    pub fn get_deployment_machine_data(&self) -> Option<&String> {
        self.deployment_machine_data.as_ref()
    }

    /// Sets the value of DeploymentUserData
    pub fn set_deployment_user_data(&mut self, value: String) {
        self.deployment_user_data = Some(value);
    }

    /// Gets the value of DeploymentUserData
    pub fn get_deployment_user_data(&self) -> Option<&String> {
        self.deployment_user_data.as_ref()
    }

    /// Sets the value of GlobalPending
    pub fn set_global_pending(&mut self, value: bool) {
        self.global_pending = Some(value);
    }

    /// Gets the value of GlobalPending
    pub fn get_global_pending(&self) -> Option<&bool> {
        self.global_pending.as_ref()
    }

    /// Sets the value of HasAssetIntelligence
    pub fn set_has_asset_intelligence(&mut self, value: bool) {
        self.has_asset_intelligence = Some(value);
    }

    /// Gets the value of HasAssetIntelligence
    pub fn get_has_asset_intelligence(&self) -> Option<&bool> {
        self.has_asset_intelligence.as_ref()
    }

    /// Sets the value of InUse
    pub fn set_in_use(&mut self, value: bool) {
        self.in_use = Some(value);
    }

    /// Gets the value of InUse
    pub fn get_in_use(&self) -> Option<&bool> {
        self.in_use.as_ref()
    }

    /// Sets the value of IsPublishedGlobally
    pub fn set_is_published_globally(&mut self, value: bool) {
        self.is_published_globally = Some(value);
    }

    /// Gets the value of IsPublishedGlobally
    pub fn get_is_published_globally(&self) -> Option<&bool> {
        self.is_published_globally.as_ref()
    }

    /// Sets the value of IsPublishedToUser
    pub fn set_is_published_to_user(&mut self, value: bool) {
        self.is_published_to_user = Some(value);
    }

    /// Gets the value of IsPublishedToUser
    pub fn get_is_published_to_user(&self) -> Option<&bool> {
        self.is_published_to_user.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PackageId
    pub fn set_package_id(&mut self, value: String) {
        self.package_id = Some(value);
    }

    /// Gets the value of PackageId
    pub fn get_package_id(&self) -> Option<&String> {
        self.package_id.as_ref()
    }

    /// Sets the value of PackageSize
    pub fn set_package_size(&mut self, value: u64) {
        self.package_size = Some(value);
    }

    /// Gets the value of PackageSize
    pub fn get_package_size(&self) -> Option<&u64> {
        self.package_size.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of PercentLoaded
    pub fn set_percent_loaded(&mut self, value: u16) {
        self.percent_loaded = Some(value);
    }

    /// Gets the value of PercentLoaded
    pub fn get_percent_loaded(&self) -> Option<&u16> {
        self.percent_loaded.as_ref()
    }

    /// Sets the value of UserConfigurationData
    pub fn set_user_configuration_data(&mut self, value: String) {
        self.user_configuration_data = Some(value);
    }

    /// Gets the value of UserConfigurationData
    pub fn get_user_configuration_data(&self) -> Option<&String> {
        self.user_configuration_data.as_ref()
    }

    /// Sets the value of UserPending
    pub fn set_user_pending(&mut self, value: bool) {
        self.user_pending = Some(value);
    }

    /// Gets the value of UserPending
    pub fn get_user_pending(&self) -> Option<&bool> {
        self.user_pending.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    /// Sets the value of VersionId
    pub fn set_version_id(&mut self, value: String) {
        self.version_id = Some(value);
    }

    /// Gets the value of VersionId
    pub fn get_version_id(&self) -> Option<&String> {
        self.version_id.as_ref()
    }
}

