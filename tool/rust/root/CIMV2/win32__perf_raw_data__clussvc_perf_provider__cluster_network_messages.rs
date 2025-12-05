// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterNetworkMessages struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterNetworkMessages {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesSent")]
    pub bytes_sent: Option<u64>,

/// 
    #[serde(rename = "BytesSentPersec")]
    pub bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "MessagesReceived")]
    pub messages_received: Option<u64>,

/// 
    #[serde(rename = "MessagesReceivedPersec")]
    pub messages_received_persec: Option<u64>,

/// 
    #[serde(rename = "MessagesSent")]
    pub messages_sent: Option<u64>,

/// 
    #[serde(rename = "MessagesSentPersec")]
    pub messages_sent_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterNetworkMessages {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_received: None,
            bytes_received_persec: None,
            bytes_sent: None,
            bytes_sent_persec: None,
            messages_received: None,
            messages_received_persec: None,
            messages_sent: None,
            messages_sent_persec: None,
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

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesSent
    pub fn set_bytes_sent(&mut self, value: u64) {
        self.bytes_sent = Some(value);
    }

    /// Gets the value of BytesSent
    pub fn get_bytes_sent(&self) -> Option<&u64> {
        self.bytes_sent.as_ref()
    }

    /// Sets the value of BytesSentPersec
    pub fn set_bytes_sent_persec(&mut self, value: u64) {
        self.bytes_sent_persec = Some(value);
    }

    /// Gets the value of BytesSentPersec
    pub fn get_bytes_sent_persec(&self) -> Option<&u64> {
        self.bytes_sent_persec.as_ref()
    }

    /// Sets the value of MessagesReceived
    pub fn set_messages_received(&mut self, value: u64) {
        self.messages_received = Some(value);
    }

    /// Gets the value of MessagesReceived
    pub fn get_messages_received(&self) -> Option<&u64> {
        self.messages_received.as_ref()
    }

    /// Sets the value of MessagesReceivedPersec
    pub fn set_messages_received_persec(&mut self, value: u64) {
        self.messages_received_persec = Some(value);
    }

    /// Gets the value of MessagesReceivedPersec
    pub fn get_messages_received_persec(&self) -> Option<&u64> {
        self.messages_received_persec.as_ref()
    }

    /// Sets the value of MessagesSent
    pub fn set_messages_sent(&mut self, value: u64) {
        self.messages_sent = Some(value);
    }

    /// Gets the value of MessagesSent
    pub fn get_messages_sent(&self) -> Option<&u64> {
        self.messages_sent.as_ref()
    }

    /// Sets the value of MessagesSentPersec
    pub fn set_messages_sent_persec(&mut self, value: u64) {
        self.messages_sent_persec = Some(value);
    }

    /// Gets the value of MessagesSentPersec
    pub fn get_messages_sent_persec(&self) -> Option<&u64> {
        self.messages_sent_persec.as_ref()
    }
}

