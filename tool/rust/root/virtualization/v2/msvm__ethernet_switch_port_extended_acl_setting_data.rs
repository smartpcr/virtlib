// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_EthernetSwitchPortExtendedAclSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_EthernetSwitchPortExtendedAclSettingData {
    #[serde(flatten)]
    pub base: Msvm_EthernetSwitchPortFeatureSettingData,

/// 
    #[serde(rename = "Action")]
    pub action: Option<u8>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<u8>,

/// 
    #[serde(rename = "IdleSessionTimeout")]
    pub idle_session_timeout: Option<u16>,

/// 
    #[serde(rename = "IsolationID")]
    pub isolation_id: Option<u32>,

/// 
    #[serde(rename = "LocalIPAddress")]
    pub local_ipaddress: Option<String>,

/// 
    #[serde(rename = "LocalPort")]
    pub local_port: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

/// 
    #[serde(rename = "RemoteIPAddress")]
    pub remote_ipaddress: Option<String>,

/// 
    #[serde(rename = "RemotePort")]
    pub remote_port: Option<String>,

/// 
    #[serde(rename = "Stateful")]
    pub stateful: Option<bool>,

/// 
    #[serde(rename = "Weight")]
    pub weight: Option<u16>,
}

impl Msvm_EthernetSwitchPortExtendedAclSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_EthernetSwitchPortFeatureSettingData::new(),
            action: None,
            direction: None,
            idle_session_timeout: None,
            isolation_id: None,
            local_ipaddress: None,
            local_port: None,
            name: None,
            protocol: None,
            remote_ipaddress: None,
            remote_port: None,
            stateful: None,
            weight: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: u8) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&u8> {
        self.action.as_ref()
    }

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: u8) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&u8> {
        self.direction.as_ref()
    }

    /// Sets the value of IdleSessionTimeout
    pub fn set_idle_session_timeout(&mut self, value: u16) {
        self.idle_session_timeout = Some(value);
    }

    /// Gets the value of IdleSessionTimeout
    pub fn get_idle_session_timeout(&self) -> Option<&u16> {
        self.idle_session_timeout.as_ref()
    }

    /// Sets the value of IsolationID
    pub fn set_isolation_id(&mut self, value: u32) {
        self.isolation_id = Some(value);
    }

    /// Gets the value of IsolationID
    pub fn get_isolation_id(&self) -> Option<&u32> {
        self.isolation_id.as_ref()
    }

    /// Sets the value of LocalIPAddress
    pub fn set_local_ipaddress(&mut self, value: String) {
        self.local_ipaddress = Some(value);
    }

    /// Gets the value of LocalIPAddress
    pub fn get_local_ipaddress(&self) -> Option<&String> {
        self.local_ipaddress.as_ref()
    }

    /// Sets the value of LocalPort
    pub fn set_local_port(&mut self, value: String) {
        self.local_port = Some(value);
    }

    /// Gets the value of LocalPort
    pub fn get_local_port(&self) -> Option<&String> {
        self.local_port.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: String) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&String> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemoteIPAddress
    pub fn set_remote_ipaddress(&mut self, value: String) {
        self.remote_ipaddress = Some(value);
    }

    /// Gets the value of RemoteIPAddress
    pub fn get_remote_ipaddress(&self) -> Option<&String> {
        self.remote_ipaddress.as_ref()
    }

    /// Sets the value of RemotePort
    pub fn set_remote_port(&mut self, value: String) {
        self.remote_port = Some(value);
    }

    /// Gets the value of RemotePort
    pub fn get_remote_port(&self) -> Option<&String> {
        self.remote_port.as_ref()
    }

    /// Sets the value of Stateful
    pub fn set_stateful(&mut self, value: bool) {
        self.stateful = Some(value);
    }

    /// Gets the value of Stateful
    pub fn get_stateful(&self) -> Option<&bool> {
        self.stateful.as_ref()
    }

    /// Sets the value of Weight
    pub fn set_weight(&mut self, value: u16) {
        self.weight = Some(value);
    }

    /// Gets the value of Weight
    pub fn get_weight(&self) -> Option<&u16> {
        self.weight.as_ref()
    }
}

impl Msvm_EthernetSwitchPortExtendedAclSettingData {
    /// Gets the related Msvm_EthernetSwitchFeatureCapabilities object(s)
    pub fn get_related__ethernet_switch_feature_capabilities(&self) -> Result<Msvm_EthernetSwitchFeatureCapabilities, WmiError> {
        self.get_related("Msvm_EthernetSwitchFeatureCapabilities")
    }

}

