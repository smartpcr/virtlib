// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_QMIPSECStats struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_QMIPSECStats {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "ActiveSA")]
    pub active_sa: Option<u64>,

/// 
    #[serde(rename = "ActiveTunnels")]
    pub active_tunnels: Option<u64>,

/// 
    #[serde(rename = "AuthenticatedBytesReceived")]
    pub authenticated_bytes_received: Option<u64>,

/// 
    #[serde(rename = "AuthenticatedBytesSent")]
    pub authenticated_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "BadSPIPackets")]
    pub bad_spipackets: Option<u64>,

/// 
    #[serde(rename = "ConfidentialBytesReceived")]
    pub confidential_bytes_received: Option<u64>,

/// 
    #[serde(rename = "ConfidentialBytesSent")]
    pub confidential_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "KeyAdditions")]
    pub key_additions: Option<u64>,

/// 
    #[serde(rename = "KeyDeletions")]
    pub key_deletions: Option<u64>,

/// 
    #[serde(rename = "PacketsNotAuthenticated")]
    pub packets_not_authenticated: Option<u64>,

/// 
    #[serde(rename = "PacketsNotDecrypted")]
    pub packets_not_decrypted: Option<u64>,

/// 
    #[serde(rename = "PacketsWithReplayDetection")]
    pub packets_with_replay_detection: Option<u64>,

/// 
    #[serde(rename = "PendingKeyOperations")]
    pub pending_key_operations: Option<u64>,

/// 
    #[serde(rename = "ReKeys")]
    pub re_keys: Option<u64>,

/// 
    #[serde(rename = "TransportBytesReceived")]
    pub transport_bytes_received: Option<u64>,

/// 
    #[serde(rename = "TransportBytesSent")]
    pub transport_bytes_sent: Option<u64>,

/// 
    #[serde(rename = "TunnelBytesReceived")]
    pub tunnel_bytes_received: Option<u64>,

/// 
    #[serde(rename = "TunnelBytesSent")]
    pub tunnel_bytes_sent: Option<u64>,
}

impl MSiSCSI_QMIPSECStats {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            active_sa: None,
            active_tunnels: None,
            authenticated_bytes_received: None,
            authenticated_bytes_sent: None,
            bad_spipackets: None,
            confidential_bytes_received: None,
            confidential_bytes_sent: None,
            instance_name: None,
            key_additions: None,
            key_deletions: None,
            packets_not_authenticated: None,
            packets_not_decrypted: None,
            packets_with_replay_detection: None,
            pending_key_operations: None,
            re_keys: None,
            transport_bytes_received: None,
            transport_bytes_sent: None,
            tunnel_bytes_received: None,
            tunnel_bytes_sent: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of ActiveSA
    pub fn set_active_sa(&mut self, value: u64) {
        self.active_sa = Some(value);
    }

    /// Gets the value of ActiveSA
    pub fn get_active_sa(&self) -> Option<&u64> {
        self.active_sa.as_ref()
    }

    /// Sets the value of ActiveTunnels
    pub fn set_active_tunnels(&mut self, value: u64) {
        self.active_tunnels = Some(value);
    }

    /// Gets the value of ActiveTunnels
    pub fn get_active_tunnels(&self) -> Option<&u64> {
        self.active_tunnels.as_ref()
    }

    /// Sets the value of AuthenticatedBytesReceived
    pub fn set_authenticated_bytes_received(&mut self, value: u64) {
        self.authenticated_bytes_received = Some(value);
    }

    /// Gets the value of AuthenticatedBytesReceived
    pub fn get_authenticated_bytes_received(&self) -> Option<&u64> {
        self.authenticated_bytes_received.as_ref()
    }

    /// Sets the value of AuthenticatedBytesSent
    pub fn set_authenticated_bytes_sent(&mut self, value: u64) {
        self.authenticated_bytes_sent = Some(value);
    }

    /// Gets the value of AuthenticatedBytesSent
    pub fn get_authenticated_bytes_sent(&self) -> Option<&u64> {
        self.authenticated_bytes_sent.as_ref()
    }

    /// Sets the value of BadSPIPackets
    pub fn set_bad_spipackets(&mut self, value: u64) {
        self.bad_spipackets = Some(value);
    }

    /// Gets the value of BadSPIPackets
    pub fn get_bad_spipackets(&self) -> Option<&u64> {
        self.bad_spipackets.as_ref()
    }

    /// Sets the value of ConfidentialBytesReceived
    pub fn set_confidential_bytes_received(&mut self, value: u64) {
        self.confidential_bytes_received = Some(value);
    }

    /// Gets the value of ConfidentialBytesReceived
    pub fn get_confidential_bytes_received(&self) -> Option<&u64> {
        self.confidential_bytes_received.as_ref()
    }

    /// Sets the value of ConfidentialBytesSent
    pub fn set_confidential_bytes_sent(&mut self, value: u64) {
        self.confidential_bytes_sent = Some(value);
    }

    /// Gets the value of ConfidentialBytesSent
    pub fn get_confidential_bytes_sent(&self) -> Option<&u64> {
        self.confidential_bytes_sent.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of KeyAdditions
    pub fn set_key_additions(&mut self, value: u64) {
        self.key_additions = Some(value);
    }

    /// Gets the value of KeyAdditions
    pub fn get_key_additions(&self) -> Option<&u64> {
        self.key_additions.as_ref()
    }

    /// Sets the value of KeyDeletions
    pub fn set_key_deletions(&mut self, value: u64) {
        self.key_deletions = Some(value);
    }

    /// Gets the value of KeyDeletions
    pub fn get_key_deletions(&self) -> Option<&u64> {
        self.key_deletions.as_ref()
    }

    /// Sets the value of PacketsNotAuthenticated
    pub fn set_packets_not_authenticated(&mut self, value: u64) {
        self.packets_not_authenticated = Some(value);
    }

    /// Gets the value of PacketsNotAuthenticated
    pub fn get_packets_not_authenticated(&self) -> Option<&u64> {
        self.packets_not_authenticated.as_ref()
    }

    /// Sets the value of PacketsNotDecrypted
    pub fn set_packets_not_decrypted(&mut self, value: u64) {
        self.packets_not_decrypted = Some(value);
    }

    /// Gets the value of PacketsNotDecrypted
    pub fn get_packets_not_decrypted(&self) -> Option<&u64> {
        self.packets_not_decrypted.as_ref()
    }

    /// Sets the value of PacketsWithReplayDetection
    pub fn set_packets_with_replay_detection(&mut self, value: u64) {
        self.packets_with_replay_detection = Some(value);
    }

    /// Gets the value of PacketsWithReplayDetection
    pub fn get_packets_with_replay_detection(&self) -> Option<&u64> {
        self.packets_with_replay_detection.as_ref()
    }

    /// Sets the value of PendingKeyOperations
    pub fn set_pending_key_operations(&mut self, value: u64) {
        self.pending_key_operations = Some(value);
    }

    /// Gets the value of PendingKeyOperations
    pub fn get_pending_key_operations(&self) -> Option<&u64> {
        self.pending_key_operations.as_ref()
    }

    /// Sets the value of ReKeys
    pub fn set_re_keys(&mut self, value: u64) {
        self.re_keys = Some(value);
    }

    /// Gets the value of ReKeys
    pub fn get_re_keys(&self) -> Option<&u64> {
        self.re_keys.as_ref()
    }

    /// Sets the value of TransportBytesReceived
    pub fn set_transport_bytes_received(&mut self, value: u64) {
        self.transport_bytes_received = Some(value);
    }

    /// Gets the value of TransportBytesReceived
    pub fn get_transport_bytes_received(&self) -> Option<&u64> {
        self.transport_bytes_received.as_ref()
    }

    /// Sets the value of TransportBytesSent
    pub fn set_transport_bytes_sent(&mut self, value: u64) {
        self.transport_bytes_sent = Some(value);
    }

    /// Gets the value of TransportBytesSent
    pub fn get_transport_bytes_sent(&self) -> Option<&u64> {
        self.transport_bytes_sent.as_ref()
    }

    /// Sets the value of TunnelBytesReceived
    pub fn set_tunnel_bytes_received(&mut self, value: u64) {
        self.tunnel_bytes_received = Some(value);
    }

    /// Gets the value of TunnelBytesReceived
    pub fn get_tunnel_bytes_received(&self) -> Option<&u64> {
        self.tunnel_bytes_received.as_ref()
    }

    /// Sets the value of TunnelBytesSent
    pub fn set_tunnel_bytes_sent(&mut self, value: u64) {
        self.tunnel_bytes_sent = Some(value);
    }

    /// Gets the value of TunnelBytesSent
    pub fn get_tunnel_bytes_sent(&self) -> Option<&u64> {
        self.tunnel_bytes_sent.as_ref()
    }
}

