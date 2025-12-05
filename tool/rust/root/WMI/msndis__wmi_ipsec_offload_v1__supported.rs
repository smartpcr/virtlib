// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiIPSecOffloadV1_Supported struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiIPSecOffloadV1_Supported {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "AhEspCombined")]
    pub ah_esp_combined: Option<u32>,

/// 
    #[serde(rename = "Encapsulation")]
    pub encapsulation: Option<u32>,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "IPv4Options")]
    pub ipv4_options: Option<u32>,

/// 
    #[serde(rename = "TransportTunnelCombined")]
    pub transport_tunnel_combined: Option<u32>,
}

impl MSNdis_WmiIPSecOffloadV1_Supported {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ah_esp_combined: None,
            encapsulation: None,
            flags: None,
            ipv4_options: None,
            transport_tunnel_combined: None,
        }
    }


    /// Sets the value of AhEspCombined
    pub fn set_ah_esp_combined(&mut self, value: u32) {
        self.ah_esp_combined = Some(value);
    }

    /// Gets the value of AhEspCombined
    pub fn get_ah_esp_combined(&self) -> Option<&u32> {
        self.ah_esp_combined.as_ref()
    }

    /// Sets the value of Encapsulation
    pub fn set_encapsulation(&mut self, value: u32) {
        self.encapsulation = Some(value);
    }

    /// Gets the value of Encapsulation
    pub fn get_encapsulation(&self) -> Option<&u32> {
        self.encapsulation.as_ref()
    }

    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of IPv4Options
    pub fn set_ipv4_options(&mut self, value: u32) {
        self.ipv4_options = Some(value);
    }

    /// Gets the value of IPv4Options
    pub fn get_ipv4_options(&self) -> Option<&u32> {
        self.ipv4_options.as_ref()
    }

    /// Sets the value of TransportTunnelCombined
    pub fn set_transport_tunnel_combined(&mut self, value: u32) {
        self.transport_tunnel_combined = Some(value);
    }

    /// Gets the value of TransportTunnelCombined
    pub fn get_transport_tunnel_combined(&self) -> Option<&u32> {
        self.transport_tunnel_combined.as_ref()
    }
}

