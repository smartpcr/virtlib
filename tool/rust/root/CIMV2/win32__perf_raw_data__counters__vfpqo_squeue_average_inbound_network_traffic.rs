// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_VFPQoSQueueAverageInboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_VFPQoSQueueAverageInboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AverageInboundBytesAllowedThroughtheQueue")]
    pub average_inbound_bytes_allowed_throughthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageInboundBytesDropped")]
    pub average_inbound_bytes_dropped: Option<u64>,

/// 
    #[serde(rename = "AverageInboundBytesEnteringtheQueue")]
    pub average_inbound_bytes_enteringthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageInboundBytesQueuedduetoBacklog")]
    pub average_inbound_bytes_queueddueto_backlog: Option<u64>,

/// 
    #[serde(rename = "AverageInboundBytesQueuedduetoInsufficientTokens")]
    pub average_inbound_bytes_queueddueto_insufficient_tokens: Option<u64>,

/// 
    #[serde(rename = "AverageInboundBytesResumed")]
    pub average_inbound_bytes_resumed: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPacketsAllowedThroughtheQueue")]
    pub average_inbound_packets_allowed_throughthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPacketsDropped")]
    pub average_inbound_packets_dropped: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPacketsEnteringtheQueue")]
    pub average_inbound_packets_enteringthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPacketsQueuedduetoBacklog")]
    pub average_inbound_packets_queueddueto_backlog: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPacketsQueuedduetoInsufficientTokens")]
    pub average_inbound_packets_queueddueto_insufficient_tokens: Option<u64>,

/// 
    #[serde(rename = "AverageInboundPacketsResumed")]
    pub average_inbound_packets_resumed: Option<u64>,
}

impl Win32_PerfRawData_Counters_VFPQoSQueueAverageInboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            average_inbound_bytes_allowed_throughthe_queue: None,
            average_inbound_bytes_dropped: None,
            average_inbound_bytes_enteringthe_queue: None,
            average_inbound_bytes_queueddueto_backlog: None,
            average_inbound_bytes_queueddueto_insufficient_tokens: None,
            average_inbound_bytes_resumed: None,
            average_inbound_packets_allowed_throughthe_queue: None,
            average_inbound_packets_dropped: None,
            average_inbound_packets_enteringthe_queue: None,
            average_inbound_packets_queueddueto_backlog: None,
            average_inbound_packets_queueddueto_insufficient_tokens: None,
            average_inbound_packets_resumed: None,
        }
    }


    /// Sets the value of AverageInboundBytesAllowedThroughtheQueue
    pub fn set_average_inbound_bytes_allowed_throughthe_queue(&mut self, value: u64) {
        self.average_inbound_bytes_allowed_throughthe_queue = Some(value);
    }

    /// Gets the value of AverageInboundBytesAllowedThroughtheQueue
    pub fn get_average_inbound_bytes_allowed_throughthe_queue(&self) -> Option<&u64> {
        self.average_inbound_bytes_allowed_throughthe_queue.as_ref()
    }

    /// Sets the value of AverageInboundBytesDropped
    pub fn set_average_inbound_bytes_dropped(&mut self, value: u64) {
        self.average_inbound_bytes_dropped = Some(value);
    }

    /// Gets the value of AverageInboundBytesDropped
    pub fn get_average_inbound_bytes_dropped(&self) -> Option<&u64> {
        self.average_inbound_bytes_dropped.as_ref()
    }

    /// Sets the value of AverageInboundBytesEnteringtheQueue
    pub fn set_average_inbound_bytes_enteringthe_queue(&mut self, value: u64) {
        self.average_inbound_bytes_enteringthe_queue = Some(value);
    }

    /// Gets the value of AverageInboundBytesEnteringtheQueue
    pub fn get_average_inbound_bytes_enteringthe_queue(&self) -> Option<&u64> {
        self.average_inbound_bytes_enteringthe_queue.as_ref()
    }

    /// Sets the value of AverageInboundBytesQueuedduetoBacklog
    pub fn set_average_inbound_bytes_queueddueto_backlog(&mut self, value: u64) {
        self.average_inbound_bytes_queueddueto_backlog = Some(value);
    }

    /// Gets the value of AverageInboundBytesQueuedduetoBacklog
    pub fn get_average_inbound_bytes_queueddueto_backlog(&self) -> Option<&u64> {
        self.average_inbound_bytes_queueddueto_backlog.as_ref()
    }

    /// Sets the value of AverageInboundBytesQueuedduetoInsufficientTokens
    pub fn set_average_inbound_bytes_queueddueto_insufficient_tokens(&mut self, value: u64) {
        self.average_inbound_bytes_queueddueto_insufficient_tokens = Some(value);
    }

    /// Gets the value of AverageInboundBytesQueuedduetoInsufficientTokens
    pub fn get_average_inbound_bytes_queueddueto_insufficient_tokens(&self) -> Option<&u64> {
        self.average_inbound_bytes_queueddueto_insufficient_tokens.as_ref()
    }

    /// Sets the value of AverageInboundBytesResumed
    pub fn set_average_inbound_bytes_resumed(&mut self, value: u64) {
        self.average_inbound_bytes_resumed = Some(value);
    }

    /// Gets the value of AverageInboundBytesResumed
    pub fn get_average_inbound_bytes_resumed(&self) -> Option<&u64> {
        self.average_inbound_bytes_resumed.as_ref()
    }

    /// Sets the value of AverageInboundPacketsAllowedThroughtheQueue
    pub fn set_average_inbound_packets_allowed_throughthe_queue(&mut self, value: u64) {
        self.average_inbound_packets_allowed_throughthe_queue = Some(value);
    }

    /// Gets the value of AverageInboundPacketsAllowedThroughtheQueue
    pub fn get_average_inbound_packets_allowed_throughthe_queue(&self) -> Option<&u64> {
        self.average_inbound_packets_allowed_throughthe_queue.as_ref()
    }

    /// Sets the value of AverageInboundPacketsDropped
    pub fn set_average_inbound_packets_dropped(&mut self, value: u64) {
        self.average_inbound_packets_dropped = Some(value);
    }

    /// Gets the value of AverageInboundPacketsDropped
    pub fn get_average_inbound_packets_dropped(&self) -> Option<&u64> {
        self.average_inbound_packets_dropped.as_ref()
    }

    /// Sets the value of AverageInboundPacketsEnteringtheQueue
    pub fn set_average_inbound_packets_enteringthe_queue(&mut self, value: u64) {
        self.average_inbound_packets_enteringthe_queue = Some(value);
    }

    /// Gets the value of AverageInboundPacketsEnteringtheQueue
    pub fn get_average_inbound_packets_enteringthe_queue(&self) -> Option<&u64> {
        self.average_inbound_packets_enteringthe_queue.as_ref()
    }

    /// Sets the value of AverageInboundPacketsQueuedduetoBacklog
    pub fn set_average_inbound_packets_queueddueto_backlog(&mut self, value: u64) {
        self.average_inbound_packets_queueddueto_backlog = Some(value);
    }

    /// Gets the value of AverageInboundPacketsQueuedduetoBacklog
    pub fn get_average_inbound_packets_queueddueto_backlog(&self) -> Option<&u64> {
        self.average_inbound_packets_queueddueto_backlog.as_ref()
    }

    /// Sets the value of AverageInboundPacketsQueuedduetoInsufficientTokens
    pub fn set_average_inbound_packets_queueddueto_insufficient_tokens(&mut self, value: u64) {
        self.average_inbound_packets_queueddueto_insufficient_tokens = Some(value);
    }

    /// Gets the value of AverageInboundPacketsQueuedduetoInsufficientTokens
    pub fn get_average_inbound_packets_queueddueto_insufficient_tokens(&self) -> Option<&u64> {
        self.average_inbound_packets_queueddueto_insufficient_tokens.as_ref()
    }

    /// Sets the value of AverageInboundPacketsResumed
    pub fn set_average_inbound_packets_resumed(&mut self, value: u64) {
        self.average_inbound_packets_resumed = Some(value);
    }

    /// Gets the value of AverageInboundPacketsResumed
    pub fn get_average_inbound_packets_resumed(&self) -> Option<&u64> {
        self.average_inbound_packets_resumed.as_ref()
    }
}

