// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetProtocolPortFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetProtocolPortFilter {
    #[serde(flatten)]
    pub base: CIM_FilterEntryBase,

/// 
    #[serde(rename = "DynamicTransport")]
    pub dynamic_transport: Option<u32>,

/// 
    #[serde(rename = "IcmpType")]
    pub icmp_type: Vec<String>,

/// 
    #[serde(rename = "LocalPort")]
    pub local_port: Vec<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

/// 
    #[serde(rename = "RemotePort")]
    pub remote_port: Vec<String>,
}

impl MSFT_NetProtocolPortFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_FilterEntryBase::new(),
            dynamic_transport: None,
            icmp_type: Vec::new(),
            local_port: Vec::new(),
            protocol: None,
            remote_port: Vec::new(),
        }
    }


    /// Sets the value of DynamicTransport
    pub fn set_dynamic_transport(&mut self, value: u32) {
        self.dynamic_transport = Some(value);
    }

    /// Gets the value of DynamicTransport
    pub fn get_dynamic_transport(&self) -> Option<&u32> {
        self.dynamic_transport.as_ref()
    }

    /// Sets the value of IcmpType
    pub fn set_icmp_type(&mut self, value: Vec<String>) {
        self.icmp_type = value;
    }

    /// Gets the value of IcmpType
    pub fn get_icmp_type(&self) -> &Vec<String> {
        &self.icmp_type
    }

    /// Sets the value of LocalPort
    pub fn set_local_port(&mut self, value: Vec<String>) {
        self.local_port = value;
    }

    /// Gets the value of LocalPort
    pub fn get_local_port(&self) -> &Vec<String> {
        &self.local_port
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: String) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&String> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemotePort
    pub fn set_remote_port(&mut self, value: Vec<String>) {
        self.remote_port = value;
    }

    /// Gets the value of RemotePort
    pub fn get_remote_port(&self) -> &Vec<String> {
        &self.remote_port
    }
}

