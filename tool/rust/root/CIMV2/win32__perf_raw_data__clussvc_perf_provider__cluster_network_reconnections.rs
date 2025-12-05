// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterNetworkReconnections struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterNetworkReconnections {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "NormalMessageQueueLength")]
    pub normal_message_queue_length: Option<u64>,

/// 
    #[serde(rename = "NormalMessageQueueLengthPersec")]
    pub normal_message_queue_length_persec: Option<u64>,

/// 
    #[serde(rename = "ReconnectCount")]
    pub reconnect_count: Option<u64>,

/// 
    #[serde(rename = "UnacknowledgedMessageQueueLength")]
    pub unacknowledged_message_queue_length: Option<u64>,

/// 
    #[serde(rename = "UnacknowledgedMessageQueueLengthPersec")]
    pub unacknowledged_message_queue_length_persec: Option<u64>,

/// 
    #[serde(rename = "UrgentMessageQueueLength")]
    pub urgent_message_queue_length: Option<u64>,

/// 
    #[serde(rename = "UrgentMessageQueueLengthPersec")]
    pub urgent_message_queue_length_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterNetworkReconnections {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            normal_message_queue_length: None,
            normal_message_queue_length_persec: None,
            reconnect_count: None,
            unacknowledged_message_queue_length: None,
            unacknowledged_message_queue_length_persec: None,
            urgent_message_queue_length: None,
            urgent_message_queue_length_persec: None,
        }
    }


    /// Sets the value of NormalMessageQueueLength
    pub fn set_normal_message_queue_length(&mut self, value: u64) {
        self.normal_message_queue_length = Some(value);
    }

    /// Gets the value of NormalMessageQueueLength
    pub fn get_normal_message_queue_length(&self) -> Option<&u64> {
        self.normal_message_queue_length.as_ref()
    }

    /// Sets the value of NormalMessageQueueLengthPersec
    pub fn set_normal_message_queue_length_persec(&mut self, value: u64) {
        self.normal_message_queue_length_persec = Some(value);
    }

    /// Gets the value of NormalMessageQueueLengthPersec
    pub fn get_normal_message_queue_length_persec(&self) -> Option<&u64> {
        self.normal_message_queue_length_persec.as_ref()
    }

    /// Sets the value of ReconnectCount
    pub fn set_reconnect_count(&mut self, value: u64) {
        self.reconnect_count = Some(value);
    }

    /// Gets the value of ReconnectCount
    pub fn get_reconnect_count(&self) -> Option<&u64> {
        self.reconnect_count.as_ref()
    }

    /// Sets the value of UnacknowledgedMessageQueueLength
    pub fn set_unacknowledged_message_queue_length(&mut self, value: u64) {
        self.unacknowledged_message_queue_length = Some(value);
    }

    /// Gets the value of UnacknowledgedMessageQueueLength
    pub fn get_unacknowledged_message_queue_length(&self) -> Option<&u64> {
        self.unacknowledged_message_queue_length.as_ref()
    }

    /// Sets the value of UnacknowledgedMessageQueueLengthPersec
    pub fn set_unacknowledged_message_queue_length_persec(&mut self, value: u64) {
        self.unacknowledged_message_queue_length_persec = Some(value);
    }

    /// Gets the value of UnacknowledgedMessageQueueLengthPersec
    pub fn get_unacknowledged_message_queue_length_persec(&self) -> Option<&u64> {
        self.unacknowledged_message_queue_length_persec.as_ref()
    }

    /// Sets the value of UrgentMessageQueueLength
    pub fn set_urgent_message_queue_length(&mut self, value: u64) {
        self.urgent_message_queue_length = Some(value);
    }

    /// Gets the value of UrgentMessageQueueLength
    pub fn get_urgent_message_queue_length(&self) -> Option<&u64> {
        self.urgent_message_queue_length.as_ref()
    }

    /// Sets the value of UrgentMessageQueueLengthPersec
    pub fn set_urgent_message_queue_length_persec(&mut self, value: u64) {
        self.urgent_message_queue_length_persec = Some(value);
    }

    /// Gets the value of UrgentMessageQueueLengthPersec
    pub fn get_urgent_message_queue_length_persec(&self) -> Option<&u64> {
        self.urgent_message_queue_length_persec.as_ref()
    }
}

