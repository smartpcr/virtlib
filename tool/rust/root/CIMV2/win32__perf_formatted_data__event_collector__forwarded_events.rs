// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_EventCollector_ForwardedEvents struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_EventCollector_ForwardedEvents {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "EventsDroppedBatchs")]
    pub events_dropped_batchs: Option<u32>,

/// 
    #[serde(rename = "EventsReceivedBatches")]
    pub events_received_batches: Option<u32>,

/// 
    #[serde(rename = "LostEvents")]
    pub lost_events: Option<u64>,

/// 
    #[serde(rename = "ProcessedEvents")]
    pub processed_events: Option<u64>,

/// 
    #[serde(rename = "TimestampOfLatestBatch")]
    pub timestamp_of_latest_batch: Option<u64>,
}

impl Win32_PerfFormattedData_EventCollector_ForwardedEvents {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            events_dropped_batchs: None,
            events_received_batches: None,
            lost_events: None,
            processed_events: None,
            timestamp_of_latest_batch: None,
        }
    }


    /// Sets the value of EventsDroppedBatchs
    pub fn set_events_dropped_batchs(&mut self, value: u32) {
        self.events_dropped_batchs = Some(value);
    }

    /// Gets the value of EventsDroppedBatchs
    pub fn get_events_dropped_batchs(&self) -> Option<&u32> {
        self.events_dropped_batchs.as_ref()
    }

    /// Sets the value of EventsReceivedBatches
    pub fn set_events_received_batches(&mut self, value: u32) {
        self.events_received_batches = Some(value);
    }

    /// Gets the value of EventsReceivedBatches
    pub fn get_events_received_batches(&self) -> Option<&u32> {
        self.events_received_batches.as_ref()
    }

    /// Sets the value of LostEvents
    pub fn set_lost_events(&mut self, value: u64) {
        self.lost_events = Some(value);
    }

    /// Gets the value of LostEvents
    pub fn get_lost_events(&self) -> Option<&u64> {
        self.lost_events.as_ref()
    }

    /// Sets the value of ProcessedEvents
    pub fn set_processed_events(&mut self, value: u64) {
        self.processed_events = Some(value);
    }

    /// Gets the value of ProcessedEvents
    pub fn get_processed_events(&self) -> Option<&u64> {
        self.processed_events.as_ref()
    }

    /// Sets the value of TimestampOfLatestBatch
    pub fn set_timestamp_of_latest_batch(&mut self, value: u64) {
        self.timestamp_of_latest_batch = Some(value);
    }

    /// Gets the value of TimestampOfLatestBatch
    pub fn get_timestamp_of_latest_batch(&self) -> Option<&u64> {
        self.timestamp_of_latest_batch.as_ref()
    }
}

