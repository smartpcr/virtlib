// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetftRouteMonitorAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetftRouteMonitorAdapter {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ACKRecievedPerSec")]
    pub ackrecieved_per_sec: Option<u64>,

/// 
    #[serde(rename = "ACKSentPerSec")]
    pub acksent_per_sec: Option<u64>,

/// 
    #[serde(rename = "HeartbeatsRecievedPerSec")]
    pub heartbeats_recieved_per_sec: Option<u64>,

/// 
    #[serde(rename = "HeartbeatsSentPerSec")]
    pub heartbeats_sent_per_sec: Option<u64>,

/// 
    #[serde(rename = "TotalACKRecieved")]
    pub total_ackrecieved: Option<u64>,

/// 
    #[serde(rename = "TotalACKSent")]
    pub total_acksent: Option<u64>,

/// 
    #[serde(rename = "TotalHeartbeatsRecieved")]
    pub total_heartbeats_recieved: Option<u64>,

/// 
    #[serde(rename = "TotalHeartbeatsSent")]
    pub total_heartbeats_sent: Option<u64>,

/// 
    #[serde(rename = "TotalReceivesDropped")]
    pub total_receives_dropped: Option<u64>,

/// 
    #[serde(rename = "TotalSendRequestsDropped")]
    pub total_send_requests_dropped: Option<u64>,
}

impl Win32_PerfFormattedData_NetFtPerfProvider_ClusterNetftRouteMonitorAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            ackrecieved_per_sec: None,
            acksent_per_sec: None,
            heartbeats_recieved_per_sec: None,
            heartbeats_sent_per_sec: None,
            total_ackrecieved: None,
            total_acksent: None,
            total_heartbeats_recieved: None,
            total_heartbeats_sent: None,
            total_receives_dropped: None,
            total_send_requests_dropped: None,
        }
    }


    /// Sets the value of ACKRecievedPerSec
    pub fn set_ackrecieved_per_sec(&mut self, value: u64) {
        self.ackrecieved_per_sec = Some(value);
    }

    /// Gets the value of ACKRecievedPerSec
    pub fn get_ackrecieved_per_sec(&self) -> Option<&u64> {
        self.ackrecieved_per_sec.as_ref()
    }

    /// Sets the value of ACKSentPerSec
    pub fn set_acksent_per_sec(&mut self, value: u64) {
        self.acksent_per_sec = Some(value);
    }

    /// Gets the value of ACKSentPerSec
    pub fn get_acksent_per_sec(&self) -> Option<&u64> {
        self.acksent_per_sec.as_ref()
    }

    /// Sets the value of HeartbeatsRecievedPerSec
    pub fn set_heartbeats_recieved_per_sec(&mut self, value: u64) {
        self.heartbeats_recieved_per_sec = Some(value);
    }

    /// Gets the value of HeartbeatsRecievedPerSec
    pub fn get_heartbeats_recieved_per_sec(&self) -> Option<&u64> {
        self.heartbeats_recieved_per_sec.as_ref()
    }

    /// Sets the value of HeartbeatsSentPerSec
    pub fn set_heartbeats_sent_per_sec(&mut self, value: u64) {
        self.heartbeats_sent_per_sec = Some(value);
    }

    /// Gets the value of HeartbeatsSentPerSec
    pub fn get_heartbeats_sent_per_sec(&self) -> Option<&u64> {
        self.heartbeats_sent_per_sec.as_ref()
    }

    /// Sets the value of TotalACKRecieved
    pub fn set_total_ackrecieved(&mut self, value: u64) {
        self.total_ackrecieved = Some(value);
    }

    /// Gets the value of TotalACKRecieved
    pub fn get_total_ackrecieved(&self) -> Option<&u64> {
        self.total_ackrecieved.as_ref()
    }

    /// Sets the value of TotalACKSent
    pub fn set_total_acksent(&mut self, value: u64) {
        self.total_acksent = Some(value);
    }

    /// Gets the value of TotalACKSent
    pub fn get_total_acksent(&self) -> Option<&u64> {
        self.total_acksent.as_ref()
    }

    /// Sets the value of TotalHeartbeatsRecieved
    pub fn set_total_heartbeats_recieved(&mut self, value: u64) {
        self.total_heartbeats_recieved = Some(value);
    }

    /// Gets the value of TotalHeartbeatsRecieved
    pub fn get_total_heartbeats_recieved(&self) -> Option<&u64> {
        self.total_heartbeats_recieved.as_ref()
    }

    /// Sets the value of TotalHeartbeatsSent
    pub fn set_total_heartbeats_sent(&mut self, value: u64) {
        self.total_heartbeats_sent = Some(value);
    }

    /// Gets the value of TotalHeartbeatsSent
    pub fn get_total_heartbeats_sent(&self) -> Option<&u64> {
        self.total_heartbeats_sent.as_ref()
    }

    /// Sets the value of TotalReceivesDropped
    pub fn set_total_receives_dropped(&mut self, value: u64) {
        self.total_receives_dropped = Some(value);
    }

    /// Gets the value of TotalReceivesDropped
    pub fn get_total_receives_dropped(&self) -> Option<&u64> {
        self.total_receives_dropped.as_ref()
    }

    /// Sets the value of TotalSendRequestsDropped
    pub fn set_total_send_requests_dropped(&mut self, value: u64) {
        self.total_send_requests_dropped = Some(value);
    }

    /// Gets the value of TotalSendRequestsDropped
    pub fn get_total_send_requests_dropped(&self) -> Option<&u64> {
        self.total_send_requests_dropped.as_ref()
    }
}

