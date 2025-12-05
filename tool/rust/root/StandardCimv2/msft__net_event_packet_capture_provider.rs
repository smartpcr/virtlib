// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventPacketCaptureProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventPacketCaptureProvider {
    #[serde(flatten)]
    pub base: MSFT_NetEventProviderBase,

/// 
    #[serde(rename = "CaptureType")]
    pub capture_type: Option<u8>,

/// 
    #[serde(rename = "EtherType")]
    pub ether_type: Vec<u16>,

/// 
    #[serde(rename = "IPAddresses")]
    pub ipaddresses: Vec<String>,

/// 
    #[serde(rename = "IPProtocols")]
    pub ipprotocols: Vec<u8>,

/// 
    #[serde(rename = "LinkLayerAddress")]
    pub link_layer_address: Vec<String>,

/// 
    #[serde(rename = "MultiLayer")]
    pub multi_layer: Option<bool>,

/// 
    #[serde(rename = "TruncationLength")]
    pub truncation_length: Option<u16>,

/// 
    #[serde(rename = "VmCaptureDirection")]
    pub vm_capture_direction: Option<u8>,
}

impl MSFT_NetEventPacketCaptureProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetEventProviderBase::new(),
            capture_type: None,
            ether_type: Vec::new(),
            ipaddresses: Vec::new(),
            ipprotocols: Vec::new(),
            link_layer_address: Vec::new(),
            multi_layer: None,
            truncation_length: None,
            vm_capture_direction: None,
        }
    }


    /// Sets the value of CaptureType
    pub fn set_capture_type(&mut self, value: u8) {
        self.capture_type = Some(value);
    }

    /// Gets the value of CaptureType
    pub fn get_capture_type(&self) -> Option<&u8> {
        self.capture_type.as_ref()
    }

    /// Sets the value of EtherType
    pub fn set_ether_type(&mut self, value: Vec<u16>) {
        self.ether_type = value;
    }

    /// Gets the value of EtherType
    pub fn get_ether_type(&self) -> &Vec<u16> {
        &self.ether_type
    }

    /// Sets the value of IPAddresses
    pub fn set_ipaddresses(&mut self, value: Vec<String>) {
        self.ipaddresses = value;
    }

    /// Gets the value of IPAddresses
    pub fn get_ipaddresses(&self) -> &Vec<String> {
        &self.ipaddresses
    }

    /// Sets the value of IPProtocols
    pub fn set_ipprotocols(&mut self, value: Vec<u8>) {
        self.ipprotocols = value;
    }

    /// Gets the value of IPProtocols
    pub fn get_ipprotocols(&self) -> &Vec<u8> {
        &self.ipprotocols
    }

    /// Sets the value of LinkLayerAddress
    pub fn set_link_layer_address(&mut self, value: Vec<String>) {
        self.link_layer_address = value;
    }

    /// Gets the value of LinkLayerAddress
    pub fn get_link_layer_address(&self) -> &Vec<String> {
        &self.link_layer_address
    }

    /// Sets the value of MultiLayer
    pub fn set_multi_layer(&mut self, value: bool) {
        self.multi_layer = Some(value);
    }

    /// Gets the value of MultiLayer
    pub fn get_multi_layer(&self) -> Option<&bool> {
        self.multi_layer.as_ref()
    }

    /// Sets the value of TruncationLength
    pub fn set_truncation_length(&mut self, value: u16) {
        self.truncation_length = Some(value);
    }

    /// Gets the value of TruncationLength
    pub fn get_truncation_length(&self) -> Option<&u16> {
        self.truncation_length.as_ref()
    }

    /// Sets the value of VmCaptureDirection
    pub fn set_vm_capture_direction(&mut self, value: u8) {
        self.vm_capture_direction = Some(value);
    }

    /// Gets the value of VmCaptureDirection
    pub fn get_vm_capture_direction(&self) -> Option<&u8> {
        self.vm_capture_direction.as_ref()
    }
}

