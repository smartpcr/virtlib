// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_FcEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_FcEndpoint {
    #[serde(flatten)]
    pub base: CIM_ProtocolEndpoint,

/// 
    #[serde(rename = "Connected")]
    pub connected: Option<bool>,

/// 
    #[serde(rename = "WWNN")]
    pub wwnn: Option<String>,

/// 
    #[serde(rename = "WWPN")]
    pub wwpn: Option<String>,
}

impl Msvm_FcEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ProtocolEndpoint::new(),
            connected: None,
            wwnn: None,
            wwpn: None,
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

    /// Sets the value of WWNN
    pub fn set_wwnn(&mut self, value: String) {
        self.wwnn = Some(value);
    }

    /// Gets the value of WWNN
    pub fn get_wwnn(&self) -> Option<&String> {
        self.wwnn.as_ref()
    }

    /// Sets the value of WWPN
    pub fn set_wwpn(&mut self, value: String) {
        self.wwpn = Some(value);
    }

    /// Gets the value of WWPN
    pub fn get_wwpn(&self) -> Option<&String> {
        self.wwpn.as_ref()
    }
}

