// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Appv
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AppvClientAsset struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppvClientAsset {

/// 
    #[serde(rename = "ChannelCode")]
    pub channel_code: Option<String>,

/// 
    #[serde(rename = "CM_DSLID")]
    pub cm__dslid: Option<String>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// 
    #[serde(rename = "InstalledLocation")]
    pub installed_location: Option<String>,

/// 
    #[serde(rename = "Language")]
    pub language: Option<String>,

/// 
    #[serde(rename = "OsComponent")]
    pub os_component: Option<String>,

/// 
    #[serde(rename = "PackageId")]
    pub package_id: Option<String>,

/// 
    #[serde(rename = "PackageVersionId")]
    pub package_version_id: Option<String>,

/// 
    #[serde(rename = "ProductID")]
    pub product_id: Option<String>,

/// 
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

/// 
    #[serde(rename = "ProductVersion")]
    pub product_version: Option<String>,

/// 
    #[serde(rename = "Publisher")]
    pub publisher: Option<String>,

/// 
    #[serde(rename = "RegisteredUser")]
    pub registered_user: Option<String>,

/// 
    #[serde(rename = "ServicePack")]
    pub service_pack: Option<String>,

/// 
    #[serde(rename = "SoftwareCode")]
    pub software_code: Option<String>,

/// 
    #[serde(rename = "UpgradeCode")]
    pub upgrade_code: Option<String>,

/// 
    #[serde(rename = "VersionMajor")]
    pub version_major: Option<String>,

/// 
    #[serde(rename = "VersionMinor")]
    pub version_minor: Option<String>,
}

impl AppvClientAsset {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            channel_code: None,
            cm__dslid: None,
            install_date: None,
            installed_location: None,
            language: None,
            os_component: None,
            package_id: None,
            package_version_id: None,
            product_id: None,
            product_name: None,
            product_version: None,
            publisher: None,
            registered_user: None,
            service_pack: None,
            software_code: None,
            upgrade_code: None,
            version_major: None,
            version_minor: None,
        }
    }


    /// Sets the value of ChannelCode
    pub fn set_channel_code(&mut self, value: String) {
        self.channel_code = Some(value);
    }

    /// Gets the value of ChannelCode
    pub fn get_channel_code(&self) -> Option<&String> {
        self.channel_code.as_ref()
    }

    /// Sets the value of CM_DSLID
    pub fn set_cm__dslid(&mut self, value: String) {
        self.cm__dslid = Some(value);
    }

    /// Gets the value of CM_DSLID
    pub fn get_cm__dslid(&self) -> Option<&String> {
        self.cm__dslid.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }

    /// Sets the value of InstalledLocation
    pub fn set_installed_location(&mut self, value: String) {
        self.installed_location = Some(value);
    }

    /// Gets the value of InstalledLocation
    pub fn get_installed_location(&self) -> Option<&String> {
        self.installed_location.as_ref()
    }

    /// Sets the value of Language
    pub fn set_language(&mut self, value: String) {
        self.language = Some(value);
    }

    /// Gets the value of Language
    pub fn get_language(&self) -> Option<&String> {
        self.language.as_ref()
    }

    /// Sets the value of OsComponent
    pub fn set_os_component(&mut self, value: String) {
        self.os_component = Some(value);
    }

    /// Gets the value of OsComponent
    pub fn get_os_component(&self) -> Option<&String> {
        self.os_component.as_ref()
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

    /// Sets the value of ProductID
    pub fn set_product_id(&mut self, value: String) {
        self.product_id = Some(value);
    }

    /// Gets the value of ProductID
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }

    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
    }

    /// Sets the value of ProductVersion
    pub fn set_product_version(&mut self, value: String) {
        self.product_version = Some(value);
    }

    /// Gets the value of ProductVersion
    pub fn get_product_version(&self) -> Option<&String> {
        self.product_version.as_ref()
    }

    /// Sets the value of Publisher
    pub fn set_publisher(&mut self, value: String) {
        self.publisher = Some(value);
    }

    /// Gets the value of Publisher
    pub fn get_publisher(&self) -> Option<&String> {
        self.publisher.as_ref()
    }

    /// Sets the value of RegisteredUser
    pub fn set_registered_user(&mut self, value: String) {
        self.registered_user = Some(value);
    }

    /// Gets the value of RegisteredUser
    pub fn get_registered_user(&self) -> Option<&String> {
        self.registered_user.as_ref()
    }

    /// Sets the value of ServicePack
    pub fn set_service_pack(&mut self, value: String) {
        self.service_pack = Some(value);
    }

    /// Gets the value of ServicePack
    pub fn get_service_pack(&self) -> Option<&String> {
        self.service_pack.as_ref()
    }

    /// Sets the value of SoftwareCode
    pub fn set_software_code(&mut self, value: String) {
        self.software_code = Some(value);
    }

    /// Gets the value of SoftwareCode
    pub fn get_software_code(&self) -> Option<&String> {
        self.software_code.as_ref()
    }

    /// Sets the value of UpgradeCode
    pub fn set_upgrade_code(&mut self, value: String) {
        self.upgrade_code = Some(value);
    }

    /// Gets the value of UpgradeCode
    pub fn get_upgrade_code(&self) -> Option<&String> {
        self.upgrade_code.as_ref()
    }

    /// Sets the value of VersionMajor
    pub fn set_version_major(&mut self, value: String) {
        self.version_major = Some(value);
    }

    /// Gets the value of VersionMajor
    pub fn get_version_major(&self) -> Option<&String> {
        self.version_major.as_ref()
    }

    /// Sets the value of VersionMinor
    pub fn set_version_minor(&mut self, value: String) {
        self.version_minor = Some(value);
    }

    /// Gets the value of VersionMinor
    pub fn get_version_minor(&self) -> Option<&String> {
        self.version_minor.as_ref()
    }
}

