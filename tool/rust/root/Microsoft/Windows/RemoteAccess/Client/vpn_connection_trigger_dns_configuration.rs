// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// VpnConnectionTriggerDnsConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VpnConnectionTriggerDnsConfiguration {

/// 
    #[serde(rename = "ConnectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "DnsIPAddress")]
    pub dns_ipaddress: Vec<String>,

/// 
    #[serde(rename = "DnsSuffix")]
    pub dns_suffix: Option<String>,

/// 
    #[serde(rename = "DnsSuffixSearchList")]
    pub dns_suffix_search_list: Vec<String>,
}

impl VpnConnectionTriggerDnsConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_name: None,
            dns_ipaddress: Vec::new(),
            dns_suffix: None,
            dns_suffix_search_list: Vec::new(),
        }
    }


    /// Sets the value of ConnectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of ConnectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }

    /// Sets the value of DnsIPAddress
    pub fn set_dns_ipaddress(&mut self, value: Vec<String>) {
        self.dns_ipaddress = value;
    }

    /// Gets the value of DnsIPAddress
    pub fn get_dns_ipaddress(&self) -> &Vec<String> {
        &self.dns_ipaddress
    }

    /// Sets the value of DnsSuffix
    pub fn set_dns_suffix(&mut self, value: String) {
        self.dns_suffix = Some(value);
    }

    /// Gets the value of DnsSuffix
    pub fn get_dns_suffix(&self) -> Option<&String> {
        self.dns_suffix.as_ref()
    }

    /// Sets the value of DnsSuffixSearchList
    pub fn set_dns_suffix_search_list(&mut self, value: Vec<String>) {
        self.dns_suffix_search_list = value;
    }

    /// Gets the value of DnsSuffixSearchList
    pub fn get_dns_suffix_search_list(&self) -> &Vec<String> {
        &self.dns_suffix_search_list
    }
}

