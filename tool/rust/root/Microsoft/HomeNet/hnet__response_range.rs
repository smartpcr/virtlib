// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_ResponseRange struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_ResponseRange {

/// 
    #[serde(rename = "EndPort")]
    pub end_port: Option<u16>,

/// 
    #[serde(rename = "IPProtocol")]
    pub ipprotocol: Option<u8>,

/// 
    #[serde(rename = "StartPort")]
    pub start_port: Option<u16>,
}

impl HNet_ResponseRange {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            end_port: None,
            ipprotocol: None,
            start_port: None,
        }
    }


    /// Sets the value of EndPort
    pub fn set_end_port(&mut self, value: u16) {
        self.end_port = Some(value);
    }

    /// Gets the value of EndPort
    pub fn get_end_port(&self) -> Option<&u16> {
        self.end_port.as_ref()
    }

    /// Sets the value of IPProtocol
    pub fn set_ipprotocol(&mut self, value: u8) {
        self.ipprotocol = Some(value);
    }

    /// Gets the value of IPProtocol
    pub fn get_ipprotocol(&self) -> Option<&u8> {
        self.ipprotocol.as_ref()
    }

    /// Sets the value of StartPort
    pub fn set_start_port(&mut self, value: u16) {
        self.start_port = Some(value);
    }

    /// Gets the value of StartPort
    pub fn get_start_port(&self) -> Option<&u16> {
        self.start_port.as_ref()
    }
}

