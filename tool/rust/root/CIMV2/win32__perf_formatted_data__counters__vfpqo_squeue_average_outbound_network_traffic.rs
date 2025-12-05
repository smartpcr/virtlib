// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_VFPQoSQueueAverageOutboundNetworkTraffic struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_VFPQoSQueueAverageOutboundNetworkTraffic {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AverageOutboundBytesAllowedThroughtheQueue")]
    pub average_outbound_bytes_allowed_throughthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundBytesDropped")]
    pub average_outbound_bytes_dropped: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundBytesEnteringtheQueue")]
    pub average_outbound_bytes_enteringthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundBytesQueuedduetoBacklog")]
    pub average_outbound_bytes_queueddueto_backlog: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundBytesQueuedduetoInsufficientTokens")]
    pub average_outbound_bytes_queueddueto_insufficient_tokens: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundBytesResumed")]
    pub average_outbound_bytes_resumed: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPacketsAllowedThroughtheQueue")]
    pub average_outbound_packets_allowed_throughthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPacketsDropped")]
    pub average_outbound_packets_dropped: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPacketsEnteringtheQueue")]
    pub average_outbound_packets_enteringthe_queue: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPacketsQueuedduetoBacklog")]
    pub average_outbound_packets_queueddueto_backlog: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPacketsQueuedduetoInsufficientTokens")]
    pub average_outbound_packets_queueddueto_insufficient_tokens: Option<u64>,

/// 
    #[serde(rename = "AverageOutboundPacketsResumed")]
    pub average_outbound_packets_resumed: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_VFPQoSQueueAverageOutboundNetworkTraffic {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            average_outbound_bytes_allowed_throughthe_queue: None,
            average_outbound_bytes_dropped: None,
            average_outbound_bytes_enteringthe_queue: None,
            average_outbound_bytes_queueddueto_backlog: None,
            average_outbound_bytes_queueddueto_insufficient_tokens: None,
            average_outbound_bytes_resumed: None,
            average_outbound_packets_allowed_throughthe_queue: None,
            average_outbound_packets_dropped: None,
            average_outbound_packets_enteringthe_queue: None,
            average_outbound_packets_queueddueto_backlog: None,
            average_outbound_packets_queueddueto_insufficient_tokens: None,
            average_outbound_packets_resumed: None,
        }
    }


    /// Sets the value of AverageOutboundBytesAllowedThroughtheQueue
    pub fn set_average_outbound_bytes_allowed_throughthe_queue(&mut self, value: u64) {
        self.average_outbound_bytes_allowed_throughthe_queue = Some(value);
    }

    /// Gets the value of AverageOutboundBytesAllowedThroughtheQueue
    pub fn get_average_outbound_bytes_allowed_throughthe_queue(&self) -> Option<&u64> {
        self.average_outbound_bytes_allowed_throughthe_queue.as_ref()
    }

    /// Sets the value of AverageOutboundBytesDropped
    pub fn set_average_outbound_bytes_dropped(&mut self, value: u64) {
        self.average_outbound_bytes_dropped = Some(value);
    }

    /// Gets the value of AverageOutboundBytesDropped
    pub fn get_average_outbound_bytes_dropped(&self) -> Option<&u64> {
        self.average_outbound_bytes_dropped.as_ref()
    }

    /// Sets the value of AverageOutboundBytesEnteringtheQueue
    pub fn set_average_outbound_bytes_enteringthe_queue(&mut self, value: u64) {
        self.average_outbound_bytes_enteringthe_queue = Some(value);
    }

    /// Gets the value of AverageOutboundBytesEnteringtheQueue
    pub fn get_average_outbound_bytes_enteringthe_queue(&self) -> Option<&u64> {
        self.average_outbound_bytes_enteringthe_queue.as_ref()
    }

    /// Sets the value of AverageOutboundBytesQueuedduetoBacklog
    pub fn set_average_outbound_bytes_queueddueto_backlog(&mut self, value: u64) {
        self.average_outbound_bytes_queueddueto_backlog = Some(value);
    }

    /// Gets the value of AverageOutboundBytesQueuedduetoBacklog
    pub fn get_average_outbound_bytes_queueddueto_backlog(&self) -> Option<&u64> {
        self.average_outbound_bytes_queueddueto_backlog.as_ref()
    }

    /// Sets the value of AverageOutboundBytesQueuedduetoInsufficientTokens
    pub fn set_average_outbound_bytes_queueddueto_insufficient_tokens(&mut self, value: u64) {
        self.average_outbound_bytes_queueddueto_insufficient_tokens = Some(value);
    }

    /// Gets the value of AverageOutboundBytesQueuedduetoInsufficientTokens
    pub fn get_average_outbound_bytes_queueddueto_insufficient_tokens(&self) -> Option<&u64> {
        self.average_outbound_bytes_queueddueto_insufficient_tokens.as_ref()
    }

    /// Sets the value of AverageOutboundBytesResumed
    pub fn set_average_outbound_bytes_resumed(&mut self, value: u64) {
        self.average_outbound_bytes_resumed = Some(value);
    }

    /// Gets the value of AverageOutboundBytesResumed
    pub fn get_average_outbound_bytes_resumed(&self) -> Option<&u64> {
        self.average_outbound_bytes_resumed.as_ref()
    }

    /// Sets the value of AverageOutboundPacketsAllowedThroughtheQueue
    pub fn set_average_outbound_packets_allowed_throughthe_queue(&mut self, value: u64) {
        self.average_outbound_packets_allowed_throughthe_queue = Some(value);
    }

    /// Gets the value of AverageOutboundPacketsAllowedThroughtheQueue
    pub fn get_average_outbound_packets_allowed_throughthe_queue(&self) -> Option<&u64> {
        self.average_outbound_packets_allowed_throughthe_queue.as_ref()
    }

    /// Sets the value of AverageOutboundPacketsDropped
    pub fn set_average_outbound_packets_dropped(&mut self, value: u64) {
        self.average_outbound_packets_dropped = Some(value);
    }

    /// Gets the value of AverageOutboundPacketsDropped
    pub fn get_average_outbound_packets_dropped(&self) -> Option<&u64> {
        self.average_outbound_packets_dropped.as_ref()
    }

    /// Sets the value of AverageOutboundPacketsEnteringtheQueue
    pub fn set_average_outbound_packets_enteringthe_queue(&mut self, value: u64) {
        self.average_outbound_packets_enteringthe_queue = Some(value);
    }

    /// Gets the value of AverageOutboundPacketsEnteringtheQueue
    pub fn get_average_outbound_packets_enteringthe_queue(&self) -> Option<&u64> {
        self.average_outbound_packets_enteringthe_queue.as_ref()
    }

    /// Sets the value of AverageOutboundPacketsQueuedduetoBacklog
    pub fn set_average_outbound_packets_queueddueto_backlog(&mut self, value: u64) {
        self.average_outbound_packets_queueddueto_backlog = Some(value);
    }

    /// Gets the value of AverageOutboundPacketsQueuedduetoBacklog
    pub fn get_average_outbound_packets_queueddueto_backlog(&self) -> Option<&u64> {
        self.average_outbound_packets_queueddueto_backlog.as_ref()
    }

    /// Sets the value of AverageOutboundPacketsQueuedduetoInsufficientTokens
    pub fn set_average_outbound_packets_queueddueto_insufficient_tokens(&mut self, value: u64) {
        self.average_outbound_packets_queueddueto_insufficient_tokens = Some(value);
    }

    /// Gets the value of AverageOutboundPacketsQueuedduetoInsufficientTokens
    pub fn get_average_outbound_packets_queueddueto_insufficient_tokens(&self) -> Option<&u64> {
        self.average_outbound_packets_queueddueto_insufficient_tokens.as_ref()
    }

    /// Sets the value of AverageOutboundPacketsResumed
    pub fn set_average_outbound_packets_resumed(&mut self, value: u64) {
        self.average_outbound_packets_resumed = Some(value);
    }

    /// Gets the value of AverageOutboundPacketsResumed
    pub fn get_average_outbound_packets_resumed(&self) -> Option<&u64> {
        self.average_outbound_packets_resumed.as_ref()
    }
}

