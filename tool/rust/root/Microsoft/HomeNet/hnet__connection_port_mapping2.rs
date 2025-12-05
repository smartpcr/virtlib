// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_ConnectionPortMapping2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_ConnectionPortMapping2 {

/// 
    #[serde(rename = "Connection")]
    pub connection: Option<HNet_Connection>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "NameActive")]
    pub name_active: Option<bool>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<HNet_PortMappingProtocol>,

/// 
    #[serde(rename = "TargetIPAddress")]
    pub target_ipaddress: Option<u32>,

/// 
    #[serde(rename = "TargetName")]
    pub target_name: Option<String>,

/// 
    #[serde(rename = "TargetPort")]
    pub target_port: Option<u16>,
}

impl HNet_ConnectionPortMapping2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection: None,
            enabled: None,
            name_active: None,
            protocol: None,
            target_ipaddress: None,
            target_name: None,
            target_port: None,
        }
    }


    /// Sets the value of Connection
    pub fn set_connection(&mut self, value: HNet_Connection) {
        self.connection = Some(value);
    }

    /// Gets the value of Connection
    pub fn get_connection(&self) -> Option<&HNet_Connection> {
        self.connection.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of NameActive
    pub fn set_name_active(&mut self, value: bool) {
        self.name_active = Some(value);
    }

    /// Gets the value of NameActive
    pub fn get_name_active(&self) -> Option<&bool> {
        self.name_active.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: HNet_PortMappingProtocol) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&HNet_PortMappingProtocol> {
        self.protocol.as_ref()
    }

    /// Sets the value of TargetIPAddress
    pub fn set_target_ipaddress(&mut self, value: u32) {
        self.target_ipaddress = Some(value);
    }

    /// Gets the value of TargetIPAddress
    pub fn get_target_ipaddress(&self) -> Option<&u32> {
        self.target_ipaddress.as_ref()
    }

    /// Sets the value of TargetName
    pub fn set_target_name(&mut self, value: String) {
        self.target_name = Some(value);
    }

    /// Gets the value of TargetName
    pub fn get_target_name(&self) -> Option<&String> {
        self.target_name.as_ref()
    }

    /// Sets the value of TargetPort
    pub fn set_target_port(&mut self, value: u16) {
        self.target_port = Some(value);
    }

    /// Gets the value of TargetPort
    pub fn get_target_port(&self) -> Option<&u16> {
        self.target_port.as_ref()
    }
}

