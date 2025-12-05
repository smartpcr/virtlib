// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceStatus {

/// 
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "SecureBootState")]
    pub secure_boot_state: Option<i32>,
}

impl MDM_DeviceStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            domain_name: None,
            instance_id: None,
            parent_id: None,
            secure_boot_state: None,
        }
    }


    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: String) {
        self.domain_name = Some(value);
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
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

    /// Sets the value of SecureBootState
    pub fn set_secure_boot_state(&mut self, value: i32) {
        self.secure_boot_state = Some(value);
    }

    /// Gets the value of SecureBootState
    pub fn get_secure_boot_state(&self) -> Option<&i32> {
        self.secure_boot_state.as_ref()
    }
}

