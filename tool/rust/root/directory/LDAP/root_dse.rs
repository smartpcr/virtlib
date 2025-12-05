// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.directory.LDAP
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RootDSE struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RootDSE {

/// 
    #[serde(rename = "configurationNamingContext")]
    pub configuration_naming_context: Option<String>,

/// 
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,

/// 
    #[serde(rename = "defaultNamingContext")]
    pub default_naming_context: Option<String>,

/// 
    #[serde(rename = "dnsHostName")]
    pub dns_host_name: Option<String>,

/// 
    #[serde(rename = "dsServiceName")]
    pub ds_service_name: Option<String>,

/// 
    #[serde(rename = "highestCommittedUSN")]
    pub highest_committed_usn: Option<String>,

/// 
    #[serde(rename = "LDAPServiceName")]
    pub ldapservice_name: Option<String>,

/// 
    #[serde(rename = "namingContexts")]
    pub naming_contexts: Vec<String>,

/// 
    #[serde(rename = "rootDomainNamingContext")]
    pub root_domain_naming_context: Option<String>,

/// 
    #[serde(rename = "schemaNamingContext")]
    pub schema_naming_context: Option<String>,

/// 
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,

/// 
    #[serde(rename = "subschemaSubentry")]
    pub subschema_subentry: Option<String>,

/// 
    #[serde(rename = "supportedCapabilities")]
    pub supported_capabilities: Option<String>,

/// 
    #[serde(rename = "supportedControl")]
    pub supported_control: Vec<String>,

/// 
    #[serde(rename = "supportedLDAPPolicies")]
    pub supported_ldappolicies: Vec<String>,

/// 
    #[serde(rename = "supportedLDAPVersion")]
    pub supported_ldapversion: Vec<String>,

/// 
    #[serde(rename = "supportedSASLMechanisms")]
    pub supported_saslmechanisms: Vec<String>,
}

impl RootDSE {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            configuration_naming_context: None,
            current_time: None,
            default_naming_context: None,
            dns_host_name: None,
            ds_service_name: None,
            highest_committed_usn: None,
            ldapservice_name: None,
            naming_contexts: Vec::new(),
            root_domain_naming_context: None,
            schema_naming_context: None,
            server_name: None,
            subschema_subentry: None,
            supported_capabilities: None,
            supported_control: Vec::new(),
            supported_ldappolicies: Vec::new(),
            supported_ldapversion: Vec::new(),
            supported_saslmechanisms: Vec::new(),
        }
    }


    /// Sets the value of configurationNamingContext
    pub fn set_configuration_naming_context(&mut self, value: String) {
        self.configuration_naming_context = Some(value);
    }

    /// Gets the value of configurationNamingContext
    pub fn get_configuration_naming_context(&self) -> Option<&String> {
        self.configuration_naming_context.as_ref()
    }

    /// Sets the value of currentTime
    pub fn set_current_time(&mut self, value: String) {
        self.current_time = Some(value);
    }

    /// Gets the value of currentTime
    pub fn get_current_time(&self) -> Option<&String> {
        self.current_time.as_ref()
    }

    /// Sets the value of defaultNamingContext
    pub fn set_default_naming_context(&mut self, value: String) {
        self.default_naming_context = Some(value);
    }

    /// Gets the value of defaultNamingContext
    pub fn get_default_naming_context(&self) -> Option<&String> {
        self.default_naming_context.as_ref()
    }

    /// Sets the value of dnsHostName
    pub fn set_dns_host_name(&mut self, value: String) {
        self.dns_host_name = Some(value);
    }

    /// Gets the value of dnsHostName
    pub fn get_dns_host_name(&self) -> Option<&String> {
        self.dns_host_name.as_ref()
    }

    /// Sets the value of dsServiceName
    pub fn set_ds_service_name(&mut self, value: String) {
        self.ds_service_name = Some(value);
    }

    /// Gets the value of dsServiceName
    pub fn get_ds_service_name(&self) -> Option<&String> {
        self.ds_service_name.as_ref()
    }

    /// Sets the value of highestCommittedUSN
    pub fn set_highest_committed_usn(&mut self, value: String) {
        self.highest_committed_usn = Some(value);
    }

    /// Gets the value of highestCommittedUSN
    pub fn get_highest_committed_usn(&self) -> Option<&String> {
        self.highest_committed_usn.as_ref()
    }

    /// Sets the value of LDAPServiceName
    pub fn set_ldapservice_name(&mut self, value: String) {
        self.ldapservice_name = Some(value);
    }

    /// Gets the value of LDAPServiceName
    pub fn get_ldapservice_name(&self) -> Option<&String> {
        self.ldapservice_name.as_ref()
    }

    /// Sets the value of namingContexts
    pub fn set_naming_contexts(&mut self, value: Vec<String>) {
        self.naming_contexts = value;
    }

    /// Gets the value of namingContexts
    pub fn get_naming_contexts(&self) -> &Vec<String> {
        &self.naming_contexts
    }

    /// Sets the value of rootDomainNamingContext
    pub fn set_root_domain_naming_context(&mut self, value: String) {
        self.root_domain_naming_context = Some(value);
    }

    /// Gets the value of rootDomainNamingContext
    pub fn get_root_domain_naming_context(&self) -> Option<&String> {
        self.root_domain_naming_context.as_ref()
    }

    /// Sets the value of schemaNamingContext
    pub fn set_schema_naming_context(&mut self, value: String) {
        self.schema_naming_context = Some(value);
    }

    /// Gets the value of schemaNamingContext
    pub fn get_schema_naming_context(&self) -> Option<&String> {
        self.schema_naming_context.as_ref()
    }

    /// Sets the value of serverName
    pub fn set_server_name(&mut self, value: String) {
        self.server_name = Some(value);
    }

    /// Gets the value of serverName
    pub fn get_server_name(&self) -> Option<&String> {
        self.server_name.as_ref()
    }

    /// Sets the value of subschemaSubentry
    pub fn set_subschema_subentry(&mut self, value: String) {
        self.subschema_subentry = Some(value);
    }

    /// Gets the value of subschemaSubentry
    pub fn get_subschema_subentry(&self) -> Option<&String> {
        self.subschema_subentry.as_ref()
    }

    /// Sets the value of supportedCapabilities
    pub fn set_supported_capabilities(&mut self, value: String) {
        self.supported_capabilities = Some(value);
    }

    /// Gets the value of supportedCapabilities
    pub fn get_supported_capabilities(&self) -> Option<&String> {
        self.supported_capabilities.as_ref()
    }

    /// Sets the value of supportedControl
    pub fn set_supported_control(&mut self, value: Vec<String>) {
        self.supported_control = value;
    }

    /// Gets the value of supportedControl
    pub fn get_supported_control(&self) -> &Vec<String> {
        &self.supported_control
    }

    /// Sets the value of supportedLDAPPolicies
    pub fn set_supported_ldappolicies(&mut self, value: Vec<String>) {
        self.supported_ldappolicies = value;
    }

    /// Gets the value of supportedLDAPPolicies
    pub fn get_supported_ldappolicies(&self) -> &Vec<String> {
        &self.supported_ldappolicies
    }

    /// Sets the value of supportedLDAPVersion
    pub fn set_supported_ldapversion(&mut self, value: Vec<String>) {
        self.supported_ldapversion = value;
    }

    /// Gets the value of supportedLDAPVersion
    pub fn get_supported_ldapversion(&self) -> &Vec<String> {
        &self.supported_ldapversion
    }

    /// Sets the value of supportedSASLMechanisms
    pub fn set_supported_saslmechanisms(&mut self, value: Vec<String>) {
        self.supported_saslmechanisms = value;
    }

    /// Gets the value of supportedSASLMechanisms
    pub fn get_supported_saslmechanisms(&self) -> &Vec<String> {
        &self.supported_saslmechanisms
    }
}

