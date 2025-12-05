// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_ConnectionInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_ConnectionInformation {

/// 
    #[serde(rename = "CID")]
    pub cid: Vec<u8>,

/// 
    #[serde(rename = "ConnectionID")]
    pub connection_id: Option<String>,

/// 
    #[serde(rename = "InitiatorAddress")]
    pub initiator_address: Option<String>,

/// 
    #[serde(rename = "InitiatorPort")]
    pub initiator_port: Option<u16>,

/// 
    #[serde(rename = "TargetAddress")]
    pub target_address: Option<String>,

/// 
    #[serde(rename = "TargetPort")]
    pub target_port: Option<u16>,
}

impl MSiSCSIInitiator_ConnectionInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cid: Vec::new(),
            connection_id: None,
            initiator_address: None,
            initiator_port: None,
            target_address: None,
            target_port: None,
        }
    }


    /// Sets the value of CID
    pub fn set_cid(&mut self, value: Vec<u8>) {
        self.cid = value;
    }

    /// Gets the value of CID
    pub fn get_cid(&self) -> &Vec<u8> {
        &self.cid
    }

    /// Sets the value of ConnectionID
    pub fn set_connection_id(&mut self, value: String) {
        self.connection_id = Some(value);
    }

    /// Gets the value of ConnectionID
    pub fn get_connection_id(&self) -> Option<&String> {
        self.connection_id.as_ref()
    }

    /// Sets the value of InitiatorAddress
    pub fn set_initiator_address(&mut self, value: String) {
        self.initiator_address = Some(value);
    }

    /// Gets the value of InitiatorAddress
    pub fn get_initiator_address(&self) -> Option<&String> {
        self.initiator_address.as_ref()
    }

    /// Sets the value of InitiatorPort
    pub fn set_initiator_port(&mut self, value: u16) {
        self.initiator_port = Some(value);
    }

    /// Gets the value of InitiatorPort
    pub fn get_initiator_port(&self) -> Option<&u16> {
        self.initiator_port.as_ref()
    }

    /// Sets the value of TargetAddress
    pub fn set_target_address(&mut self, value: String) {
        self.target_address = Some(value);
    }

    /// Gets the value of TargetAddress
    pub fn get_target_address(&self) -> Option<&String> {
        self.target_address.as_ref()
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

