// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_HTTPServiceUrlGroups struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_HTTPServiceUrlGroups {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "AllRequests")]
    pub all_requests: Option<u32>,

/// 
    #[serde(rename = "BytesReceivedRate")]
    pub bytes_received_rate: Option<u64>,

/// 
    #[serde(rename = "BytesSentRate")]
    pub bytes_sent_rate: Option<u64>,

/// 
    #[serde(rename = "BytesTransferredRate")]
    pub bytes_transferred_rate: Option<u64>,

/// 
    #[serde(rename = "ConnectionAttempts")]
    pub connection_attempts: Option<u32>,

/// 
    #[serde(rename = "CurrentConnections")]
    pub current_connections: Option<u32>,

/// 
    #[serde(rename = "GetRequests")]
    pub get_requests: Option<u32>,

/// 
    #[serde(rename = "HeadRequests")]
    pub head_requests: Option<u32>,

/// 
    #[serde(rename = "MaxConnections")]
    pub max_connections: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_HTTPServiceUrlGroups {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            all_requests: None,
            bytes_received_rate: None,
            bytes_sent_rate: None,
            bytes_transferred_rate: None,
            connection_attempts: None,
            current_connections: None,
            get_requests: None,
            head_requests: None,
            max_connections: None,
        }
    }


    /// Sets the value of AllRequests
    pub fn set_all_requests(&mut self, value: u32) {
        self.all_requests = Some(value);
    }

    /// Gets the value of AllRequests
    pub fn get_all_requests(&self) -> Option<&u32> {
        self.all_requests.as_ref()
    }

    /// Sets the value of BytesReceivedRate
    pub fn set_bytes_received_rate(&mut self, value: u64) {
        self.bytes_received_rate = Some(value);
    }

    /// Gets the value of BytesReceivedRate
    pub fn get_bytes_received_rate(&self) -> Option<&u64> {
        self.bytes_received_rate.as_ref()
    }

    /// Sets the value of BytesSentRate
    pub fn set_bytes_sent_rate(&mut self, value: u64) {
        self.bytes_sent_rate = Some(value);
    }

    /// Gets the value of BytesSentRate
    pub fn get_bytes_sent_rate(&self) -> Option<&u64> {
        self.bytes_sent_rate.as_ref()
    }

    /// Sets the value of BytesTransferredRate
    pub fn set_bytes_transferred_rate(&mut self, value: u64) {
        self.bytes_transferred_rate = Some(value);
    }

    /// Gets the value of BytesTransferredRate
    pub fn get_bytes_transferred_rate(&self) -> Option<&u64> {
        self.bytes_transferred_rate.as_ref()
    }

    /// Sets the value of ConnectionAttempts
    pub fn set_connection_attempts(&mut self, value: u32) {
        self.connection_attempts = Some(value);
    }

    /// Gets the value of ConnectionAttempts
    pub fn get_connection_attempts(&self) -> Option<&u32> {
        self.connection_attempts.as_ref()
    }

    /// Sets the value of CurrentConnections
    pub fn set_current_connections(&mut self, value: u32) {
        self.current_connections = Some(value);
    }

    /// Gets the value of CurrentConnections
    pub fn get_current_connections(&self) -> Option<&u32> {
        self.current_connections.as_ref()
    }

    /// Sets the value of GetRequests
    pub fn set_get_requests(&mut self, value: u32) {
        self.get_requests = Some(value);
    }

    /// Gets the value of GetRequests
    pub fn get_get_requests(&self) -> Option<&u32> {
        self.get_requests.as_ref()
    }

    /// Sets the value of HeadRequests
    pub fn set_head_requests(&mut self, value: u32) {
        self.head_requests = Some(value);
    }

    /// Gets the value of HeadRequests
    pub fn get_head_requests(&self) -> Option<&u32> {
        self.head_requests.as_ref()
    }

    /// Sets the value of MaxConnections
    pub fn set_max_connections(&mut self, value: u32) {
        self.max_connections = Some(value);
    }

    /// Gets the value of MaxConnections
    pub fn get_max_connections(&self) -> Option<&u32> {
        self.max_connections.as_ref()
    }
}

