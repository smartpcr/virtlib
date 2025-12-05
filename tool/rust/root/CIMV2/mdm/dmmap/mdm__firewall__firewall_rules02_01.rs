// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Firewall_FirewallRules02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Firewall_FirewallRules02_01 {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<String>,

/// 
    #[serde(rename = "EdgeTraversal")]
    pub edge_traversal: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<bool>,

/// 
    #[serde(rename = "IcmpTypesAndCodes")]
    pub icmp_types_and_codes: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "InterfaceTypes")]
    pub interface_types: Option<String>,

/// 
    #[serde(rename = "LocalAddressRanges")]
    pub local_address_ranges: Option<String>,

/// 
    #[serde(rename = "LocalPortRanges")]
    pub local_port_ranges: Option<String>,

/// 
    #[serde(rename = "LocalUserAuthorizationList")]
    pub local_user_authorization_list: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PolicyAppId")]
    pub policy_app_id: Option<String>,

/// 
    #[serde(rename = "Profiles")]
    pub profiles: Option<i32>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<i32>,

/// 
    #[serde(rename = "RemoteAddressDynamicKeywords")]
    pub remote_address_dynamic_keywords: Option<String>,

/// 
    #[serde(rename = "RemoteAddressRanges")]
    pub remote_address_ranges: Option<String>,

/// 
    #[serde(rename = "RemotePortRanges")]
    pub remote_port_ranges: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<i32>,
}

impl MDM_Firewall_FirewallRules02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            direction: None,
            edge_traversal: None,
            enabled: None,
            friendly_name: None,
            icmp_types_and_codes: None,
            instance_id: None,
            interface_types: None,
            local_address_ranges: None,
            local_port_ranges: None,
            local_user_authorization_list: None,
            name: None,
            parent_id: None,
            policy_app_id: None,
            profiles: None,
            protocol: None,
            remote_address_dynamic_keywords: None,
            remote_address_ranges: None,
            remote_port_ranges: None,
            status: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: String) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&String> {
        self.direction.as_ref()
    }

    /// Sets the value of EdgeTraversal
    pub fn set_edge_traversal(&mut self, value: bool) {
        self.edge_traversal = Some(value);
    }

    /// Gets the value of EdgeTraversal
    pub fn get_edge_traversal(&self) -> Option<&bool> {
        self.edge_traversal.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: bool) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&bool> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of IcmpTypesAndCodes
    pub fn set_icmp_types_and_codes(&mut self, value: String) {
        self.icmp_types_and_codes = Some(value);
    }

    /// Gets the value of IcmpTypesAndCodes
    pub fn get_icmp_types_and_codes(&self) -> Option<&String> {
        self.icmp_types_and_codes.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of InterfaceTypes
    pub fn set_interface_types(&mut self, value: String) {
        self.interface_types = Some(value);
    }

    /// Gets the value of InterfaceTypes
    pub fn get_interface_types(&self) -> Option<&String> {
        self.interface_types.as_ref()
    }

    /// Sets the value of LocalAddressRanges
    pub fn set_local_address_ranges(&mut self, value: String) {
        self.local_address_ranges = Some(value);
    }

    /// Gets the value of LocalAddressRanges
    pub fn get_local_address_ranges(&self) -> Option<&String> {
        self.local_address_ranges.as_ref()
    }

    /// Sets the value of LocalPortRanges
    pub fn set_local_port_ranges(&mut self, value: String) {
        self.local_port_ranges = Some(value);
    }

    /// Gets the value of LocalPortRanges
    pub fn get_local_port_ranges(&self) -> Option<&String> {
        self.local_port_ranges.as_ref()
    }

    /// Sets the value of LocalUserAuthorizationList
    pub fn set_local_user_authorization_list(&mut self, value: String) {
        self.local_user_authorization_list = Some(value);
    }

    /// Gets the value of LocalUserAuthorizationList
    pub fn get_local_user_authorization_list(&self) -> Option<&String> {
        self.local_user_authorization_list.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PolicyAppId
    pub fn set_policy_app_id(&mut self, value: String) {
        self.policy_app_id = Some(value);
    }

    /// Gets the value of PolicyAppId
    pub fn get_policy_app_id(&self) -> Option<&String> {
        self.policy_app_id.as_ref()
    }

    /// Sets the value of Profiles
    pub fn set_profiles(&mut self, value: i32) {
        self.profiles = Some(value);
    }

    /// Gets the value of Profiles
    pub fn get_profiles(&self) -> Option<&i32> {
        self.profiles.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: i32) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&i32> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemoteAddressDynamicKeywords
    pub fn set_remote_address_dynamic_keywords(&mut self, value: String) {
        self.remote_address_dynamic_keywords = Some(value);
    }

    /// Gets the value of RemoteAddressDynamicKeywords
    pub fn get_remote_address_dynamic_keywords(&self) -> Option<&String> {
        self.remote_address_dynamic_keywords.as_ref()
    }

    /// Sets the value of RemoteAddressRanges
    pub fn set_remote_address_ranges(&mut self, value: String) {
        self.remote_address_ranges = Some(value);
    }

    /// Gets the value of RemoteAddressRanges
    pub fn get_remote_address_ranges(&self) -> Option<&String> {
        self.remote_address_ranges.as_ref()
    }

    /// Sets the value of RemotePortRanges
    pub fn set_remote_port_ranges(&mut self, value: String) {
        self.remote_port_ranges = Some(value);
    }

    /// Gets the value of RemotePortRanges
    pub fn get_remote_port_ranges(&self) -> Option<&String> {
        self.remote_port_ranges.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }
}

