// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNatTransitionMonitoring struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNatTransitionMonitoring {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "InboundAddress")]
    pub inbound_address: Option<String>,

/// 
    #[serde(rename = "NatOutboundAddress")]
    pub nat_outbound_address: Option<String>,

/// 
    #[serde(rename = "OutboundAddress")]
    pub outbound_address: Option<String>,

/// 
    #[serde(rename = "TransportProtocol")]
    pub transport_protocol: Option<u32>,
}

impl MSFT_NetNatTransitionMonitoring {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            inbound_address: None,
            nat_outbound_address: None,
            outbound_address: None,
            transport_protocol: None,
        }
    }


    /// Sets the value of InboundAddress
    pub fn set_inbound_address(&mut self, value: String) {
        self.inbound_address = Some(value);
    }

    /// Gets the value of InboundAddress
    pub fn get_inbound_address(&self) -> Option<&String> {
        self.inbound_address.as_ref()
    }

    /// Sets the value of NatOutboundAddress
    pub fn set_nat_outbound_address(&mut self, value: String) {
        self.nat_outbound_address = Some(value);
    }

    /// Gets the value of NatOutboundAddress
    pub fn get_nat_outbound_address(&self) -> Option<&String> {
        self.nat_outbound_address.as_ref()
    }

    /// Sets the value of OutboundAddress
    pub fn set_outbound_address(&mut self, value: String) {
        self.outbound_address = Some(value);
    }

    /// Gets the value of OutboundAddress
    pub fn get_outbound_address(&self) -> Option<&String> {
        self.outbound_address.as_ref()
    }

    /// Sets the value of TransportProtocol
    pub fn set_transport_protocol(&mut self, value: u32) {
        self.transport_protocol = Some(value);
    }

    /// Gets the value of TransportProtocol
    pub fn get_transport_protocol(&self) -> Option<&u32> {
        self.transport_protocol.as_ref()
    }
}

