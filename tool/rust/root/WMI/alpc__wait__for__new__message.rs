// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ALPC_Wait_For_New_Message struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ALPC_Wait_For_New_Message {
    #[serde(flatten)]
    pub base: ALPC,

/// 
    #[serde(rename = "IsServerPort")]
    pub is_server_port: Option<u32>,

/// 
    #[serde(rename = "PortName")]
    pub port_name: Option<String>,
}

impl ALPC_Wait_For_New_Message {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ALPC::new(),
            is_server_port: None,
            port_name: None,
        }
    }


    /// Sets the value of IsServerPort
    pub fn set_is_server_port(&mut self, value: u32) {
        self.is_server_port = Some(value);
    }

    /// Gets the value of IsServerPort
    pub fn get_is_server_port(&self) -> Option<&u32> {
        self.is_server_port.as_ref()
    }

    /// Sets the value of PortName
    pub fn set_port_name(&mut self, value: String) {
        self.port_name = Some(value);
    }

    /// Gets the value of PortName
    pub fn get_port_name(&self) -> Option<&String> {
        self.port_name.as_ref()
    }
}

