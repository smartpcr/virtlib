// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_SwitchPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_SwitchPort {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// Numeric identifier for a switch port.
    #[serde(rename = "PortNumber")]
    pub port_number: Option<u16>,
}

impl CIM_SwitchPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            port_number: None,
        }
    }


    /// Sets the value of PortNumber
    pub fn set_port_number(&mut self, value: u16) {
        self.port_number = Some(value);
    }

    /// Gets the value of PortNumber
    pub fn get_port_number(&self) -> Option<&u16> {
        self.port_number.as_ref()
    }
}

