// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_FailoverNetworkAdapterSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_FailoverNetworkAdapterSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "DefaultGateways")]
    pub default_gateways: Vec<String>,

/// 
    #[serde(rename = "DHCPEnabled")]
    pub dhcpenabled: Option<bool>,

/// 
    #[serde(rename = "DNSServers")]
    pub dnsservers: Vec<String>,

/// 
    #[serde(rename = "IPAddresses")]
    pub ipaddresses: Vec<String>,

/// 
    #[serde(rename = "ProtocolIFType")]
    pub protocol_iftype: Option<FailoverNetworkAdapterSettingData_ProtocolIFType>,

/// 
    #[serde(rename = "Subnets")]
    pub subnets: Vec<String>,
}

impl Msvm_FailoverNetworkAdapterSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            default_gateways: Vec::new(),
            dhcpenabled: None,
            dnsservers: Vec::new(),
            ipaddresses: Vec::new(),
            protocol_iftype: None,
            subnets: Vec::new(),
        }
    }


    /// Sets the value of DefaultGateways
    pub fn set_default_gateways(&mut self, value: Vec<String>) {
        self.default_gateways = value;
    }

    /// Gets the value of DefaultGateways
    pub fn get_default_gateways(&self) -> &Vec<String> {
        &self.default_gateways
    }

    /// Sets the value of DHCPEnabled
    pub fn set_dhcpenabled(&mut self, value: bool) {
        self.dhcpenabled = Some(value);
    }

    /// Gets the value of DHCPEnabled
    pub fn get_dhcpenabled(&self) -> Option<&bool> {
        self.dhcpenabled.as_ref()
    }

    /// Sets the value of DNSServers
    pub fn set_dnsservers(&mut self, value: Vec<String>) {
        self.dnsservers = value;
    }

    /// Gets the value of DNSServers
    pub fn get_dnsservers(&self) -> &Vec<String> {
        &self.dnsservers
    }

    /// Sets the value of IPAddresses
    pub fn set_ipaddresses(&mut self, value: Vec<String>) {
        self.ipaddresses = value;
    }

    /// Gets the value of IPAddresses
    pub fn get_ipaddresses(&self) -> &Vec<String> {
        &self.ipaddresses
    }

    /// Sets the value of ProtocolIFType
    pub fn set_protocol_iftype(&mut self, value: FailoverNetworkAdapterSettingData_ProtocolIFType) {
        self.protocol_iftype = Some(value);
    }

    /// Gets the value of ProtocolIFType
    pub fn get_protocol_iftype(&self) -> Option<&FailoverNetworkAdapterSettingData_ProtocolIFType> {
        self.protocol_iftype.as_ref()
    }

    /// Sets the value of Subnets
    pub fn set_subnets(&mut self, value: Vec<String>) {
        self.subnets = value;
    }

    /// Gets the value of Subnets
    pub fn get_subnets(&self) -> &Vec<String> {
        &self.subnets
    }
}

