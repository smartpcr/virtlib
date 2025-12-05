// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFC_NPIVLUNMappingInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFC_NPIVLUNMappingInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "OSBus")]
    pub osbus: Option<u8>,

/// 
    #[serde(rename = "OSLUN")]
    pub oslun: Option<u8>,

/// 
    #[serde(rename = "OSTarget")]
    pub ostarget: Option<u8>,

/// 
    #[serde(rename = "WWPNPhysicalPort")]
    pub wwpnphysical_port: Vec<u8>,

/// 
    #[serde(rename = "WWPNVirtualPort")]
    pub wwpnvirtual_port: Vec<u8>,
}

impl MSFC_NPIVLUNMappingInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            osbus: None,
            oslun: None,
            ostarget: None,
            wwpnphysical_port: Vec::new(),
            wwpnvirtual_port: Vec::new(),
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of OSBus
    pub fn set_osbus(&mut self, value: u8) {
        self.osbus = Some(value);
    }

    /// Gets the value of OSBus
    pub fn get_osbus(&self) -> Option<&u8> {
        self.osbus.as_ref()
    }

    /// Sets the value of OSLUN
    pub fn set_oslun(&mut self, value: u8) {
        self.oslun = Some(value);
    }

    /// Gets the value of OSLUN
    pub fn get_oslun(&self) -> Option<&u8> {
        self.oslun.as_ref()
    }

    /// Sets the value of OSTarget
    pub fn set_ostarget(&mut self, value: u8) {
        self.ostarget = Some(value);
    }

    /// Gets the value of OSTarget
    pub fn get_ostarget(&self) -> Option<&u8> {
        self.ostarget.as_ref()
    }

    /// Sets the value of WWPNPhysicalPort
    pub fn set_wwpnphysical_port(&mut self, value: Vec<u8>) {
        self.wwpnphysical_port = value;
    }

    /// Gets the value of WWPNPhysicalPort
    pub fn get_wwpnphysical_port(&self) -> &Vec<u8> {
        &self.wwpnphysical_port
    }

    /// Sets the value of WWPNVirtualPort
    pub fn set_wwpnvirtual_port(&mut self, value: Vec<u8>) {
        self.wwpnvirtual_port = value;
    }

    /// Gets the value of WWPNVirtualPort
    pub fn get_wwpnvirtual_port(&self) -> &Vec<u8> {
        &self.wwpnvirtual_port
    }
}

