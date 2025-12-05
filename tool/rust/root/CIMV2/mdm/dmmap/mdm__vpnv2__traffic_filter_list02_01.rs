// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_TrafficFilterList02_01 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_TrafficFilterList02_01 {

/// 
    #[serde(rename = "Claims")]
    pub claims: Option<String>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<String>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LocalAddressRanges")]
    pub local_address_ranges: Option<String>,

/// 
    #[serde(rename = "LocalPortRanges")]
    pub local_port_ranges: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<i32>,

/// 
    #[serde(rename = "RemoteAddressRanges")]
    pub remote_address_ranges: Option<String>,

/// 
    #[serde(rename = "RemotePortRanges")]
    pub remote_port_ranges: Option<String>,

/// 
    #[serde(rename = "RoutingPolicyType")]
    pub routing_policy_type: Option<String>,
}

impl MDM_VPNv2_TrafficFilterList02_01 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            claims: None,
            direction: None,
            instance_id: None,
            local_address_ranges: None,
            local_port_ranges: None,
            parent_id: None,
            protocol: None,
            remote_address_ranges: None,
            remote_port_ranges: None,
            routing_policy_type: None,
        }
    }


    /// Sets the value of Claims
    pub fn set_claims(&mut self, value: String) {
        self.claims = Some(value);
    }

    /// Gets the value of Claims
    pub fn get_claims(&self) -> Option<&String> {
        self.claims.as_ref()
    }

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: String) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&String> {
        self.direction.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
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

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: i32) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&i32> {
        self.protocol.as_ref()
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

    /// Sets the value of RoutingPolicyType
    pub fn set_routing_policy_type(&mut self, value: String) {
        self.routing_policy_type = Some(value);
    }

    /// Gets the value of RoutingPolicyType
    pub fn get_routing_policy_type(&self) -> Option<&String> {
        self.routing_policy_type.as_ref()
    }
}

