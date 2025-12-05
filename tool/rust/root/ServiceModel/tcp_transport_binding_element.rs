// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TcpTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TcpTransportBindingElement {
    #[serde(flatten)]
    pub base: ConnectionOrientedTransportBindingElement,

/// The connection pool settings.
    #[serde(rename = "ConnectionPoolSettings")]
    pub connection_pool_settings: Option<TcpConnectionPoolSettings>,

/// The extended protection policy used by the server to validate incoming client connections.
    #[serde(rename = "ExtendedProtectionPolicy")]
    pub extended_protection_policy: Option<ExtendedProtectionPolicy>,

/// The maximum number of queued connection requests that can be pending.
    #[serde(rename = "ListenBacklog")]
    pub listen_backlog: Option<i32>,

/// A boolean value that specifies whether TCP port sharing is enabled for this connection.
    #[serde(rename = "PortSharingEnabled")]
    pub port_sharing_enabled: Option<bool>,

/// A Boolean value that specifies whether Teredo (a technology for addressing clients that are behind firewalls) is enabled. 
    #[serde(rename = "TeredoEnabled")]
    pub teredo_enabled: Option<bool>,
}

impl TcpTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: ConnectionOrientedTransportBindingElement::new(),
            connection_pool_settings: None,
            extended_protection_policy: None,
            listen_backlog: None,
            port_sharing_enabled: None,
            teredo_enabled: None,
        }
    }


    /// Sets the value of ConnectionPoolSettings
    pub fn set_connection_pool_settings(&mut self, value: TcpConnectionPoolSettings) {
        self.connection_pool_settings = Some(value);
    }

    /// Gets the value of ConnectionPoolSettings
    pub fn get_connection_pool_settings(&self) -> Option<&TcpConnectionPoolSettings> {
        self.connection_pool_settings.as_ref()
    }

    /// Sets the value of ExtendedProtectionPolicy
    pub fn set_extended_protection_policy(&mut self, value: ExtendedProtectionPolicy) {
        self.extended_protection_policy = Some(value);
    }

    /// Gets the value of ExtendedProtectionPolicy
    pub fn get_extended_protection_policy(&self) -> Option<&ExtendedProtectionPolicy> {
        self.extended_protection_policy.as_ref()
    }

    /// Sets the value of ListenBacklog
    pub fn set_listen_backlog(&mut self, value: i32) {
        self.listen_backlog = Some(value);
    }

    /// Gets the value of ListenBacklog
    pub fn get_listen_backlog(&self) -> Option<&i32> {
        self.listen_backlog.as_ref()
    }

    /// Sets the value of PortSharingEnabled
    pub fn set_port_sharing_enabled(&mut self, value: bool) {
        self.port_sharing_enabled = Some(value);
    }

    /// Gets the value of PortSharingEnabled
    pub fn get_port_sharing_enabled(&self) -> Option<&bool> {
        self.port_sharing_enabled.as_ref()
    }

    /// Sets the value of TeredoEnabled
    pub fn set_teredo_enabled(&mut self, value: bool) {
        self.teredo_enabled = Some(value);
    }

    /// Gets the value of TeredoEnabled
    pub fn get_teredo_enabled(&self) -> Option<&bool> {
        self.teredo_enabled.as_ref()
    }
}

