// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Firewall_App04 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Firewall_App04 {

/// 
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,

/// 
    #[serde(rename = "Fqbn")]
    pub fqbn: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "PackageFamilyName")]
    pub package_family_name: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,
}

impl MDM_Firewall_App04 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            file_path: None,
            fqbn: None,
            instance_id: None,
            package_family_name: None,
            parent_id: None,
            service_name: None,
        }
    }


    /// Sets the value of FilePath
    pub fn set_file_path(&mut self, value: String) {
        self.file_path = Some(value);
    }

    /// Gets the value of FilePath
    pub fn get_file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }

    /// Sets the value of Fqbn
    pub fn set_fqbn(&mut self, value: String) {
        self.fqbn = Some(value);
    }

    /// Gets the value of Fqbn
    pub fn get_fqbn(&self) -> Option<&String> {
        self.fqbn.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of PackageFamilyName
    pub fn set_package_family_name(&mut self, value: String) {
        self.package_family_name = Some(value);
    }

    /// Gets the value of PackageFamilyName
    pub fn get_package_family_name(&self) -> Option<&String> {
        self.package_family_name.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }
}

