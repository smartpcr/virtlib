// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MountedStorageImage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MountedStorageImage {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Access")]
    pub access: Option<u16>,

/// 
    #[serde(rename = "Lun")]
    pub lun: Option<u8>,

/// 
    #[serde(rename = "PathId")]
    pub path_id: Option<u8>,

/// 
    #[serde(rename = "PnpDevicePath")]
    pub pnp_device_path: Option<String>,

/// 
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u8>,

/// 
    #[serde(rename = "TargetId")]
    pub target_id: Option<u8>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u16>,
}

impl Msvm_MountedStorageImage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            access: None,
            lun: None,
            path_id: None,
            pnp_device_path: None,
            port_number: None,
            target_id: None,
            type: None,
        }
    }


    /// Sets the value of Access
    pub fn set_access(&mut self, value: u16) {
        self.access = Some(value);
    }

    /// Gets the value of Access
    pub fn get_access(&self) -> Option<&u16> {
        self.access.as_ref()
    }

    /// Sets the value of Lun
    pub fn set_lun(&mut self, value: u8) {
        self.lun = Some(value);
    }

    /// Gets the value of Lun
    pub fn get_lun(&self) -> Option<&u8> {
        self.lun.as_ref()
    }

    /// Sets the value of PathId
    pub fn set_path_id(&mut self, value: u8) {
        self.path_id = Some(value);
    }

    /// Gets the value of PathId
    pub fn get_path_id(&self) -> Option<&u8> {
        self.path_id.as_ref()
    }

    /// Sets the value of PnpDevicePath
    pub fn set_pnp_device_path(&mut self, value: String) {
        self.pnp_device_path = Some(value);
    }

    /// Gets the value of PnpDevicePath
    pub fn get_pnp_device_path(&self) -> Option<&String> {
        self.pnp_device_path.as_ref()
    }

    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u8) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u8> {
        self.port_number.as_ref()
    }

    /// Sets the value of TargetId
    pub fn set_target_id(&mut self, value: u8) {
        self.target_id = Some(value);
    }

    /// Gets the value of TargetId
    pub fn get_target_id(&self) -> Option<&u8> {
        self.target_id.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u16) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u16> {
        self.type.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn detach_virtual_hard_disk(&self) -> Result<(), WmiError> {
        self.invoke_method("DetachVirtualHardDisk", &[])

    }

}

