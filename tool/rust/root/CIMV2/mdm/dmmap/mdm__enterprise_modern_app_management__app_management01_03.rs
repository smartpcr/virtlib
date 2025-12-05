// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnterpriseModernAppManagement_AppManagement01_03 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnterpriseModernAppManagement_AppManagement01_03 {

/// 
    #[serde(rename = "Architecture")]
    pub architecture: Option<String>,

/// 
    #[serde(rename = "InstallDate")]
    pub install_date: Option<String>,

/// 
    #[serde(rename = "InstallLocation")]
    pub install_location: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IsBundle")]
    pub is_bundle: Option<i32>,

/// 
    #[serde(rename = "IsFramework")]
    pub is_framework: Option<i32>,

/// 
    #[serde(rename = "IsProvisioned")]
    pub is_provisioned: Option<i32>,

/// 
    #[serde(rename = "IsStub")]
    pub is_stub: Option<i32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "PackageStatus")]
    pub package_status: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Publisher")]
    pub publisher: Option<String>,

/// 
    #[serde(rename = "RequiresReinstall")]
    pub requires_reinstall: Option<i32>,

/// 
    #[serde(rename = "ResourceID")]
    pub resource_id: Option<String>,

/// 
    #[serde(rename = "Users")]
    pub users: Option<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

impl MDM_EnterpriseModernAppManagement_AppManagement01_03 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            architecture: None,
            install_date: None,
            install_location: None,
            instance_id: None,
            is_bundle: None,
            is_framework: None,
            is_provisioned: None,
            is_stub: None,
            name: None,
            package_status: None,
            parent_id: None,
            publisher: None,
            requires_reinstall: None,
            resource_id: None,
            users: None,
            version: None,
        }
    }


    /// Sets the value of Architecture
    pub fn set_architecture(&mut self, value: String) {
        self.architecture = Some(value);
    }

    /// Gets the value of Architecture
    pub fn get_architecture(&self) -> Option<&String> {
        self.architecture.as_ref()
    }

    /// Sets the value of InstallDate
    pub fn set_install_date(&mut self, value: String) {
        self.install_date = Some(value);
    }

    /// Gets the value of InstallDate
    pub fn get_install_date(&self) -> Option<&String> {
        self.install_date.as_ref()
    }

    /// Sets the value of InstallLocation
    pub fn set_install_location(&mut self, value: String) {
        self.install_location = Some(value);
    }

    /// Gets the value of InstallLocation
    pub fn get_install_location(&self) -> Option<&String> {
        self.install_location.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IsBundle
    pub fn set_is_bundle(&mut self, value: i32) {
        self.is_bundle = Some(value);
    }

    /// Gets the value of IsBundle
    pub fn get_is_bundle(&self) -> Option<&i32> {
        self.is_bundle.as_ref()
    }

    /// Sets the value of IsFramework
    pub fn set_is_framework(&mut self, value: i32) {
        self.is_framework = Some(value);
    }

    /// Gets the value of IsFramework
    pub fn get_is_framework(&self) -> Option<&i32> {
        self.is_framework.as_ref()
    }

    /// Sets the value of IsProvisioned
    pub fn set_is_provisioned(&mut self, value: i32) {
        self.is_provisioned = Some(value);
    }

    /// Gets the value of IsProvisioned
    pub fn get_is_provisioned(&self) -> Option<&i32> {
        self.is_provisioned.as_ref()
    }

    /// Sets the value of IsStub
    pub fn set_is_stub(&mut self, value: i32) {
        self.is_stub = Some(value);
    }

    /// Gets the value of IsStub
    pub fn get_is_stub(&self) -> Option<&i32> {
        self.is_stub.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of PackageStatus
    pub fn set_package_status(&mut self, value: i32) {
        self.package_status = Some(value);
    }

    /// Gets the value of PackageStatus
    pub fn get_package_status(&self) -> Option<&i32> {
        self.package_status.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Publisher
    pub fn set_publisher(&mut self, value: String) {
        self.publisher = Some(value);
    }

    /// Gets the value of Publisher
    pub fn get_publisher(&self) -> Option<&String> {
        self.publisher.as_ref()
    }

    /// Sets the value of RequiresReinstall
    pub fn set_requires_reinstall(&mut self, value: i32) {
        self.requires_reinstall = Some(value);
    }

    /// Gets the value of RequiresReinstall
    pub fn get_requires_reinstall(&self) -> Option<&i32> {
        self.requires_reinstall.as_ref()
    }

    /// Sets the value of ResourceID
    pub fn set_resource_id(&mut self, value: String) {
        self.resource_id = Some(value);
    }

    /// Gets the value of ResourceID
    pub fn get_resource_id(&self) -> Option<&String> {
        self.resource_id.as_ref()
    }

    /// Sets the value of Users
    pub fn set_users(&mut self, value: String) {
        self.users = Some(value);
    }

    /// Gets the value of Users
    pub fn get_users(&self) -> Option<&String> {
        self.users.as_ref()
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

