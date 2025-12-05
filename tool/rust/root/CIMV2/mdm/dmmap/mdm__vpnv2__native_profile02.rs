// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_VPNv2_NativeProfile02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_VPNv2_NativeProfile02 {

/// 
    #[serde(rename = "DisableClassBasedDefaultRoute")]
    pub disable_class_based_default_route: Option<bool>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "L2tpPsk")]
    pub l2tp_psk: Option<String>,

/// 
    #[serde(rename = "NativeProtocolType")]
    pub native_protocol_type: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PlumbIKEv2TSAsRoutes")]
    pub plumb_ikev2_tsas_routes: Option<bool>,

/// 
    #[serde(rename = "RoutingPolicyType")]
    pub routing_policy_type: Option<String>,

/// 
    #[serde(rename = "Servers")]
    pub servers: Option<String>,
}

impl MDM_VPNv2_NativeProfile02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            disable_class_based_default_route: None,
            instance_id: None,
            l2tp_psk: None,
            native_protocol_type: None,
            parent_id: None,
            plumb_ikev2_tsas_routes: None,
            routing_policy_type: None,
            servers: None,
        }
    }


    /// Sets the value of DisableClassBasedDefaultRoute
    pub fn set_disable_class_based_default_route(&mut self, value: bool) {
        self.disable_class_based_default_route = Some(value);
    }

    /// Gets the value of DisableClassBasedDefaultRoute
    pub fn get_disable_class_based_default_route(&self) -> Option<&bool> {
        self.disable_class_based_default_route.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of L2tpPsk
    pub fn set_l2tp_psk(&mut self, value: String) {
        self.l2tp_psk = Some(value);
    }

    /// Gets the value of L2tpPsk
    pub fn get_l2tp_psk(&self) -> Option<&String> {
        self.l2tp_psk.as_ref()
    }

    /// Sets the value of NativeProtocolType
    pub fn set_native_protocol_type(&mut self, value: String) {
        self.native_protocol_type = Some(value);
    }

    /// Gets the value of NativeProtocolType
    pub fn get_native_protocol_type(&self) -> Option<&String> {
        self.native_protocol_type.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PlumbIKEv2TSAsRoutes
    pub fn set_plumb_ikev2_tsas_routes(&mut self, value: bool) {
        self.plumb_ikev2_tsas_routes = Some(value);
    }

    /// Gets the value of PlumbIKEv2TSAsRoutes
    pub fn get_plumb_ikev2_tsas_routes(&self) -> Option<&bool> {
        self.plumb_ikev2_tsas_routes.as_ref()
    }

    /// Sets the value of RoutingPolicyType
    pub fn set_routing_policy_type(&mut self, value: String) {
        self.routing_policy_type = Some(value);
    }

    /// Gets the value of RoutingPolicyType
    pub fn get_routing_policy_type(&self) -> Option<&String> {
        self.routing_policy_type.as_ref()
    }

    /// Sets the value of Servers
    pub fn set_servers(&mut self, value: String) {
        self.servers = Some(value);
    }

    /// Gets the value of Servers
    pub fn get_servers(&self) -> Option<&String> {
        self.servers.as_ref()
    }
}

