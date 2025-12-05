// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventVFPProvider struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventVFPProvider {
    #[serde(flatten)]
    pub base: MSFT_NetEventProviderBase,

/// 
    #[serde(rename = "DestinationIPAddresses")]
    pub destination_ipaddresses: Vec<String>,

/// 
    #[serde(rename = "DestinationMACAddresses")]
    pub destination_macaddresses: Vec<String>,

/// 
    #[serde(rename = "GREKeys")]
    pub grekeys: Vec<u32>,

/// 
    #[serde(rename = "IPProtocols")]
    pub ipprotocols: Vec<u8>,

/// 
    #[serde(rename = "PortIds")]
    pub port_ids: Vec<u32>,

/// 
    #[serde(rename = "SourceIPAddresses")]
    pub source_ipaddresses: Vec<String>,

/// 
    #[serde(rename = "SourceMACAddresses")]
    pub source_macaddresses: Vec<String>,

/// 
    #[serde(rename = "SwitchName")]
    pub switch_name: Option<String>,

/// 
    #[serde(rename = "TCPPorts")]
    pub tcpports: Vec<u16>,

/// 
    #[serde(rename = "TenantIds")]
    pub tenant_ids: Vec<u32>,

/// 
    #[serde(rename = "UDPPorts")]
    pub udpports: Vec<u16>,

/// 
    #[serde(rename = "VFPFlowDirection")]
    pub vfpflow_direction: Option<u32>,

/// 
    #[serde(rename = "VLANIds")]
    pub vlanids: Vec<u16>,
}

impl MSFT_NetEventVFPProvider {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetEventProviderBase::new(),
            destination_ipaddresses: Vec::new(),
            destination_macaddresses: Vec::new(),
            grekeys: Vec::new(),
            ipprotocols: Vec::new(),
            port_ids: Vec::new(),
            source_ipaddresses: Vec::new(),
            source_macaddresses: Vec::new(),
            switch_name: None,
            tcpports: Vec::new(),
            tenant_ids: Vec::new(),
            udpports: Vec::new(),
            vfpflow_direction: None,
            vlanids: Vec::new(),
        }
    }


    /// Sets the value of DestinationIPAddresses
    pub fn set_destination_ipaddresses(&mut self, value: Vec<String>) {
        self.destination_ipaddresses = value;
    }

    /// Gets the value of DestinationIPAddresses
    pub fn get_destination_ipaddresses(&self) -> &Vec<String> {
        &self.destination_ipaddresses
    }

    /// Sets the value of DestinationMACAddresses
    pub fn set_destination_macaddresses(&mut self, value: Vec<String>) {
        self.destination_macaddresses = value;
    }

    /// Gets the value of DestinationMACAddresses
    pub fn get_destination_macaddresses(&self) -> &Vec<String> {
        &self.destination_macaddresses
    }

    /// Sets the value of GREKeys
    pub fn set_grekeys(&mut self, value: Vec<u32>) {
        self.grekeys = value;
    }

    /// Gets the value of GREKeys
    pub fn get_grekeys(&self) -> &Vec<u32> {
        &self.grekeys
    }

    /// Sets the value of IPProtocols
    pub fn set_ipprotocols(&mut self, value: Vec<u8>) {
        self.ipprotocols = value;
    }

    /// Gets the value of IPProtocols
    pub fn get_ipprotocols(&self) -> &Vec<u8> {
        &self.ipprotocols
    }

    /// Sets the value of PortIds
    pub fn set_port_ids(&mut self, value: Vec<u32>) {
        self.port_ids = value;
    }

    /// Gets the value of PortIds
    pub fn get_port_ids(&self) -> &Vec<u32> {
        &self.port_ids
    }

    /// Sets the value of SourceIPAddresses
    pub fn set_source_ipaddresses(&mut self, value: Vec<String>) {
        self.source_ipaddresses = value;
    }

    /// Gets the value of SourceIPAddresses
    pub fn get_source_ipaddresses(&self) -> &Vec<String> {
        &self.source_ipaddresses
    }

    /// Sets the value of SourceMACAddresses
    pub fn set_source_macaddresses(&mut self, value: Vec<String>) {
        self.source_macaddresses = value;
    }

    /// Gets the value of SourceMACAddresses
    pub fn get_source_macaddresses(&self) -> &Vec<String> {
        &self.source_macaddresses
    }

    /// Sets the value of SwitchName
    pub fn set_switch_name(&mut self, value: String) {
        self.switch_name = Some(value);
    }

    /// Gets the value of SwitchName
    pub fn get_switch_name(&self) -> Option<&String> {
        self.switch_name.as_ref()
    }

    /// Sets the value of TCPPorts
    pub fn set_tcpports(&mut self, value: Vec<u16>) {
        self.tcpports = value;
    }

    /// Gets the value of TCPPorts
    pub fn get_tcpports(&self) -> &Vec<u16> {
        &self.tcpports
    }

    /// Sets the value of TenantIds
    pub fn set_tenant_ids(&mut self, value: Vec<u32>) {
        self.tenant_ids = value;
    }

    /// Gets the value of TenantIds
    pub fn get_tenant_ids(&self) -> &Vec<u32> {
        &self.tenant_ids
    }

    /// Sets the value of UDPPorts
    pub fn set_udpports(&mut self, value: Vec<u16>) {
        self.udpports = value;
    }

    /// Gets the value of UDPPorts
    pub fn get_udpports(&self) -> &Vec<u16> {
        &self.udpports
    }

    /// Sets the value of VFPFlowDirection
    pub fn set_vfpflow_direction(&mut self, value: u32) {
        self.vfpflow_direction = Some(value);
    }

    /// Gets the value of VFPFlowDirection
    pub fn get_vfpflow_direction(&self) -> Option<&u32> {
        self.vfpflow_direction.as_ref()
    }

    /// Sets the value of VLANIds
    pub fn set_vlanids(&mut self, value: Vec<u16>) {
        self.vlanids = value;
    }

    /// Gets the value of VLANIds
    pub fn get_vlanids(&self) -> &Vec<u16> {
        &self.vlanids
    }
}

