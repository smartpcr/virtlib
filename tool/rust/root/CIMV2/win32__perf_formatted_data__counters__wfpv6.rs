// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_WFPv6 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_WFPv6 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ActiveInboundConnections")]
    pub active_inbound_connections: Option<u32>,

/// 
    #[serde(rename = "ActiveOutboundConnections")]
    pub active_outbound_connections: Option<u32>,

/// 
    #[serde(rename = "AllowedClassifiesPersec")]
    pub allowed_classifies_persec: Option<u32>,

/// 
    #[serde(rename = "BlockedBinds")]
    pub blocked_binds: Option<u32>,

/// 
    #[serde(rename = "InboundConnections")]
    pub inbound_connections: Option<u32>,

/// 
    #[serde(rename = "InboundConnectionsAllowedPersec")]
    pub inbound_connections_allowed_persec: Option<u32>,

/// 
    #[serde(rename = "InboundConnectionsBlockedPersec")]
    pub inbound_connections_blocked_persec: Option<u32>,

/// 
    #[serde(rename = "InboundPacketsDiscardedPersec")]
    pub inbound_packets_discarded_persec: Option<u32>,

/// 
    #[serde(rename = "OutboundConnections")]
    pub outbound_connections: Option<u32>,

/// 
    #[serde(rename = "OutboundConnectionsAllowedPersec")]
    pub outbound_connections_allowed_persec: Option<u32>,

/// 
    #[serde(rename = "OutboundConnectionsBlockedPersec")]
    pub outbound_connections_blocked_persec: Option<u32>,

/// 
    #[serde(rename = "OutboundPacketsDiscardedPersec")]
    pub outbound_packets_discarded_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsDiscardedPersec")]
    pub packets_discarded_persec: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_WFPv6 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            active_inbound_connections: None,
            active_outbound_connections: None,
            allowed_classifies_persec: None,
            blocked_binds: None,
            inbound_connections: None,
            inbound_connections_allowed_persec: None,
            inbound_connections_blocked_persec: None,
            inbound_packets_discarded_persec: None,
            outbound_connections: None,
            outbound_connections_allowed_persec: None,
            outbound_connections_blocked_persec: None,
            outbound_packets_discarded_persec: None,
            packets_discarded_persec: None,
        }
    }


    /// Sets the value of ActiveInboundConnections
    pub fn set_active_inbound_connections(&mut self, value: u32) {
        self.active_inbound_connections = Some(value);
    }

    /// Gets the value of ActiveInboundConnections
    pub fn get_active_inbound_connections(&self) -> Option<&u32> {
        self.active_inbound_connections.as_ref()
    }

    /// Sets the value of ActiveOutboundConnections
    pub fn set_active_outbound_connections(&mut self, value: u32) {
        self.active_outbound_connections = Some(value);
    }

    /// Gets the value of ActiveOutboundConnections
    pub fn get_active_outbound_connections(&self) -> Option<&u32> {
        self.active_outbound_connections.as_ref()
    }

    /// Sets the value of AllowedClassifiesPersec
    pub fn set_allowed_classifies_persec(&mut self, value: u32) {
        self.allowed_classifies_persec = Some(value);
    }

    /// Gets the value of AllowedClassifiesPersec
    pub fn get_allowed_classifies_persec(&self) -> Option<&u32> {
        self.allowed_classifies_persec.as_ref()
    }

    /// Sets the value of BlockedBinds
    pub fn set_blocked_binds(&mut self, value: u32) {
        self.blocked_binds = Some(value);
    }

    /// Gets the value of BlockedBinds
    pub fn get_blocked_binds(&self) -> Option<&u32> {
        self.blocked_binds.as_ref()
    }

    /// Sets the value of InboundConnections
    pub fn set_inbound_connections(&mut self, value: u32) {
        self.inbound_connections = Some(value);
    }

    /// Gets the value of InboundConnections
    pub fn get_inbound_connections(&self) -> Option<&u32> {
        self.inbound_connections.as_ref()
    }

    /// Sets the value of InboundConnectionsAllowedPersec
    pub fn set_inbound_connections_allowed_persec(&mut self, value: u32) {
        self.inbound_connections_allowed_persec = Some(value);
    }

    /// Gets the value of InboundConnectionsAllowedPersec
    pub fn get_inbound_connections_allowed_persec(&self) -> Option<&u32> {
        self.inbound_connections_allowed_persec.as_ref()
    }

    /// Sets the value of InboundConnectionsBlockedPersec
    pub fn set_inbound_connections_blocked_persec(&mut self, value: u32) {
        self.inbound_connections_blocked_persec = Some(value);
    }

    /// Gets the value of InboundConnectionsBlockedPersec
    pub fn get_inbound_connections_blocked_persec(&self) -> Option<&u32> {
        self.inbound_connections_blocked_persec.as_ref()
    }

    /// Sets the value of InboundPacketsDiscardedPersec
    pub fn set_inbound_packets_discarded_persec(&mut self, value: u32) {
        self.inbound_packets_discarded_persec = Some(value);
    }

    /// Gets the value of InboundPacketsDiscardedPersec
    pub fn get_inbound_packets_discarded_persec(&self) -> Option<&u32> {
        self.inbound_packets_discarded_persec.as_ref()
    }

    /// Sets the value of OutboundConnections
    pub fn set_outbound_connections(&mut self, value: u32) {
        self.outbound_connections = Some(value);
    }

    /// Gets the value of OutboundConnections
    pub fn get_outbound_connections(&self) -> Option<&u32> {
        self.outbound_connections.as_ref()
    }

    /// Sets the value of OutboundConnectionsAllowedPersec
    pub fn set_outbound_connections_allowed_persec(&mut self, value: u32) {
        self.outbound_connections_allowed_persec = Some(value);
    }

    /// Gets the value of OutboundConnectionsAllowedPersec
    pub fn get_outbound_connections_allowed_persec(&self) -> Option<&u32> {
        self.outbound_connections_allowed_persec.as_ref()
    }

    /// Sets the value of OutboundConnectionsBlockedPersec
    pub fn set_outbound_connections_blocked_persec(&mut self, value: u32) {
        self.outbound_connections_blocked_persec = Some(value);
    }

    /// Gets the value of OutboundConnectionsBlockedPersec
    pub fn get_outbound_connections_blocked_persec(&self) -> Option<&u32> {
        self.outbound_connections_blocked_persec.as_ref()
    }

    /// Sets the value of OutboundPacketsDiscardedPersec
    pub fn set_outbound_packets_discarded_persec(&mut self, value: u32) {
        self.outbound_packets_discarded_persec = Some(value);
    }

    /// Gets the value of OutboundPacketsDiscardedPersec
    pub fn get_outbound_packets_discarded_persec(&self) -> Option<&u32> {
        self.outbound_packets_discarded_persec.as_ref()
    }

    /// Sets the value of PacketsDiscardedPersec
    pub fn set_packets_discarded_persec(&mut self, value: u32) {
        self.packets_discarded_persec = Some(value);
    }

    /// Gets the value of PacketsDiscardedPersec
    pub fn get_packets_discarded_persec(&self) -> Option<&u32> {
        self.packets_discarded_persec.as_ref()
    }
}

