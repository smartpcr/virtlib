// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Appv
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AppvClientApplication struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppvClientApplication {

/// 
    #[serde(rename = "ApplicationId")]
    pub application_id: Option<String>,

/// 
    #[serde(rename = "EnabledForUser")]
    pub enabled_for_user: Option<bool>,

/// 
    #[serde(rename = "EnabledGlobally")]
    pub enabled_globally: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PackageId")]
    pub package_id: Option<String>,

/// 
    #[serde(rename = "PackageVersionId")]
    pub package_version_id: Option<String>,

/// 
    #[serde(rename = "TargetPath")]
    pub target_path: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl AppvClientApplication {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application_id: None,
            enabled_for_user: None,
            enabled_globally: None,
            name: None,
            package_id: None,
            package_version_id: None,
            target_path: None,
            version: None,
        }
    }


    /// Sets the value of ApplicationId
    pub fn set_application_id(&mut self, value: String) {
        self.application_id = Some(value);
    }

    /// Gets the value of ApplicationId
    pub fn get_application_id(&self) -> Option<&String> {
        self.application_id.as_ref()
    }

    /// Sets the value of EnabledForUser
    pub fn set_enabled_for_user(&mut self, value: bool) {
        self.enabled_for_user = Some(value);
    }

    /// Gets the value of EnabledForUser
    pub fn get_enabled_for_user(&self) -> Option<&bool> {
        self.enabled_for_user.as_ref()
    }

    /// Sets the value of EnabledGlobally
    pub fn set_enabled_globally(&mut self, value: bool) {
        self.enabled_globally = Some(value);
    }

    /// Gets the value of EnabledGlobally
    pub fn get_enabled_globally(&self) -> Option<&bool> {
        self.enabled_globally.as_ref()
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

    /// Sets the value of PackageVersionId
    pub fn set_package_version_id(&mut self, value: String) {
        self.package_version_id = Some(value);
    }

    /// Gets the value of PackageVersionId
    pub fn get_package_version_id(&self) -> Option<&String> {
        self.package_version_id.as_ref()
    }

    /// Sets the value of TargetPath
    pub fn set_target_path(&mut self, value: String) {
        self.target_path = Some(value);
    }

    /// Gets the value of TargetPath
    pub fn get_target_path(&self) -> Option<&String> {
        self.target_path.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: String) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&String> {
        self.version.as_ref()
    }
}

