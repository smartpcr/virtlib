// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_NetworkIsolation02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_NetworkIsolation02 {

/// 
    #[serde(rename = "EnterpriseCloudResources")]
    pub enterprise_cloud_resources: Option<String>,

/// 
    #[serde(rename = "EnterpriseInternalProxyServers")]
    pub enterprise_internal_proxy_servers: Option<String>,

/// 
    #[serde(rename = "EnterpriseIPRange")]
    pub enterprise_iprange: Option<String>,

/// 
    #[serde(rename = "EnterpriseIPRangesAreAuthoritative")]
    pub enterprise_ipranges_are_authoritative: Option<i32>,

/// 
    #[serde(rename = "EnterpriseNetworkDomainNames")]
    pub enterprise_network_domain_names: Option<String>,

/// 
    #[serde(rename = "EnterpriseProxyServers")]
    pub enterprise_proxy_servers: Option<String>,

/// 
    #[serde(rename = "EnterpriseProxyServersAreAuthoritative")]
    pub enterprise_proxy_servers_are_authoritative: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "NeutralResources")]
    pub neutral_resources: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_NetworkIsolation02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            enterprise_cloud_resources: None,
            enterprise_internal_proxy_servers: None,
            enterprise_iprange: None,
            enterprise_ipranges_are_authoritative: None,
            enterprise_network_domain_names: None,
            enterprise_proxy_servers: None,
            enterprise_proxy_servers_are_authoritative: None,
            instance_id: None,
            neutral_resources: None,
            parent_id: None,
        }
    }


    /// Sets the value of EnterpriseCloudResources
    pub fn set_enterprise_cloud_resources(&mut self, value: String) {
        self.enterprise_cloud_resources = Some(value);
    }

    /// Gets the value of EnterpriseCloudResources
    pub fn get_enterprise_cloud_resources(&self) -> Option<&String> {
        self.enterprise_cloud_resources.as_ref()
    }

    /// Sets the value of EnterpriseInternalProxyServers
    pub fn set_enterprise_internal_proxy_servers(&mut self, value: String) {
        self.enterprise_internal_proxy_servers = Some(value);
    }

    /// Gets the value of EnterpriseInternalProxyServers
    pub fn get_enterprise_internal_proxy_servers(&self) -> Option<&String> {
        self.enterprise_internal_proxy_servers.as_ref()
    }

    /// Sets the value of EnterpriseIPRange
    pub fn set_enterprise_iprange(&mut self, value: String) {
        self.enterprise_iprange = Some(value);
    }

    /// Gets the value of EnterpriseIPRange
    pub fn get_enterprise_iprange(&self) -> Option<&String> {
        self.enterprise_iprange.as_ref()
    }

    /// Sets the value of EnterpriseIPRangesAreAuthoritative
    pub fn set_enterprise_ipranges_are_authoritative(&mut self, value: i32) {
        self.enterprise_ipranges_are_authoritative = Some(value);
    }

    /// Gets the value of EnterpriseIPRangesAreAuthoritative
    pub fn get_enterprise_ipranges_are_authoritative(&self) -> Option<&i32> {
        self.enterprise_ipranges_are_authoritative.as_ref()
    }

    /// Sets the value of EnterpriseNetworkDomainNames
    pub fn set_enterprise_network_domain_names(&mut self, value: String) {
        self.enterprise_network_domain_names = Some(value);
    }

    /// Gets the value of EnterpriseNetworkDomainNames
    pub fn get_enterprise_network_domain_names(&self) -> Option<&String> {
        self.enterprise_network_domain_names.as_ref()
    }

    /// Sets the value of EnterpriseProxyServers
    pub fn set_enterprise_proxy_servers(&mut self, value: String) {
        self.enterprise_proxy_servers = Some(value);
    }

    /// Gets the value of EnterpriseProxyServers
    pub fn get_enterprise_proxy_servers(&self) -> Option<&String> {
        self.enterprise_proxy_servers.as_ref()
    }

    /// Sets the value of EnterpriseProxyServersAreAuthoritative
    pub fn set_enterprise_proxy_servers_are_authoritative(&mut self, value: i32) {
        self.enterprise_proxy_servers_are_authoritative = Some(value);
    }

    /// Gets the value of EnterpriseProxyServersAreAuthoritative
    pub fn get_enterprise_proxy_servers_are_authoritative(&self) -> Option<&i32> {
        self.enterprise_proxy_servers_are_authoritative.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of NeutralResources
    pub fn set_neutral_resources(&mut self, value: String) {
        self.neutral_resources = Some(value);
    }

    /// Gets the value of NeutralResources
    pub fn get_neutral_resources(&self) -> Option<&String> {
        self.neutral_resources.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

