// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_ApplicationManagement02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_ApplicationManagement02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MSIAlwaysInstallWithElevatedPrivileges")]
    pub msialways_install_with_elevated_privileges: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RequirePrivateStoreOnly")]
    pub require_private_store_only: Option<i32>,
}

impl MDM_Policy_User_Config01_ApplicationManagement02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            msialways_install_with_elevated_privileges: None,
            parent_id: None,
            require_private_store_only: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MSIAlwaysInstallWithElevatedPrivileges
    pub fn set_msialways_install_with_elevated_privileges(&mut self, value: i32) {
        self.msialways_install_with_elevated_privileges = Some(value);
    }

    /// Gets the value of MSIAlwaysInstallWithElevatedPrivileges
    pub fn get_msialways_install_with_elevated_privileges(&self) -> Option<&i32> {
        self.msialways_install_with_elevated_privileges.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RequirePrivateStoreOnly
    pub fn set_require_private_store_only(&mut self, value: i32) {
        self.require_private_store_only = Some(value);
    }

    /// Gets the value of RequirePrivateStoreOnly
    pub fn get_require_private_store_only(&self) -> Option<&i32> {
        self.require_private_store_only.as_ref()
    }
}

