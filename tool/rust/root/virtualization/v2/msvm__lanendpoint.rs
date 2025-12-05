// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_LANEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_LANEndpoint {
    #[serde(flatten)]
    pub base: CIM_LANEndpoint,

/// 
    #[serde(rename = "Connected")]
    pub connected: Option<bool>,
}

impl Msvm_LANEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LANEndpoint::new(),
            connected: None,
        }
    }


    /// Sets the value of Connected
    pub fn set_connected(&mut self, value: bool) {
        self.connected = Some(value);
    }

    /// Gets the value of Connected
    pub fn get_connected(&self) -> Option<&bool> {
        self.connected.as_ref()
    }
}

impl Msvm_LANEndpoint {
    /// Gets the related Msvm_LANEndpoint object(s)
    pub fn get_related__lanendpoint(&self) -> Result<Msvm_LANEndpoint, WmiError> {
        self.get_related("Msvm_LANEndpoint")
    }

    /// Gets the related Msvm_InternalEthernetPort object(s)
    pub fn get_related__internal_ethernet_port(&self) -> Result<Msvm_InternalEthernetPort, WmiError> {
        self.get_related("Msvm_InternalEthernetPort")
    }

}

