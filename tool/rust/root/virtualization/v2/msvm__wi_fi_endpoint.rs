// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_WiFiEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_WiFiEndpoint {
    #[serde(flatten)]
    pub base: CIM_WiFiEndpoint,

/// 
    #[serde(rename = "Connected")]
    pub connected: Option<bool>,
}

impl Msvm_WiFiEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_WiFiEndpoint::new(),
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

