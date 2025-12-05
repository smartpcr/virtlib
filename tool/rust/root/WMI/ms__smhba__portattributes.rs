// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SMHBA_PORTATTRIBUTES struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SMHBA_PORTATTRIBUTES {

/// 
    #[serde(rename = "OSDeviceName")]
    pub osdevice_name: Option<String>,

/// 
    #[serde(rename = "PortSpecificAttributes")]
    pub port_specific_attributes: Vec<u8>,

/// 
    #[serde(rename = "PortSpecificAttributesSize")]
    pub port_specific_attributes_size: Option<u32>,

/// 
    #[serde(rename = "PortState")]
    pub port_state: Option<u32>,

/// 
    #[serde(rename = "PortType")]
    pub port_type: Option<u32>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u64>,
}

impl MS_SMHBA_PORTATTRIBUTES {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            osdevice_name: None,
            port_specific_attributes: Vec::new(),
            port_specific_attributes_size: None,
            port_state: None,
            port_type: None,
            reserved: None,
        }
    }


    /// Sets the value of OSDeviceName
    pub fn set_osdevice_name(&mut self, value: String) {
        self.osdevice_name = Some(value);
    }

    /// Gets the value of OSDeviceName
    pub fn get_osdevice_name(&self) -> Option<&String> {
        self.osdevice_name.as_ref()
    }

    /// Sets the value of PortSpecificAttributes
    pub fn set_port_specific_attributes(&mut self, value: Vec<u8>) {
        self.port_specific_attributes = value;
    }

    /// Gets the value of PortSpecificAttributes
    pub fn get_port_specific_attributes(&self) -> &Vec<u8> {
        &self.port_specific_attributes
    }

    /// Sets the value of PortSpecificAttributesSize
    pub fn set_port_specific_attributes_size(&mut self, value: u32) {
        self.port_specific_attributes_size = Some(value);
    }

    /// Gets the value of PortSpecificAttributesSize
    pub fn get_port_specific_attributes_size(&self) -> Option<&u32> {
        self.port_specific_attributes_size.as_ref()
    }

    /// Sets the value of PortState
    pub fn set_port_state(&mut self, value: u32) {
        self.port_state = Some(value);
    }

    /// Gets the value of PortState
    pub fn get_port_state(&self) -> Option<&u32> {
        self.port_state.as_ref()
    }

    /// Sets the value of PortType
    pub fn set_port_type(&mut self, value: u32) {
        self.port_type = Some(value);
    }

    /// Gets the value of PortType
    pub fn get_port_type(&self) -> Option<&u32> {
        self.port_type.as_ref()
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u64) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u64> {
        self.reserved.as_ref()
    }
}

