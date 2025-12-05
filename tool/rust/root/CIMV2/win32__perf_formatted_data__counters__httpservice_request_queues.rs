// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_HTTPServiceRequestQueues struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_HTTPServiceRequestQueues {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ArrivalRate")]
    pub arrival_rate: Option<u64>,

/// 
    #[serde(rename = "CacheHitRate")]
    pub cache_hit_rate: Option<u64>,

/// 
    #[serde(rename = "CurrentQueueSize")]
    pub current_queue_size: Option<u32>,

/// 
    #[serde(rename = "MaxQueueItemAge")]
    pub max_queue_item_age: Option<u64>,

/// 
    #[serde(rename = "PendingReceiveRequests")]
    pub pending_receive_requests: Option<u32>,

/// 
    #[serde(rename = "RejectedRequests")]
    pub rejected_requests: Option<u64>,

/// 
    #[serde(rename = "RejectionRate")]
    pub rejection_rate: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_HTTPServiceRequestQueues {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            arrival_rate: None,
            cache_hit_rate: None,
            current_queue_size: None,
            max_queue_item_age: None,
            pending_receive_requests: None,
            rejected_requests: None,
            rejection_rate: None,
        }
    }


    /// Sets the value of ArrivalRate
    pub fn set_arrival_rate(&mut self, value: u64) {
        self.arrival_rate = Some(value);
    }

    /// Gets the value of ArrivalRate
    pub fn get_arrival_rate(&self) -> Option<&u64> {
        self.arrival_rate.as_ref()
    }

    /// Sets the value of CacheHitRate
    pub fn set_cache_hit_rate(&mut self, value: u64) {
        self.cache_hit_rate = Some(value);
    }

    /// Gets the value of CacheHitRate
    pub fn get_cache_hit_rate(&self) -> Option<&u64> {
        self.cache_hit_rate.as_ref()
    }

    /// Sets the value of CurrentQueueSize
    pub fn set_current_queue_size(&mut self, value: u32) {
        self.current_queue_size = Some(value);
    }

    /// Gets the value of CurrentQueueSize
    pub fn get_current_queue_size(&self) -> Option<&u32> {
        self.current_queue_size.as_ref()
    }

    /// Sets the value of MaxQueueItemAge
    pub fn set_max_queue_item_age(&mut self, value: u64) {
        self.max_queue_item_age = Some(value);
    }

    /// Gets the value of MaxQueueItemAge
    pub fn get_max_queue_item_age(&self) -> Option<&u64> {
        self.max_queue_item_age.as_ref()
    }

    /// Sets the value of PendingReceiveRequests
    pub fn set_pending_receive_requests(&mut self, value: u32) {
        self.pending_receive_requests = Some(value);
    }

    /// Gets the value of PendingReceiveRequests
    pub fn get_pending_receive_requests(&self) -> Option<&u32> {
        self.pending_receive_requests.as_ref()
    }

    /// Sets the value of RejectedRequests
    pub fn set_rejected_requests(&mut self, value: u64) {
        self.rejected_requests = Some(value);
    }

    /// Gets the value of RejectedRequests
    pub fn get_rejected_requests(&self) -> Option<&u64> {
        self.rejected_requests.as_ref()
    }

    /// Sets the value of RejectionRate
    pub fn set_rejection_rate(&mut self, value: u64) {
        self.rejection_rate = Some(value);
    }

    /// Gets the value of RejectionRate
    pub fn get_rejection_rate(&self) -> Option<&u64> {
        self.rejection_rate.as_ref()
    }
}

