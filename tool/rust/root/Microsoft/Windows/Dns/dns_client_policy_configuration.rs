// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Dns
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DnsClientPolicyConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DnsClientPolicyConfiguration {

/// 
    #[serde(rename = "DirectAccessDnsServers")]
    pub direct_access_dns_servers: Vec<String>,

/// 
    #[serde(rename = "DirectAccessEnabled")]
    pub direct_access_enabled: Option<bool>,

/// 
    #[serde(rename = "DirectAccessIPsecCARestriction")]
    pub direct_access_ipsec_carestriction: Option<String>,

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
    #[serde(rename = "DnsSecIPsecCARestriction")]
    pub dns_sec_ipsec_carestriction: Option<String>,

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
    #[serde(rename = "NameEncoding")]
    pub name_encoding: Option<String>,

/// 
    #[serde(rename = "NameServers")]
    pub name_servers: Vec<String>,

/// 
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// 
    #[serde(rename = "QueryPolicy")]
    pub query_policy: Option<String>,

/// 
    #[serde(rename = "SecureNameQueryFallback")]
    pub secure_name_query_fallback: Option<String>,
}

impl DnsClientPolicyConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            direct_access_dns_servers: Vec::new(),
            direct_access_enabled: None,
            direct_access_ipsec_carestriction: None,
            direct_access_proxy_name: None,
            direct_access_proxy_type: None,
            direct_access_query_ipsec_encryption: None,
            direct_access_query_ipsec_required: None,
            dns_sec_ipsec_carestriction: None,
            dns_sec_query_ipsec_encryption: None,
            dns_sec_query_ipsec_required: None,
            dns_sec_validation_required: None,
            name_encoding: None,
            name_servers: Vec::new(),
            namespace: None,
            query_policy: None,
            secure_name_query_fallback: None,
        }
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

    /// Sets the value of DirectAccessIPsecCARestriction
    pub fn set_direct_access_ipsec_carestriction(&mut self, value: String) {
        self.direct_access_ipsec_carestriction = Some(value);
    }

    /// Gets the value of DirectAccessIPsecCARestriction
    pub fn get_direct_access_ipsec_carestriction(&self) -> Option<&String> {
        self.direct_access_ipsec_carestriction.as_ref()
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

    /// Sets the value of DnsSecIPsecCARestriction
    pub fn set_dns_sec_ipsec_carestriction(&mut self, value: String) {
        self.dns_sec_ipsec_carestriction = Some(value);
    }

    /// Gets the value of DnsSecIPsecCARestriction
    pub fn get_dns_sec_ipsec_carestriction(&self) -> Option<&String> {
        self.dns_sec_ipsec_carestriction.as_ref()
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
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of QueryPolicy
    pub fn set_query_policy(&mut self, value: String) {
        self.query_policy = Some(value);
    }

    /// Gets the value of QueryPolicy
    pub fn get_query_policy(&self) -> Option<&String> {
        self.query_policy.as_ref()
    }

    /// Sets the value of SecureNameQueryFallback
    pub fn set_secure_name_query_fallback(&mut self, value: String) {
        self.secure_name_query_fallback = Some(value);
    }

    /// Gets the value of SecureNameQueryFallback
    pub fn get_secure_name_query_fallback(&self) -> Option<&String> {
        self.secure_name_query_fallback.as_ref()
    }
}

