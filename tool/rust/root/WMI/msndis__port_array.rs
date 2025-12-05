// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PortArray struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PortArray {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "ElementSize")]
    pub element_size: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "NumberOfPorts")]
    pub number_of_ports: Option<u32>,

/// 
    #[serde(rename = "OffsetFirstPort")]
    pub offset_first_port: Option<u32>,

/// 
    #[serde(rename = "Port")]
    pub port: Vec<MSNdis_PortChar>,
}

impl MSNdis_PortArray {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            element_size: None,
            header: None,
            number_of_ports: None,
            offset_first_port: None,
            port: Vec::new(),
        }
    }


    /// Sets the value of ElementSize
    pub fn set_element_size(&mut self, value: u32) {
        self.element_size = Some(value);
    }

    /// Gets the value of ElementSize
    pub fn get_element_size(&self) -> Option<&u32> {
        self.element_size.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of NumberOfPorts
    pub fn set_number_of_ports(&mut self, value: u32) {
        self.number_of_ports = Some(value);
    }

    /// Gets the value of NumberOfPorts
    pub fn get_number_of_ports(&self) -> Option<&u32> {
        self.number_of_ports.as_ref()
    }

    /// Sets the value of OffsetFirstPort
    pub fn set_offset_first_port(&mut self, value: u32) {
        self.offset_first_port = Some(value);
    }

    /// Gets the value of OffsetFirstPort
    pub fn get_offset_first_port(&self) -> Option<&u32> {
        self.offset_first_port.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: Vec<MSNdis_PortChar>) {
        self.port = value;
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> &Vec<MSNdis_PortChar> {
        &self.port
    }
}

