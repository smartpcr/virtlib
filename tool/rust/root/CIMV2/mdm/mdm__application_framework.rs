// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ApplicationFramework struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ApplicationFramework {

/// 
    #[serde(rename = "MinimumPackageVersion")]
    pub minimum_package_version: Option<String>,

/// 
    #[serde(rename = "PackageArchitecture")]
    pub package_architecture: Option<String>,

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

impl MDM_ApplicationFramework {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            minimum_package_version: None,
            package_architecture: None,
            package_full_name: None,
            package_name: None,
            package_publisher: None,
            package_version: None,
            user_sid: None,
        }
    }


    /// Sets the value of MinimumPackageVersion
    pub fn set_minimum_package_version(&mut self, value: String) {
        self.minimum_package_version = Some(value);
    }

    /// Gets the value of MinimumPackageVersion
    pub fn get_minimum_package_version(&self) -> Option<&String> {
        self.minimum_package_version.as_ref()
    }

    /// Sets the value of PackageArchitecture
    pub fn set_package_architecture(&mut self, value: String) {
        self.package_architecture = Some(value);
    }

    /// Gets the value of PackageArchitecture
    pub fn get_package_architecture(&self) -> Option<&String> {
        self.package_architecture.as_ref()
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

