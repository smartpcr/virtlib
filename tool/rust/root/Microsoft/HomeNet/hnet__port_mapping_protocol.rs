// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_PortMappingProtocol struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_PortMappingProtocol {

/// 
    #[serde(rename = "BuiltIn")]
    pub built_in: Option<bool>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IPProtocol")]
    pub ipprotocol: Option<u8>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Port")]
    pub port: Option<u16>,
}

impl HNet_PortMappingProtocol {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            built_in: None,
            id: None,
            ipprotocol: None,
            name: None,
            port: None,
        }
    }


    /// Sets the value of BuiltIn
    pub fn set_built_in(&mut self, value: bool) {
        self.built_in = Some(value);
    }

    /// Gets the value of BuiltIn
    pub fn get_built_in(&self) -> Option<&bool> {
        self.built_in.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IPProtocol
    pub fn set_ipprotocol(&mut self, value: u8) {
        self.ipprotocol = Some(value);
    }

    /// Gets the value of IPProtocol
    pub fn get_ipprotocol(&self) -> Option<&u8> {
        self.ipprotocol.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: u16) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&u16> {
        self.port.as_ref()
    }
}

