// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DnsClientNrptRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DnsClientNrptRule {

/// 
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

/// 
    #[serde(rename = "DirectAccessDnsServers")]
    pub direct_access_dns_servers: Vec<String>,

/// 
    #[serde(rename = "DirectAccessEnabled")]
    pub direct_access_enabled: Option<bool>,

/// 
    #[serde(rename = "DirectAccessProxyName")]
    pub direct_access_proxy_name: Option<String>,

/// 
    #[serde(rename = "DirectAccessProxyType")]
    pub direct_access_proxy_type: Option<String>,

/// 
    #[serde(rename = "DirectAccessQueryIPsecEncryption")]
    pub direct_access_query_ipsec_encryption: Option<String>,

/// 
    #[serde(rename = "DirectAccessQueryIPsecRequired")]
    pub direct_access_query_ipsec_required: Option<bool>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "DnsSecEnabled")]
    pub dns_sec_enabled: Option<bool>,

/// 
    #[serde(rename = "DnsSecQueryIPsecEncryption")]
    pub dns_sec_query_ipsec_encryption: Option<String>,

/// 
    #[serde(rename = "DnsSecQueryIPsecRequired")]
    pub dns_sec_query_ipsec_required: Option<bool>,

/// 
    #[serde(rename = "DnsSecValidationRequired")]
    pub dns_sec_validation_required: Option<bool>,

/// 
    #[serde(rename = "IPsecCARestriction")]
    pub ipsec_carestriction: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NameEncoding")]
    pub name_encoding: Option<String>,

/// 
    #[serde(rename = "NameServers")]
    pub name_servers: Vec<String>,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Vec<String>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<u32>,
}

impl DnsClientNrptRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            comment: None,
            direct_access_dns_servers: Vec::new(),
            direct_access_enabled: None,
            direct_access_proxy_name: None,
            direct_access_proxy_type: None,
            direct_access_query_ipsec_encryption: None,
            direct_access_query_ipsec_required: None,
            display_name: None,
            dns_sec_enabled: None,
            dns_sec_query_ipsec_encryption: None,
            dns_sec_query_ipsec_required: None,
            dns_sec_validation_required: None,
            ipsec_carestriction: None,
            name: None,
            name_encoding: None,
            name_servers: Vec::new(),
            namespace: Vec::new(),
            version: None,
        }
    }


    /// Sets the value of Comment
    pub fn set_comment(&mut self, value: String) {
        self.comment = Some(value);
    }

    /// Gets the value of Comment
    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Sets the value of DirectAccessDnsServers
    pub fn set_direct_access_dns_servers(&mut self, value: Vec<String>) {
        self.direct_access_dns_servers = value;
    }

    /// Gets the value of DirectAccessDnsServers
    pub fn get_direct_access_dns_servers(&self) -> &Vec<String> {
        &self.direct_access_dns_servers
    }

    /// Sets the value of DirectAccessEnabled
    pub fn set_direct_access_enabled(&mut self, value: bool) {
        self.direct_access_enabled = Some(value);
    }

    /// Gets the value of DirectAccessEnabled
    pub fn get_direct_access_enabled(&self) -> Option<&bool> {
        self.direct_access_enabled.as_ref()
    }

    /// Sets the value of DirectAccessProxyName
    pub fn set_direct_access_proxy_name(&mut self, value: String) {
        self.direct_access_proxy_name = Some(value);
    }

    /// Gets the value of DirectAccessProxyName
    pub fn get_direct_access_proxy_name(&self) -> Option<&String> {
        self.direct_access_proxy_name.as_ref()
    }

    /// Sets the value of DirectAccessProxyType
    pub fn set_direct_access_proxy_type(&mut self, value: String) {
        self.direct_access_proxy_type = Some(value);
    }

    /// Gets the value of DirectAccessProxyType
    pub fn get_direct_access_proxy_type(&self) -> Option<&String> {
        self.direct_access_proxy_type.as_ref()
    }

    /// Sets the value of DirectAccessQueryIPsecEncryption
    pub fn set_direct_access_query_ipsec_encryption(&mut self, value: String) {
        self.direct_access_query_ipsec_encryption = Some(value);
    }

    /// Gets the value of DirectAccessQueryIPsecEncryption
    pub fn get_direct_access_query_ipsec_encryption(&self) -> Option<&String> {
        self.direct_access_query_ipsec_encryption.as_ref()
    }

    /// Sets the value of DirectAccessQueryIPsecRequired
    pub fn set_direct_access_query_ipsec_required(&mut self, value: bool) {
        self.direct_access_query_ipsec_required = Some(value);
    }

    /// Gets the value of DirectAccessQueryIPsecRequired
    pub fn get_direct_access_query_ipsec_required(&self) -> Option<&bool> {
        self.direct_access_query_ipsec_required.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of DnsSecEnabled
    pub fn set_dns_sec_enabled(&mut self, value: bool) {
        self.dns_sec_enabled = Some(value);
    }

    /// Gets the value of DnsSecEnabled
    pub fn get_dns_sec_enabled(&self) -> Option<&bool> {
        self.dns_sec_enabled.as_ref()
    }

    /// Sets the value of DnsSecQueryIPsecEncryption
    pub fn set_dns_sec_query_ipsec_encryption(&mut self, value: String) {
        self.dns_sec_query_ipsec_encryption = Some(value);
    }

    /// Gets the value of DnsSecQueryIPsecEncryption
    pub fn get_dns_sec_query_ipsec_encryption(&self) -> Option<&String> {
        self.dns_sec_query_ipsec_encryption.as_ref()
    }

    /// Sets the value of DnsSecQueryIPsecRequired
    pub fn set_dns_sec_query_ipsec_required(&mut self, value: bool) {
        self.dns_sec_query_ipsec_required = Some(value);
    }

    /// Gets the value of DnsSecQueryIPsecRequired
    pub fn get_dns_sec_query_ipsec_required(&self) -> Option<&bool> {
        self.dns_sec_query_ipsec_required.as_ref()
    }

    /// Sets the value of DnsSecValidationRequired
    pub fn set_dns_sec_validation_required(&mut self, value: bool) {
        self.dns_sec_validation_required = Some(value);
    }

    /// Gets the value of DnsSecValidationRequired
    pub fn get_dns_sec_validation_required(&self) -> Option<&bool> {
        self.dns_sec_validation_required.as_ref()
    }

    /// Sets the value of IPsecCARestriction
    pub fn set_ipsec_carestriction(&mut self, value: String) {
        self.ipsec_carestriction = Some(value);
    }

    /// Gets the value of IPsecCARestriction
    pub fn get_ipsec_carestriction(&self) -> Option<&String> {
        self.ipsec_carestriction.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NameEncoding
    pub fn set_name_encoding(&mut self, value: String) {
        self.name_encoding = Some(value);
    }

    /// Gets the value of NameEncoding
    pub fn get_name_encoding(&self) -> Option<&String> {
        self.name_encoding.as_ref()
    }

    /// Sets the value of NameServers
    pub fn set_name_servers(&mut self, value: Vec<String>) {
        self.name_servers = value;
    }

    /// Gets the value of NameServers
    pub fn get_name_servers(&self) -> &Vec<String> {
        &self.name_servers
    }

    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: Vec<String>) {
        self.namespace = value;
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> &Vec<String> {
        &self.namespace
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

