// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DeviceStatus_NetworkIdentifiers01_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DeviceStatus_NetworkIdentifiers01_01 {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IPAddressV4")]
    pub ipaddress_v4: Option<String>,

/// 
    #[serde(rename = "IPAddressV6")]
    pub ipaddress_v6: Option<String>,

/// 
    #[serde(rename = "IsConnected")]
    pub is_connected: Option<bool>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<i32>,
}

impl MDM_DeviceStatus_NetworkIdentifiers01_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            ipaddress_v4: None,
            ipaddress_v6: None,
            is_connected: None,
            parent_id: None,
            type: None,
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

    /// Sets the value of IPAddressV4
    pub fn set_ipaddress_v4(&mut self, value: String) {
        self.ipaddress_v4 = Some(value);
    }

    /// Gets the value of IPAddressV4
    pub fn get_ipaddress_v4(&self) -> Option<&String> {
        self.ipaddress_v4.as_ref()
    }

    /// Sets the value of IPAddressV6
    pub fn set_ipaddress_v6(&mut self, value: String) {
        self.ipaddress_v6 = Some(value);
    }

    /// Gets the value of IPAddressV6
    pub fn get_ipaddress_v6(&self) -> Option<&String> {
        self.ipaddress_v6.as_ref()
    }

    /// Sets the value of IsConnected
    pub fn set_is_connected(&mut self, value: bool) {
        self.is_connected = Some(value);
    }

    /// Gets the value of IsConnected
    pub fn get_is_connected(&self) -> Option<&bool> {
        self.is_connected.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: i32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&i32> {
        self.type.as_ref()
    }
}

