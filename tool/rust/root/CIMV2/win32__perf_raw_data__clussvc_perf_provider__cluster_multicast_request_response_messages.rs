// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterMulticastRequestResponseMessages struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterMulticastRequestResponseMessages {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "MessagesOutstanding")]
    pub messages_outstanding: Option<u64>,

/// 
    #[serde(rename = "MessagesSent")]
    pub messages_sent: Option<u64>,

/// 
    #[serde(rename = "MessagesSentPersec")]
    pub messages_sent_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterMulticastRequestResponseMessages {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            messages_outstanding: None,
            messages_sent: None,
            messages_sent_persec: None,
        }
    }


    /// Sets the value of MessagesOutstanding
    pub fn set_messages_outstanding(&mut self, value: u64) {
        self.messages_outstanding = Some(value);
    }

    /// Gets the value of MessagesOutstanding
    pub fn get_messages_outstanding(&self) -> Option<&u64> {
        self.messages_outstanding.as_ref()
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

