// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_ApplicationProtocol struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_ApplicationProtocol {

/// 
    #[serde(rename = "BuiltIn")]
    pub built_in: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "OutgoingIPProtocol")]
    pub outgoing_ipprotocol: Option<u8>,

/// 
    #[serde(rename = "OutgoingPort")]
    pub outgoing_port: Option<u16>,

/// 
    #[serde(rename = "ResponseArray")]
    pub response_array: Vec<HNet_ResponseRange>,

/// 
    #[serde(rename = "ResponseCount")]
    pub response_count: Option<u16>,
}

impl HNet_ApplicationProtocol {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            built_in: None,
            enabled: None,
            id: None,
            name: None,
            outgoing_ipprotocol: None,
            outgoing_port: None,
            response_array: Vec::new(),
            response_count: None,
        }
    }


    /// Sets the value of BuiltIn
    pub fn set_built_in(&mut self, value: bool) {
        self.built_in = Some(value);
    }

    /// Gets the value of BuiltIn
    pub fn get_built_in(&self) -> Option<&bool> {
        self.built_in.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of OutgoingIPProtocol
    pub fn set_outgoing_ipprotocol(&mut self, value: u8) {
        self.outgoing_ipprotocol = Some(value);
    }

    /// Gets the value of OutgoingIPProtocol
    pub fn get_outgoing_ipprotocol(&self) -> Option<&u8> {
        self.outgoing_ipprotocol.as_ref()
    }

    /// Sets the value of OutgoingPort
    pub fn set_outgoing_port(&mut self, value: u16) {
        self.outgoing_port = Some(value);
    }

    /// Gets the value of OutgoingPort
    pub fn get_outgoing_port(&self) -> Option<&u16> {
        self.outgoing_port.as_ref()
    }

    /// Sets the value of ResponseArray
    pub fn set_response_array(&mut self, value: Vec<HNet_ResponseRange>) {
        self.response_array = value;
    }

    /// Gets the value of ResponseArray
    pub fn get_response_array(&self) -> &Vec<HNet_ResponseRange> {
        &self.response_array
    }

    /// Sets the value of ResponseCount
    pub fn set_response_count(&mut self, value: u16) {
        self.response_count = Some(value);
    }

    /// Gets the value of ResponseCount
    pub fn get_response_count(&self) -> Option<&u16> {
        self.response_count.as_ref()
    }
}

