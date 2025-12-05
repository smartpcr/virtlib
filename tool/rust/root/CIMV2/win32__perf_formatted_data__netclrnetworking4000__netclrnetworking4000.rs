// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_NETCLRNetworking4000_NETCLRNetworking4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_NETCLRNetworking4000_NETCLRNetworking4000 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesSent")]
    pub bytes_sent: Option<u64>,

/// 
    #[serde(rename = "ConnectionsEstablished")]
    pub connections_established: Option<u32>,

/// 
    #[serde(rename = "DatagramsReceived")]
    pub datagrams_received: Option<u32>,

/// 
    #[serde(rename = "DatagramsSent")]
    pub datagrams_sent: Option<u32>,

/// 
    #[serde(rename = "HttpWebRequestsAbortedPerSec")]
    pub http_web_requests_aborted_per_sec: Option<u32>,

/// 
    #[serde(rename = "HttpWebRequestsAverageLifetime")]
    pub http_web_requests_average_lifetime: Option<u64>,

/// 
    #[serde(rename = "HttpWebRequestsAverageQueueTime")]
    pub http_web_requests_average_queue_time: Option<u64>,

/// 
    #[serde(rename = "HttpWebRequestsCreatedPerSec")]
    pub http_web_requests_created_per_sec: Option<u32>,

/// 
    #[serde(rename = "HttpWebRequestsFailedPerSec")]
    pub http_web_requests_failed_per_sec: Option<u32>,

/// 
    #[serde(rename = "HttpWebRequestsQueuedPerSec")]
    pub http_web_requests_queued_per_sec: Option<u32>,
}

impl Win32_PerfFormattedData_NETCLRNetworking4000_NETCLRNetworking4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            bytes_received: None,
            bytes_sent: None,
            connections_established: None,
            datagrams_received: None,
            datagrams_sent: None,
            http_web_requests_aborted_per_sec: None,
            http_web_requests_average_lifetime: None,
            http_web_requests_average_queue_time: None,
            http_web_requests_created_per_sec: None,
            http_web_requests_failed_per_sec: None,
            http_web_requests_queued_per_sec: None,
        }
    }


    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u64) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u64> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesSent
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.bytes_sent = Some(value);
    }

    /// Gets the value of BytesSent
    pub fn get_bytes_sent(&self) -> Option<&u64> {
        self.bytes_sent.as_ref()
    }

    /// Sets the value of ConnectionsEstablished
    pub fn set_connections_established(&mut self, value: u32) {
        self.connections_established = Some(value);
    }

    /// Gets the value of ConnectionsEstablished
    pub fn get_connections_established(&self) -> Option<&u32> {
        self.connections_established.as_ref()
    }

    /// Sets the value of DatagramsReceived
    pub fn set_datagrams_received(&mut self, value: u32) {
        self.datagrams_received = Some(value);
    }

    /// Gets the value of DatagramsReceived
    pub fn get_datagrams_received(&self) -> Option<&u32> {
        self.datagrams_received.as_ref()
    }

    /// Sets the value of DatagramsSent
    pub fn set_datagrams_sent(&mut self, value: u32) {
        self.datagrams_sent = Some(value);
    }

    /// Gets the value of DatagramsSent
    pub fn get_datagrams_sent(&self) -> Option<&u32> {
        self.datagrams_sent.as_ref()
    }

    /// Sets the value of HttpWebRequestsAbortedPerSec
    pub fn set_http_web_requests_aborted_per_sec(&mut self, value: u32) {
        self.http_web_requests_aborted_per_sec = Some(value);
    }

    /// Gets the value of HttpWebRequestsAbortedPerSec
    pub fn get_http_web_requests_aborted_per_sec(&self) -> Option<&u32> {
        self.http_web_requests_aborted_per_sec.as_ref()
    }

    /// Sets the value of HttpWebRequestsAverageLifetime
    pub fn set_http_web_requests_average_lifetime(&mut self, value: u64) {
        self.http_web_requests_average_lifetime = Some(value);
    }

    /// Gets the value of HttpWebRequestsAverageLifetime
    pub fn get_http_web_requests_average_lifetime(&self) -> Option<&u64> {
        self.http_web_requests_average_lifetime.as_ref()
    }

    /// Sets the value of HttpWebRequestsAverageQueueTime
    pub fn set_http_web_requests_average_queue_time(&mut self, value: u64) {
        self.http_web_requests_average_queue_time = Some(value);
    }

    /// Gets the value of HttpWebRequestsAverageQueueTime
    pub fn get_http_web_requests_average_queue_time(&self) -> Option<&u64> {
        self.http_web_requests_average_queue_time.as_ref()
    }

    /// Sets the value of HttpWebRequestsCreatedPerSec
    pub fn set_http_web_requests_created_per_sec(&mut self, value: u32) {
        self.http_web_requests_created_per_sec = Some(value);
    }

    /// Gets the value of HttpWebRequestsCreatedPerSec
    pub fn get_http_web_requests_created_per_sec(&self) -> Option<&u32> {
        self.http_web_requests_created_per_sec.as_ref()
    }

    /// Sets the value of HttpWebRequestsFailedPerSec
    pub fn set_http_web_requests_failed_per_sec(&mut self, value: u32) {
        self.http_web_requests_failed_per_sec = Some(value);
    }

    /// Gets the value of HttpWebRequestsFailedPerSec
    pub fn get_http_web_requests_failed_per_sec(&self) -> Option<&u32> {
        self.http_web_requests_failed_per_sec.as_ref()
    }

    /// Sets the value of HttpWebRequestsQueuedPerSec
    pub fn set_http_web_requests_queued_per_sec(&mut self, value: u32) {
        self.http_web_requests_queued_per_sec = Some(value);
    }

    /// Gets the value of HttpWebRequestsQueuedPerSec
    pub fn get_http_web_requests_queued_per_sec(&self) -> Option<&u32> {
        self.http_web_requests_queued_per_sec.as_ref()
    }
}

