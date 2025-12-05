// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_ApplicationControl struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_ApplicationControl {

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "TenantID")]
    pub tenant_id: Option<String>,
}

impl MDM_ApplicationControl {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_id: None,
            instance_id: None,
            parent_id: None,
            tenant_id: None,
        }
    }


    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
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

    /// Sets the value of TenantID
    pub fn set_tenant_id(&mut self, value: String) {
        self.tenant_id = Some(value);
    }

    /// Gets the value of TenantID
    pub fn get_tenant_id(&self) -> Option<&String> {
        self.tenant_id.as_ref()
    }
}

