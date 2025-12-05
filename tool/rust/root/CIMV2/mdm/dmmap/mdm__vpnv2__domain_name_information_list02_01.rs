// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_DomainNameInformationList02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_DomainNameInformationList02_01 {

/// 
    #[serde(rename = "AutoTrigger")]
    pub auto_trigger: Option<bool>,

/// 
    #[serde(rename = "DnsServers")]
    pub dns_servers: Option<String>,

/// 
    #[serde(rename = "DomainName")]
    pub domain_name: Option<String>,

/// 
    #[serde(rename = "DomainNameType")]
    pub domain_name_type: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Persistent")]
    pub persistent: Option<bool>,

/// 
    #[serde(rename = "WebProxyServers")]
    pub web_proxy_servers: Option<String>,
}

impl MDM_VPNv2_DomainNameInformationList02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            auto_trigger: None,
            dns_servers: None,
            domain_name: None,
            domain_name_type: None,
            instance_id: None,
            parent_id: None,
            persistent: None,
            web_proxy_servers: None,
        }
    }


    /// Sets the value of AutoTrigger
    pub fn set_auto_trigger(&mut self, value: bool) {
        self.auto_trigger = Some(value);
    }

    /// Gets the value of AutoTrigger
    pub fn get_auto_trigger(&self) -> Option<&bool> {
        self.auto_trigger.as_ref()
    }

    /// Sets the value of DnsServers
    pub fn set_dns_servers(&mut self, value: String) {
        self.dns_servers = Some(value);
    }

    /// Gets the value of DnsServers
    pub fn get_dns_servers(&self) -> Option<&String> {
        self.dns_servers.as_ref()
    }

    /// Sets the value of DomainName
    pub fn set_domain_name(&mut self, value: String) {
        self.domain_name = Some(value);
    }

    /// Gets the value of DomainName
    pub fn get_domain_name(&self) -> Option<&String> {
        self.domain_name.as_ref()
    }

    /// Sets the value of DomainNameType
    pub fn set_domain_name_type(&mut self, value: String) {
        self.domain_name_type = Some(value);
    }

    /// Gets the value of DomainNameType
    pub fn get_domain_name_type(&self) -> Option<&String> {
        self.domain_name_type.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Persistent
    pub fn set_persistent(&mut self, value: bool) {
        self.persistent = Some(value);
    }

    /// Gets the value of Persistent
    pub fn get_persistent(&self) -> Option<&bool> {
        self.persistent.as_ref()
    }

    /// Sets the value of WebProxyServers
    pub fn set_web_proxy_servers(&mut self, value: String) {
        self.web_proxy_servers = Some(value);
    }

    /// Gets the value of WebProxyServers
    pub fn get_web_proxy_servers(&self) -> Option<&String> {
        self.web_proxy_servers.as_ref()
    }
}

