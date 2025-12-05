// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_ConnectionProperties struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_ConnectionProperties {

/// 
    #[serde(rename = "Connection")]
    pub connection: Option<HNet_Connection>,

/// 
    #[serde(rename = "IsBridge")]
    pub is_bridge: Option<bool>,

/// 
    #[serde(rename = "IsBridgeMember")]
    pub is_bridge_member: Option<bool>,

/// 
    #[serde(rename = "IsFirewalled")]
    pub is_firewalled: Option<bool>,

/// 
    #[serde(rename = "IsIcsPrivate")]
    pub is_ics_private: Option<bool>,

/// 
    #[serde(rename = "IsIcsPublic")]
    pub is_ics_public: Option<bool>,
}

impl HNet_ConnectionProperties {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection: None,
            is_bridge: None,
            is_bridge_member: None,
            is_firewalled: None,
            is_ics_private: None,
            is_ics_public: None,
        }
    }


    /// Sets the value of Connection
    pub fn set_connection(&mut self, value: HNet_Connection) {
        self.connection = Some(value);
    }

    /// Gets the value of Connection
    pub fn get_connection(&self) -> Option<&HNet_Connection> {
        self.connection.as_ref()
    }

    /// Sets the value of IsBridge
    pub fn set_is_bridge(&mut self, value: bool) {
        self.is_bridge = Some(value);
    }

    /// Gets the value of IsBridge
    pub fn get_is_bridge(&self) -> Option<&bool> {
        self.is_bridge.as_ref()
    }

    /// Sets the value of IsBridgeMember
    pub fn set_is_bridge_member(&mut self, value: bool) {
        self.is_bridge_member = Some(value);
    }

    /// Gets the value of IsBridgeMember
    pub fn get_is_bridge_member(&self) -> Option<&bool> {
        self.is_bridge_member.as_ref()
    }

    /// Sets the value of IsFirewalled
    pub fn set_is_firewalled(&mut self, value: bool) {
        self.is_firewalled = Some(value);
    }

    /// Gets the value of IsFirewalled
    pub fn get_is_firewalled(&self) -> Option<&bool> {
        self.is_firewalled.as_ref()
    }

    /// Sets the value of IsIcsPrivate
    pub fn set_is_ics_private(&mut self, value: bool) {
        self.is_ics_private = Some(value);
    }

    /// Gets the value of IsIcsPrivate
    pub fn get_is_ics_private(&self) -> Option<&bool> {
        self.is_ics_private.as_ref()
    }

    /// Sets the value of IsIcsPublic
    pub fn set_is_ics_public(&mut self, value: bool) {
        self.is_ics_public = Some(value);
    }

    /// Gets the value of IsIcsPublic
    pub fn get_is_ics_public(&self) -> Option<&bool> {
        self.is_ics_public.as_ref()
    }
}

