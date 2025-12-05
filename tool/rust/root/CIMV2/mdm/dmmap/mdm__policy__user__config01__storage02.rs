// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_User_Config01_Storage02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_User_Config01_Storage02 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "WPDDevicesDenyReadAccessPerUser")]
    pub wpddevices_deny_read_access_per_user: Option<String>,

/// 
    #[serde(rename = "WPDDevicesDenyWriteAccessPerUser")]
    pub wpddevices_deny_write_access_per_user: Option<String>,
}

impl MDM_Policy_User_Config01_Storage02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            parent_id: None,
            wpddevices_deny_read_access_per_user: None,
            wpddevices_deny_write_access_per_user: None,
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

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of WPDDevicesDenyReadAccessPerUser
    pub fn set_wpddevices_deny_read_access_per_user(&mut self, value: String) {
        self.wpddevices_deny_read_access_per_user = Some(value);
    }

    /// Gets the value of WPDDevicesDenyReadAccessPerUser
    pub fn get_wpddevices_deny_read_access_per_user(&self) -> Option<&String> {
        self.wpddevices_deny_read_access_per_user.as_ref()
    }

    /// Sets the value of WPDDevicesDenyWriteAccessPerUser
    pub fn set_wpddevices_deny_write_access_per_user(&mut self, value: String) {
        self.wpddevices_deny_write_access_per_user = Some(value);
    }

    /// Gets the value of WPDDevicesDenyWriteAccessPerUser
    pub fn get_wpddevices_deny_write_access_per_user(&self) -> Option<&String> {
        self.wpddevices_deny_write_access_per_user.as_ref()
    }
}

