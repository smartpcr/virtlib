// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_iSCSIConnection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_iSCSIConnection {

/// 
    #[serde(rename = "ConnectionIdentifier")]
    pub connection_identifier: Option<String>,

/// 
    #[serde(rename = "InitiatorAddress")]
    pub initiator_address: Option<String>,

/// 
    #[serde(rename = "InitiatorPortNumber")]
    pub initiator_port_number: Option<u32>,

/// 
    #[serde(rename = "TargetAddress")]
    pub target_address: Option<String>,

/// 
    #[serde(rename = "TargetPortNumber")]
    pub target_port_number: Option<u32>,
}

impl MSFT_iSCSIConnection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_identifier: None,
            initiator_address: None,
            initiator_port_number: None,
            target_address: None,
            target_port_number: None,
        }
    }


    /// Sets the value of ConnectionIdentifier
    pub fn set_connection_identifier(&mut self, value: String) {
        self.connection_identifier = Some(value);
    }

    /// Gets the value of ConnectionIdentifier
    pub fn get_connection_identifier(&self) -> Option<&String> {
        self.connection_identifier.as_ref()
    }

    /// Sets the value of InitiatorAddress
    pub fn set_initiator_address(&mut self, value: String) {
        self.initiator_address = Some(value);
    }

    /// Gets the value of InitiatorAddress
    pub fn get_initiator_address(&self) -> Option<&String> {
        self.initiator_address.as_ref()
    }

    /// Sets the value of InitiatorPortNumber
    pub fn set_initiator_port_number(&mut self, value: u32) {
        self.initiator_port_number = Some(value);
    }

    /// Gets the value of InitiatorPortNumber
    pub fn get_initiator_port_number(&self) -> Option<&u32> {
        self.initiator_port_number.as_ref()
    }

    /// Sets the value of TargetAddress
    pub fn set_target_address(&mut self, value: String) {
        self.target_address = Some(value);
    }

    /// Gets the value of TargetAddress
    pub fn get_target_address(&self) -> Option<&String> {
        self.target_address.as_ref()
    }

    /// Sets the value of TargetPortNumber
    pub fn set_target_port_number(&mut self, value: u32) {
        self.target_port_number = Some(value);
    }

    /// Gets the value of TargetPortNumber
    pub fn get_target_port_number(&self) -> Option<&u32> {
        self.target_port_number.as_ref()
    }
}

