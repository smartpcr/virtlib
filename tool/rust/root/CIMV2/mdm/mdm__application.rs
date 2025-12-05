// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Application struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Application {

/// 
    #[serde(rename = "Dependencies")]
    pub dependencies: Option<String>,

/// 
    #[serde(rename = "InstallPath")]
    pub install_path: Option<String>,

/// 
    #[serde(rename = "IsBundle")]
    pub is_bundle: Option<bool>,

/// 
    #[serde(rename = "IsDevelopmentMode")]
    pub is_development_mode: Option<bool>,

/// 
    #[serde(rename = "IsFramework")]
    pub is_framework: Option<bool>,

/// 
    #[serde(rename = "IsResourcePackage")]
    pub is_resource_package: Option<bool>,

/// 
    #[serde(rename = "PackageFullName")]
    pub package_full_name: Option<String>,

/// 
    #[serde(rename = "PackageName")]
    pub package_name: Option<String>,

/// 
    #[serde(rename = "PackagePublisher")]
    pub package_publisher: Option<String>,

/// 
    #[serde(rename = "PackageVersion")]
    pub package_version: Option<String>,

/// 
    #[serde(rename = "UserSID")]
    pub user_sid: Option<String>,
}

impl MDM_Application {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dependencies: None,
            install_path: None,
            is_bundle: None,
            is_development_mode: None,
            is_framework: None,
            is_resource_package: None,
            package_full_name: None,
            package_name: None,
            package_publisher: None,
            package_version: None,
            user_sid: None,
        }
    }


    /// Sets the value of Dependencies
    pub fn set_dependencies(&mut self, value: String) {
        self.dependencies = Some(value);
    }

    /// Gets the value of Dependencies
    pub fn get_dependencies(&self) -> Option<&String> {
        self.dependencies.as_ref()
    }

    /// Sets the value of InstallPath
    pub fn set_install_path(&mut self, value: String) {
        self.install_path = Some(value);
    }

    /// Gets the value of InstallPath
    pub fn get_install_path(&self) -> Option<&String> {
        self.install_path.as_ref()
    }

    /// Sets the value of IsBundle
    pub fn set_is_bundle(&mut self, value: bool) {
        self.is_bundle = Some(value);
    }

    /// Gets the value of IsBundle
    pub fn get_is_bundle(&self) -> Option<&bool> {
        self.is_bundle.as_ref()
    }

    /// Sets the value of IsDevelopmentMode
    pub fn set_is_development_mode(&mut self, value: bool) {
        self.is_development_mode = Some(value);
    }

    /// Gets the value of IsDevelopmentMode
    pub fn get_is_development_mode(&self) -> Option<&bool> {
        self.is_development_mode.as_ref()
    }

    /// Sets the value of IsFramework
    pub fn set_is_framework(&mut self, value: bool) {
        self.is_framework = Some(value);
    }

    /// Gets the value of IsFramework
    pub fn get_is_framework(&self) -> Option<&bool> {
        self.is_framework.as_ref()
    }

    /// Sets the value of IsResourcePackage
    pub fn set_is_resource_package(&mut self, value: bool) {
        self.is_resource_package = Some(value);
    }

    /// Gets the value of IsResourcePackage
    pub fn get_is_resource_package(&self) -> Option<&bool> {
        self.is_resource_package.as_ref()
    }

    /// Sets the value of PackageFullName
    pub fn set_package_full_name(&mut self, value: String) {
        self.package_full_name = Some(value);
    }

    /// Gets the value of PackageFullName
    pub fn get_package_full_name(&self) -> Option<&String> {
        self.package_full_name.as_ref()
    }

    /// Sets the value of PackageName
    pub fn set_package_name(&mut self, value: String) {
        self.package_name = Some(value);
    }

    /// Gets the value of PackageName
    pub fn get_package_name(&self) -> Option<&String> {
        self.package_name.as_ref()
    }

    /// Sets the value of PackagePublisher
    pub fn set_package_publisher(&mut self, value: String) {
        self.package_publisher = Some(value);
    }

    /// Gets the value of PackagePublisher
    pub fn get_package_publisher(&self) -> Option<&String> {
        self.package_publisher.as_ref()
    }

    /// Sets the value of PackageVersion
    pub fn set_package_version(&mut self, value: String) {
        self.package_version = Some(value);
    }

    /// Gets the value of PackageVersion
    pub fn get_package_version(&self) -> Option<&String> {
        self.package_version.as_ref()
    }

    /// Sets the value of UserSID
    pub fn set_user_sid(&mut self, value: String) {
        self.user_sid = Some(value);
    }

    /// Gets the value of UserSID
    pub fn get_user_sid(&self) -> Option<&String> {
        self.user_sid.as_ref()
    }
}

