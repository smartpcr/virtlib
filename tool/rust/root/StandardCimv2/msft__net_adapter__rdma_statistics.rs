// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_RdmaStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_RdmaStatistics {

/// 
    #[serde(rename = "AcceptedConnections")]
    pub accepted_connections: Option<u64>,

/// 
    #[serde(rename = "ActiveConnections")]
    pub active_connections: Option<u64>,

/// 
    #[serde(rename = "CompletionQueueErrors")]
    pub completion_queue_errors: Option<u64>,

/// 
    #[serde(rename = "ConnectionErrors")]
    pub connection_errors: Option<u64>,

/// 
    #[serde(rename = "FailedConnectionAttempts")]
    pub failed_connection_attempts: Option<u64>,

/// 
    #[serde(rename = "InboundBytes")]
    pub inbound_bytes: Option<u64>,

/// 
    #[serde(rename = "InboundFrames")]
    pub inbound_frames: Option<u64>,

/// 
    #[serde(rename = "InitiatedConnections")]
    pub initiated_connections: Option<u64>,

/// 
    #[serde(rename = "OutboundBytes")]
    pub outbound_bytes: Option<u64>,

/// 
    #[serde(rename = "OutboundFrames")]
    pub outbound_frames: Option<u64>,
}

impl MSFT_NetAdapter_RdmaStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            accepted_connections: None,
            active_connections: None,
            completion_queue_errors: None,
            connection_errors: None,
            failed_connection_attempts: None,
            inbound_bytes: None,
            inbound_frames: None,
            initiated_connections: None,
            outbound_bytes: None,
            outbound_frames: None,
        }
    }


    /// Sets the value of AcceptedConnections
    pub fn set_accepted_connections(&mut self, value: u64) {
        self.accepted_connections = Some(value);
    }

    /// Gets the value of AcceptedConnections
    pub fn get_accepted_connections(&self) -> Option<&u64> {
        self.accepted_connections.as_ref()
    }

    /// Sets the value of ActiveConnections
    pub fn set_active_connections(&mut self, value: u64) {
        self.active_connections = Some(value);
    }

    /// Gets the value of ActiveConnections
    pub fn get_active_connections(&self) -> Option<&u64> {
        self.active_connections.as_ref()
    }

    /// Sets the value of CompletionQueueErrors
    pub fn set_completion_queue_errors(&mut self, value: u64) {
        self.completion_queue_errors = Some(value);
    }

    /// Gets the value of CompletionQueueErrors
    pub fn get_completion_queue_errors(&self) -> Option<&u64> {
        self.completion_queue_errors.as_ref()
    }

    /// Sets the value of ConnectionErrors
    pub fn set_connection_errors(&mut self, value: u64) {
        self.connection_errors = Some(value);
    }

    /// Gets the value of ConnectionErrors
    pub fn get_connection_errors(&self) -> Option<&u64> {
        self.connection_errors.as_ref()
    }

    /// Sets the value of FailedConnectionAttempts
    pub fn set_failed_connection_attempts(&mut self, value: u64) {
        self.failed_connection_attempts = Some(value);
    }

    /// Gets the value of FailedConnectionAttempts
    pub fn get_failed_connection_attempts(&self) -> Option<&u64> {
        self.failed_connection_attempts.as_ref()
    }

    /// Sets the value of InboundBytes
    pub fn set_inbound_bytes(&mut self, value: u64) {
        self.inbound_bytes = Some(value);
    }

    /// Gets the value of InboundBytes
    pub fn get_inbound_bytes(&self) -> Option<&u64> {
        self.inbound_bytes.as_ref()
    }

    /// Sets the value of InboundFrames
    pub fn set_inbound_frames(&mut self, value: u64) {
        self.inbound_frames = Some(value);
    }

    /// Gets the value of InboundFrames
    pub fn get_inbound_frames(&self) -> Option<&u64> {
        self.inbound_frames.as_ref()
    }

    /// Sets the value of InitiatedConnections
    pub fn set_initiated_connections(&mut self, value: u64) {
        self.initiated_connections = Some(value);
    }

    /// Gets the value of InitiatedConnections
    pub fn get_initiated_connections(&self) -> Option<&u64> {
        self.initiated_connections.as_ref()
    }

    /// Sets the value of OutboundBytes
    pub fn set_outbound_bytes(&mut self, value: u64) {
        self.outbound_bytes = Some(value);
    }

    /// Gets the value of OutboundBytes
    pub fn get_outbound_bytes(&self) -> Option<&u64> {
        self.outbound_bytes.as_ref()
    }

    /// Sets the value of OutboundFrames
    pub fn set_outbound_frames(&mut self, value: u64) {
        self.outbound_frames = Some(value);
    }

    /// Gets the value of OutboundFrames
    pub fn get_outbound_frames(&self) -> Option<&u64> {
        self.outbound_frames.as_ref()
    }
}

