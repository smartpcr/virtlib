// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.EventTracingManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_AutologgerConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_AutologgerConfig {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "BufferSize")]
    pub buffer_size: Option<u32>,

/// 
    #[serde(rename = "ClockType")]
    pub clock_type: Option<u32>,

/// 
    #[serde(rename = "DisableRealtimePersistence")]
    pub disable_realtime_persistence: Option<u32>,

/// 
    #[serde(rename = "FileCount")]
    pub file_count: Option<u32>,

/// 
    #[serde(rename = "FileMax")]
    pub file_max: Option<u32>,

/// 
    #[serde(rename = "FlushTimer")]
    pub flush_timer: Option<u32>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "InitStatus")]
    pub init_status: Option<u32>,

/// 
    #[serde(rename = "LocalFilePath")]
    pub local_file_path: Option<String>,

/// 
    #[serde(rename = "LogFileMode")]
    pub log_file_mode: Option<u32>,

/// 
    #[serde(rename = "MaximumBuffers")]
    pub maximum_buffers: Option<u32>,

/// 
    #[serde(rename = "MaximumFileSize")]
    pub maximum_file_size: Option<u32>,

/// 
    #[serde(rename = "MinimumBuffers")]
    pub minimum_buffers: Option<u32>,

/// 
    #[serde(rename = "Start")]
    pub start: Option<u32>,
}

impl MSFT_AutologgerConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            buffer_size: None,
            clock_type: None,
            disable_realtime_persistence: None,
            file_count: None,
            file_max: None,
            flush_timer: None,
            guid: None,
            init_status: None,
            local_file_path: None,
            log_file_mode: None,
            maximum_buffers: None,
            maximum_file_size: None,
            minimum_buffers: None,
            start: None,
        }
    }


    /// Sets the value of BufferSize
    pub fn set_buffer_size(&mut self, value: u32) {
        self.buffer_size = Some(value);
    }

    /// Gets the value of BufferSize
    pub fn get_buffer_size(&self) -> Option<&u32> {
        self.buffer_size.as_ref()
    }

    /// Sets the value of ClockType
    pub fn set_clock_type(&mut self, value: u32) {
        self.clock_type = Some(value);
    }

    /// Gets the value of ClockType
    pub fn get_clock_type(&self) -> Option<&u32> {
        self.clock_type.as_ref()
    }

    /// Sets the value of DisableRealtimePersistence
    pub fn set_disable_realtime_persistence(&mut self, value: u32) {
        self.disable_realtime_persistence = Some(value);
    }

    /// Gets the value of DisableRealtimePersistence
    pub fn get_disable_realtime_persistence(&self) -> Option<&u32> {
        self.disable_realtime_persistence.as_ref()
    }

    /// Sets the value of FileCount
    pub fn set_file_count(&mut self, value: u32) {
        self.file_count = Some(value);
    }

    /// Gets the value of FileCount
    pub fn get_file_count(&self) -> Option<&u32> {
        self.file_count.as_ref()
    }

    /// Sets the value of FileMax
    pub fn set_file_max(&mut self, value: u32) {
        self.file_max = Some(value);
    }

    /// Gets the value of FileMax
    pub fn get_file_max(&self) -> Option<&u32> {
        self.file_max.as_ref()
    }

    /// Sets the value of FlushTimer
    pub fn set_flush_timer(&mut self, value: u32) {
        self.flush_timer = Some(value);
    }

    /// Gets the value of FlushTimer
    pub fn get_flush_timer(&self) -> Option<&u32> {
        self.flush_timer.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of InitStatus
    pub fn set_init_status(&mut self, value: u32) {
        self.init_status = Some(value);
    }

    /// Gets the value of InitStatus
    pub fn get_init_status(&self) -> Option<&u32> {
        self.init_status.as_ref()
    }

    /// Sets the value of LocalFilePath
    pub fn set_local_file_path(&mut self, value: String) {
        self.local_file_path = Some(value);
    }

    /// Gets the value of LocalFilePath
    pub fn get_local_file_path(&self) -> Option<&String> {
        self.local_file_path.as_ref()
    }

    /// Sets the value of LogFileMode
    pub fn set_log_file_mode(&mut self, value: u32) {
        self.log_file_mode = Some(value);
    }

    /// Gets the value of LogFileMode
    pub fn get_log_file_mode(&self) -> Option<&u32> {
        self.log_file_mode.as_ref()
    }

    /// Sets the value of MaximumBuffers
    pub fn set_maximum_buffers(&mut self, value: u32) {
        self.maximum_buffers = Some(value);
    }

    /// Gets the value of MaximumBuffers
    pub fn get_maximum_buffers(&self) -> Option<&u32> {
        self.maximum_buffers.as_ref()
    }

    /// Sets the value of MaximumFileSize
    pub fn set_maximum_file_size(&mut self, value: u32) {
        self.maximum_file_size = Some(value);
    }

    /// Gets the value of MaximumFileSize
    pub fn get_maximum_file_size(&self) -> Option<&u32> {
        self.maximum_file_size.as_ref()
    }

    /// Sets the value of MinimumBuffers
    pub fn set_minimum_buffers(&mut self, value: u32) {
        self.minimum_buffers = Some(value);
    }

    /// Gets the value of MinimumBuffers
    pub fn get_minimum_buffers(&self) -> Option<&u32> {
        self.minimum_buffers.as_ref()
    }

    /// Sets the value of Start
    pub fn set_start(&mut self, value: u32) {
        self.start = Some(value);
    }

    /// Gets the value of Start
    pub fn get_start(&self) -> Option<&u32> {
        self.start.as_ref()
    }
}

