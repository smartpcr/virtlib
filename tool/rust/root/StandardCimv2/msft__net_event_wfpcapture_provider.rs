// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventWFPCaptureProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventWFPCaptureProvider {
    #[serde(flatten)]
    pub base: MSFT_NetEventProviderBase,

/// 
    #[serde(rename = "CaptureLayerSet")]
    pub capture_layer_set: Option<u64>,

/// 
    #[serde(rename = "DiscardedEvents")]
    pub discarded_events: Option<bool>,

/// 
    #[serde(rename = "IPAddresses")]
    pub ipaddresses: Vec<String>,

/// 
    #[serde(rename = "TCPPorts")]
    pub tcpports: Vec<u16>,

/// 
    #[serde(rename = "UDPPorts")]
    pub udpports: Vec<u16>,
}

impl MSFT_NetEventWFPCaptureProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetEventProviderBase::new(),
            capture_layer_set: None,
            discarded_events: None,
            ipaddresses: Vec::new(),
            tcpports: Vec::new(),
            udpports: Vec::new(),
        }
    }


    /// Sets the value of CaptureLayerSet
    pub fn set_capture_layer_set(&mut self, value: u64) {
        self.capture_layer_set = Some(value);
    }

    /// Gets the value of CaptureLayerSet
    pub fn get_capture_layer_set(&self) -> Option<&u64> {
        self.capture_layer_set.as_ref()
    }

    /// Sets the value of DiscardedEvents
    pub fn set_discarded_events(&mut self, value: bool) {
        self.discarded_events = Some(value);
    }

    /// Gets the value of DiscardedEvents
    pub fn get_discarded_events(&self) -> Option<&bool> {
        self.discarded_events.as_ref()
    }

    /// Sets the value of IPAddresses
    pub fn set_ipaddresses(&mut self, value: Vec<String>) {
        self.ipaddresses = value;
    }

    /// Gets the value of IPAddresses
    pub fn get_ipaddresses(&self) -> &Vec<String> {
        &self.ipaddresses
    }

    /// Sets the value of TCPPorts
    pub fn set_tcpports(&mut self, value: Vec<u16>) {
        self.tcpports = value;
    }

    /// Gets the value of TCPPorts
    pub fn get_tcpports(&self) -> &Vec<u16> {
        &self.tcpports
    }

    /// Sets the value of UDPPorts
    pub fn set_udpports(&mut self, value: Vec<u16>) {
        self.udpports = value;
    }

    /// Gets the value of UDPPorts
    pub fn get_udpports(&self) -> &Vec<u16> {
        &self.udpports
    }
}

