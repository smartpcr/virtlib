// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSRedbook_Performance struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSRedbook_Performance {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "DataProcessed")]
    pub data_processed: Option<i64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "StreamPausedCount")]
    pub stream_paused_count: Option<u32>,

/// 
    #[serde(rename = "TimeReadDelay")]
    pub time_read_delay: Option<i64>,

/// 
    #[serde(rename = "TimeReading")]
    pub time_reading: Option<i64>,

/// 
    #[serde(rename = "TimeStreamDelay")]
    pub time_stream_delay: Option<i64>,

/// 
    #[serde(rename = "TimeStreaming")]
    pub time_streaming: Option<i64>,
}

impl MSRedbook_Performance {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            data_processed: None,
            instance_name: None,
            stream_paused_count: None,
            time_read_delay: None,
            time_reading: None,
            time_stream_delay: None,
            time_streaming: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of DataProcessed
    pub fn set_data_processed(&mut self, value: i64) {
        self.data_processed = Some(value);
    }

    /// Gets the value of DataProcessed
    pub fn get_data_processed(&self) -> Option<&i64> {
        self.data_processed.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of StreamPausedCount
    pub fn set_stream_paused_count(&mut self, value: u32) {
        self.stream_paused_count = Some(value);
    }

    /// Gets the value of StreamPausedCount
    pub fn get_stream_paused_count(&self) -> Option<&u32> {
        self.stream_paused_count.as_ref()
    }

    /// Sets the value of TimeReadDelay
    pub fn set_time_read_delay(&mut self, value: i64) {
        self.time_read_delay = Some(value);
    }

    /// Gets the value of TimeReadDelay
    pub fn get_time_read_delay(&self) -> Option<&i64> {
        self.time_read_delay.as_ref()
    }

    /// Sets the value of TimeReading
    pub fn set_time_reading(&mut self, value: i64) {
        self.time_reading = Some(value);
    }

    /// Gets the value of TimeReading
    pub fn get_time_reading(&self) -> Option<&i64> {
        self.time_reading.as_ref()
    }

    /// Sets the value of TimeStreamDelay
    pub fn set_time_stream_delay(&mut self, value: i64) {
        self.time_stream_delay = Some(value);
    }

    /// Gets the value of TimeStreamDelay
    pub fn get_time_stream_delay(&self) -> Option<&i64> {
        self.time_stream_delay.as_ref()
    }

    /// Sets the value of TimeStreaming
    pub fn set_time_streaming(&mut self, value: i64) {
        self.time_streaming = Some(value);
    }

    /// Gets the value of TimeStreaming
    pub fn get_time_streaming(&self) -> Option<&i64> {
        self.time_streaming.as_ref()
    }
}

