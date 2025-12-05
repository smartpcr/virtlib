// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_MSSLegacy02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_MSSLegacy02 {

/// 
    #[serde(rename = "AllowICMPRedirectsToOverrideOSPFGeneratedRoutes")]
    pub allow_icmpredirects_to_override_ospfgenerated_routes: Option<String>,

/// 
    #[serde(rename = "AllowTheComputerToIgnoreNetBIOSNameReleaseRequestsExceptFromWINSServers")]
    pub allow_the_computer_to_ignore_net_biosname_release_requests_except_from_winsservers: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "IPSourceRoutingProtectionLevel")]
    pub ipsource_routing_protection_level: Option<String>,

/// 
    #[serde(rename = "IPv6SourceRoutingProtectionLevel")]
    pub ipv6_source_routing_protection_level: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_Policy_Result01_MSSLegacy02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_icmpredirects_to_override_ospfgenerated_routes: None,
            allow_the_computer_to_ignore_net_biosname_release_requests_except_from_winsservers: None,
            instance_id: None,
            ipsource_routing_protection_level: None,
            ipv6_source_routing_protection_level: None,
            parent_id: None,
        }
    }


    /// Sets the value of AllowICMPRedirectsToOverrideOSPFGeneratedRoutes
    pub fn set_allow_icmpredirects_to_override_ospfgenerated_routes(&mut self, value: String) {
        self.allow_icmpredirects_to_override_ospfgenerated_routes = Some(value);
    }

    /// Gets the value of AllowICMPRedirectsToOverrideOSPFGeneratedRoutes
    pub fn get_allow_icmpredirects_to_override_ospfgenerated_routes(&self) -> Option<&String> {
        self.allow_icmpredirects_to_override_ospfgenerated_routes.as_ref()
    }

    /// Sets the value of AllowTheComputerToIgnoreNetBIOSNameReleaseRequestsExceptFromWINSServers
    pub fn set_allow_the_computer_to_ignore_net_biosname_release_requests_except_from_winsservers(&mut self, value: String) {
        self.allow_the_computer_to_ignore_net_biosname_release_requests_except_from_winsservers = Some(value);
    }

    /// Gets the value of AllowTheComputerToIgnoreNetBIOSNameReleaseRequestsExceptFromWINSServers
    pub fn get_allow_the_computer_to_ignore_net_biosname_release_requests_except_from_winsservers(&self) -> Option<&String> {
        self.allow_the_computer_to_ignore_net_biosname_release_requests_except_from_winsservers.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of IPSourceRoutingProtectionLevel
    pub fn set_ipsource_routing_protection_level(&mut self, value: String) {
        self.ipsource_routing_protection_level = Some(value);
    }

    /// Gets the value of IPSourceRoutingProtectionLevel
    pub fn get_ipsource_routing_protection_level(&self) -> Option<&String> {
        self.ipsource_routing_protection_level.as_ref()
    }

    /// Sets the value of IPv6SourceRoutingProtectionLevel
    pub fn set_ipv6_source_routing_protection_level(&mut self, value: String) {
        self.ipv6_source_routing_protection_level = Some(value);
    }

    /// Gets the value of IPv6SourceRoutingProtectionLevel
    pub fn get_ipv6_source_routing_protection_level(&self) -> Option<&String> {
        self.ipv6_source_routing_protection_level.as_ref()
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

