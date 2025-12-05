// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_BackupIpConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_BackupIpConfiguration {

/// 
    #[serde(rename = "Connection")]
    pub connection: Option<HNet_Connection>,

/// 
    #[serde(rename = "DefaultGateway")]
    pub default_gateway: Option<String>,

/// 
    #[serde(rename = "EnableDHCP")]
    pub enable_dhcp: Option<u32>,

/// 
    #[serde(rename = "IPAddress")]
    pub ipaddress: Option<String>,

/// 
    #[serde(rename = "SubnetMask")]
    pub subnet_mask: Option<String>,
}

impl HNet_BackupIpConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection: None,
            default_gateway: None,
            enable_dhcp: None,
            ipaddress: None,
            subnet_mask: None,
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

    /// Sets the value of DefaultGateway
    pub fn set_default_gateway(&mut self, value: String) {
        self.default_gateway = Some(value);
    }

    /// Gets the value of DefaultGateway
    pub fn get_default_gateway(&self) -> Option<&String> {
        self.default_gateway.as_ref()
    }

    /// Sets the value of EnableDHCP
    pub fn set_enable_dhcp(&mut self, value: u32) {
        self.enable_dhcp = Some(value);
    }

    /// Gets the value of EnableDHCP
    pub fn get_enable_dhcp(&self) -> Option<&u32> {
        self.enable_dhcp.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: String) {
        self.ipaddress = Some(value);
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> Option<&String> {
        self.ipaddress.as_ref()
    }

    /// Sets the value of SubnetMask
    pub fn set_subnet_mask(&mut self, value: String) {
        self.subnet_mask = Some(value);
    }

    /// Gets the value of SubnetMask
    pub fn get_subnet_mask(&self) -> Option<&String> {
        self.subnet_mask.as_ref()
    }
}

