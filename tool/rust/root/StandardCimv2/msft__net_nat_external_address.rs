// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNatExternalAddress struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNatExternalAddress {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<u8>,

/// 
    #[serde(rename = "ExternalAddressID")]
    pub external_address_id: Option<u32>,

/// 
    #[serde(rename = "IPAddress")]
    pub ipaddress: Option<String>,

/// 
    #[serde(rename = "NatName")]
    pub nat_name: Option<String>,

/// 
    #[serde(rename = "PortEnd")]
    pub port_end: Option<u16>,

/// 
    #[serde(rename = "PortStart")]
    pub port_start: Option<u16>,
}

impl MSFT_NetNatExternalAddress {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            active: None,
            external_address_id: None,
            ipaddress: None,
            nat_name: None,
            port_end: None,
            port_start: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: u8) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&u8> {
        self.active.as_ref()
    }

    /// Sets the value of ExternalAddressID
    pub fn set_external_address_id(&mut self, value: u32) {
        self.external_address_id = Some(value);
    }

    /// Gets the value of ExternalAddressID
    pub fn get_external_address_id(&self) -> Option<&u32> {
        self.external_address_id.as_ref()
    }

    /// Sets the value of IPAddress
    pub fn set_ipaddress(&mut self, value: String) {
        self.ipaddress = Some(value);
    }

    /// Gets the value of IPAddress
    pub fn get_ipaddress(&self) -> Option<&String> {
        self.ipaddress.as_ref()
    }

    /// Sets the value of NatName
    pub fn set_nat_name(&mut self, value: String) {
        self.nat_name = Some(value);
    }

    /// Gets the value of NatName
    pub fn get_nat_name(&self) -> Option<&String> {
        self.nat_name.as_ref()
    }

    /// Sets the value of PortEnd
    pub fn set_port_end(&mut self, value: u16) {
        self.port_end = Some(value);
    }

    /// Gets the value of PortEnd
    pub fn get_port_end(&self) -> Option<&u16> {
        self.port_end.as_ref()
    }

    /// Sets the value of PortStart
    pub fn set_port_start(&mut self, value: u16) {
        self.port_start = Some(value);
    }

    /// Gets the value of PortStart
    pub fn get_port_start(&self) -> Option<&u16> {
        self.port_start.as_ref()
    }
}

