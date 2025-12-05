// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNatSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNatSession {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "CreationTime")]
    pub creation_time: Option<String>,

/// 
    #[serde(rename = "ExternalDestinationAddress")]
    pub external_destination_address: Option<String>,

/// 
    #[serde(rename = "ExternalDestinationPort")]
    pub external_destination_port: Option<u16>,

/// 
    #[serde(rename = "ExternalSourceAddress")]
    pub external_source_address: Option<String>,

/// 
    #[serde(rename = "ExternalSourcePort")]
    pub external_source_port: Option<u16>,

/// 
    #[serde(rename = "InternalDestinationAddress")]
    pub internal_destination_address: Option<String>,

/// 
    #[serde(rename = "InternalDestinationPort")]
    pub internal_destination_port: Option<u16>,

/// 
    #[serde(rename = "InternalRoutingDomainId")]
    pub internal_routing_domain_id: Option<String>,

/// 
    #[serde(rename = "InternalSourceAddress")]
    pub internal_source_address: Option<String>,

/// 
    #[serde(rename = "InternalSourcePort")]
    pub internal_source_port: Option<u16>,

/// 
    #[serde(rename = "NatName")]
    pub nat_name: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<u32>,
}

impl MSFT_NetNatSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            creation_time: None,
            external_destination_address: None,
            external_destination_port: None,
            external_source_address: None,
            external_source_port: None,
            internal_destination_address: None,
            internal_destination_port: None,
            internal_routing_domain_id: None,
            internal_source_address: None,
            internal_source_port: None,
            nat_name: None,
            protocol: None,
        }
    }


    /// Sets the value of CreationTime
    pub fn set_creation_time(&mut self, value: String) {
        self.creation_time = Some(value);
    }

    /// Gets the value of CreationTime
    pub fn get_creation_time(&self) -> Option<&String> {
        self.creation_time.as_ref()
    }

    /// Sets the value of ExternalDestinationAddress
    pub fn set_external_destination_address(&mut self, value: String) {
        self.external_destination_address = Some(value);
    }

    /// Gets the value of ExternalDestinationAddress
    pub fn get_external_destination_address(&self) -> Option<&String> {
        self.external_destination_address.as_ref()
    }

    /// Sets the value of ExternalDestinationPort
    pub fn set_external_destination_port(&mut self, value: u16) {
        self.external_destination_port = Some(value);
    }

    /// Gets the value of ExternalDestinationPort
    pub fn get_external_destination_port(&self) -> Option<&u16> {
        self.external_destination_port.as_ref()
    }

    /// Sets the value of ExternalSourceAddress
    pub fn set_external_source_address(&mut self, value: String) {
        self.external_source_address = Some(value);
    }

    /// Gets the value of ExternalSourceAddress
    pub fn get_external_source_address(&self) -> Option<&String> {
        self.external_source_address.as_ref()
    }

    /// Sets the value of ExternalSourcePort
    pub fn set_external_source_port(&mut self, value: u16) {
        self.external_source_port = Some(value);
    }

    /// Gets the value of ExternalSourcePort
    pub fn get_external_source_port(&self) -> Option<&u16> {
        self.external_source_port.as_ref()
    }

    /// Sets the value of InternalDestinationAddress
    pub fn set_internal_destination_address(&mut self, value: String) {
        self.internal_destination_address = Some(value);
    }

    /// Gets the value of InternalDestinationAddress
    pub fn get_internal_destination_address(&self) -> Option<&String> {
        self.internal_destination_address.as_ref()
    }

    /// Sets the value of InternalDestinationPort
    pub fn set_internal_destination_port(&mut self, value: u16) {
        self.internal_destination_port = Some(value);
    }

    /// Gets the value of InternalDestinationPort
    pub fn get_internal_destination_port(&self) -> Option<&u16> {
        self.internal_destination_port.as_ref()
    }

    /// Sets the value of InternalRoutingDomainId
    pub fn set_internal_routing_domain_id(&mut self, value: String) {
        self.internal_routing_domain_id = Some(value);
    }

    /// Gets the value of InternalRoutingDomainId
    pub fn get_internal_routing_domain_id(&self) -> Option<&String> {
        self.internal_routing_domain_id.as_ref()
    }

    /// Sets the value of InternalSourceAddress
    pub fn set_internal_source_address(&mut self, value: String) {
        self.internal_source_address = Some(value);
    }

    /// Gets the value of InternalSourceAddress
    pub fn get_internal_source_address(&self) -> Option<&String> {
        self.internal_source_address.as_ref()
    }

    /// Sets the value of InternalSourcePort
    pub fn set_internal_source_port(&mut self, value: u16) {
        self.internal_source_port = Some(value);
    }

    /// Gets the value of InternalSourcePort
    pub fn get_internal_source_port(&self) -> Option<&u16> {
        self.internal_source_port.as_ref()
    }

    /// Sets the value of NatName
    pub fn set_nat_name(&mut self, value: String) {
        self.nat_name = Some(value);
    }

    /// Gets the value of NatName
    pub fn get_nat_name(&self) -> Option<&String> {
        self.nat_name.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: u32) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&u32> {
        self.protocol.as_ref()
    }
}

