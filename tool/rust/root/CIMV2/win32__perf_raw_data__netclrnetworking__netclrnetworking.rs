// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETCLRNetworking_NETCLRNetworking struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETCLRNetworking_NETCLRNetworking {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

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
}

impl Win32_PerfRawData_NETCLRNetworking_NETCLRNetworking {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_received: None,
            bytes_sent: None,
            connections_established: None,
            datagrams_received: None,
            datagrams_sent: None,
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
}

