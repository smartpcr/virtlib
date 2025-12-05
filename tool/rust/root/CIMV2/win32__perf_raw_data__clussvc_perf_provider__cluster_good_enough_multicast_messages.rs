// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterGoodEnoughMulticastMessages struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterGoodEnoughMulticastMessages {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "MessageQueueLength")]
    pub message_queue_length: Option<u64>,

/// 
    #[serde(rename = "UnacknowledgedMessages")]
    pub unacknowledged_messages: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterGoodEnoughMulticastMessages {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            message_queue_length: None,
            unacknowledged_messages: None,
        }
    }


    /// Sets the value of MessageQueueLength
    pub fn set_message_queue_length(&mut self, value: u64) {
        self.message_queue_length = Some(value);
    }

    /// Gets the value of MessageQueueLength
    pub fn get_message_queue_length(&self) -> Option<&u64> {
        self.message_queue_length.as_ref()
    }

    /// Sets the value of UnacknowledgedMessages
    pub fn set_unacknowledged_messages(&mut self, value: u64) {
        self.unacknowledged_messages = Some(value);
    }

    /// Gets the value of UnacknowledgedMessages
    pub fn get_unacknowledged_messages(&self) -> Option<&u64> {
        self.unacknowledged_messages.as_ref()
    }
}

