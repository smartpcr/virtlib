// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnectionTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnectionTrigger {

/// 
    #[serde(rename = "ApplicationID")]
    pub application_id: Vec<String>,

/// 
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "dnsConfig")]
    pub dns_config: Vec<VpnConnectionTriggerDnsConfiguration>,

/// 
    #[serde(rename = "DnsSuffixSearchList")]
    pub dns_suffix_search_list: Vec<String>,

/// 
    #[serde(rename = "TrustedNetwork")]
    pub trusted_network: Vec<String>,
}

impl VpnConnectionTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            application_id: Vec::new(),
            connection_name: None,
            dns_config: Vec::new(),
            dns_suffix_search_list: Vec::new(),
            trusted_network: Vec::new(),
        }
    }


    /// Sets the value of ApplicationID
    pub fn set_application_id(&mut self, value: Vec<String>) {
        self.application_id = value;
    }

    /// Gets the value of ApplicationID
    pub fn get_application_id(&self) -> &Vec<String> {
        &self.application_id
    }

    /// Sets the value of ConnectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of ConnectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }

    /// Sets the value of dnsConfig
    pub fn set_dns_config(&mut self, value: Vec<VpnConnectionTriggerDnsConfiguration>) {
        self.dns_config = value;
    }

    /// Gets the value of dnsConfig
    pub fn get_dns_config(&self) -> &Vec<VpnConnectionTriggerDnsConfiguration> {
        &self.dns_config
    }

    /// Sets the value of DnsSuffixSearchList
    pub fn set_dns_suffix_search_list(&mut self, value: Vec<String>) {
        self.dns_suffix_search_list = value;
    }

    /// Gets the value of DnsSuffixSearchList
    pub fn get_dns_suffix_search_list(&self) -> &Vec<String> {
        &self.dns_suffix_search_list
    }

    /// Sets the value of TrustedNetwork
    pub fn set_trusted_network(&mut self, value: Vec<String>) {
        self.trusted_network = value;
    }

    /// Gets the value of TrustedNetwork
    pub fn get_trusted_network(&self) -> &Vec<String> {
        &self.trusted_network
    }
}

