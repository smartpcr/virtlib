// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DevDetail_Ext01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DevDetail_Ext01 {

/// 
    #[serde(rename = "DeviceHardwareData")]
    pub device_hardware_data: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "WLANMACAddress")]
    pub wlanmacaddress: Option<String>,
}

impl MDM_DevDetail_Ext01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_hardware_data: None,
            instance_id: None,
            parent_id: None,
            wlanmacaddress: None,
        }
    }


    /// Sets the value of DeviceHardwareData
    pub fn set_device_hardware_data(&mut self, value: String) {
        self.device_hardware_data = Some(value);
    }

    /// Gets the value of DeviceHardwareData
    pub fn get_device_hardware_data(&self) -> Option<&String> {
        self.device_hardware_data.as_ref()
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

    /// Sets the value of WLANMACAddress
    pub fn set_wlanmacaddress(&mut self, value: String) {
        self.wlanmacaddress = Some(value);
    }

    /// Gets the value of WLANMACAddress
    pub fn get_wlanmacaddress(&self) -> Option<&String> {
        self.wlanmacaddress.as_ref()
    }
}

