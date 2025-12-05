// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// EventTrace_V1_Header struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventTrace_V1_Header {
    #[serde(flatten)]
    pub base: EventTraceEvent_V1,

/// 
    #[serde(rename = "BootTime")]
    pub boot_time: Option<u64>,

/// 
    #[serde(rename = "BufferSize")]
    pub buffer_size: Option<u32>,

/// 
    #[serde(rename = "BuffersLost")]
    pub buffers_lost: Option<u32>,

/// 
    #[serde(rename = "BuffersWritten")]
    pub buffers_written: Option<u32>,

/// 
    #[serde(rename = "CPUSpeed")]
    pub cpuspeed: Option<u32>,

/// 
    #[serde(rename = "EndTime")]
    pub end_time: Option<u64>,

/// 
    #[serde(rename = "EventsLost")]
    pub events_lost: Option<u32>,

/// 
    #[serde(rename = "LogFileMode")]
    pub log_file_mode: Option<u32>,

/// 
    #[serde(rename = "LogFileName")]
    pub log_file_name: Option<u32>,

/// 
    #[serde(rename = "LogFileNameString")]
    pub log_file_name_string: Option<String>,

/// 
    #[serde(rename = "LoggerName")]
    pub logger_name: Option<u32>,

/// 
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<u32>,

/// 
    #[serde(rename = "NumberOfProcessors")]
    pub number_of_processors: Option<u32>,

/// 
    #[serde(rename = "PerfFreq")]
    pub perf_freq: Option<u64>,

/// 
    #[serde(rename = "PointerSize")]
    pub pointer_size: Option<u32>,

/// 
    #[serde(rename = "ProviderVersion")]
    pub provider_version: Option<u32>,

/// 
    #[serde(rename = "ReservedFlags")]
    pub reserved_flags: Option<u32>,

/// 
    #[serde(rename = "SessionNameString")]
    pub session_name_string: Option<String>,

/// 
    #[serde(rename = "StartBuffers")]
    pub start_buffers: Option<u32>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<u64>,

/// 
    #[serde(rename = "TimerResolution")]
    pub timer_resolution: Option<u32>,

/// 
    #[serde(rename = "TimeZoneInformation")]
    pub time_zone_information: Vec<u8>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<u32>,
}

impl EventTrace_V1_Header {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent_V1::new(),
            boot_time: None,
            buffer_size: None,
            buffers_lost: None,
            buffers_written: None,
            cpuspeed: None,
            end_time: None,
            events_lost: None,
            log_file_mode: None,
            log_file_name: None,
            log_file_name_string: None,
            logger_name: None,
            max_file_size: None,
            number_of_processors: None,
            perf_freq: None,
            pointer_size: None,
            provider_version: None,
            reserved_flags: None,
            session_name_string: None,
            start_buffers: None,
            start_time: None,
            timer_resolution: None,
            time_zone_information: Vec::new(),
            version: None,
        }
    }


    /// Sets the value of BootTime
    pub fn set_boot_time(&mut self, value: u64) {
        self.boot_time = Some(value);
    }

    /// Gets the value of BootTime
    pub fn get_boot_time(&self) -> Option<&u64> {
        self.boot_time.as_ref()
    }

    /// Sets the value of BufferSize
    pub fn set_buffer_size(&mut self, value: u32) {
        self.buffer_size = Some(value);
    }

    /// Gets the value of BufferSize
    pub fn get_buffer_size(&self) -> Option<&u32> {
        self.buffer_size.as_ref()
    }

    /// Sets the value of BuffersLost
    pub fn set_buffers_lost(&mut self, value: u32) {
        self.buffers_lost = Some(value);
    }

    /// Gets the value of BuffersLost
    pub fn get_buffers_lost(&self) -> Option<&u32> {
        self.buffers_lost.as_ref()
    }

    /// Sets the value of BuffersWritten
    pub fn set_buffers_written(&mut self, value: u32) {
        self.buffers_written = Some(value);
    }

    /// Gets the value of BuffersWritten
    pub fn get_buffers_written(&self) -> Option<&u32> {
        self.buffers_written.as_ref()
    }

    /// Sets the value of CPUSpeed
    pub fn set_cpuspeed(&mut self, value: u32) {
        self.cpuspeed = Some(value);
    }

    /// Gets the value of CPUSpeed
    pub fn get_cpuspeed(&self) -> Option<&u32> {
        self.cpuspeed.as_ref()
    }

    /// Sets the value of EndTime
    pub fn set_end_time(&mut self, value: u64) {
        self.end_time = Some(value);
    }

    /// Gets the value of EndTime
    pub fn get_end_time(&self) -> Option<&u64> {
        self.end_time.as_ref()
    }

    /// Sets the value of EventsLost
    pub fn set_events_lost(&mut self, value: u32) {
        self.events_lost = Some(value);
    }

    /// Gets the value of EventsLost
    pub fn get_events_lost(&self) -> Option<&u32> {
        self.events_lost.as_ref()
    }

    /// Sets the value of LogFileMode
    pub fn set_log_file_mode(&mut self, value: u32) {
        self.log_file_mode = Some(value);
    }

    /// Gets the value of LogFileMode
    pub fn get_log_file_mode(&self) -> Option<&u32> {
        self.log_file_mode.as_ref()
    }

    /// Sets the value of LogFileName
    pub fn set_log_file_name(&mut self, value: u32) {
        self.log_file_name = Some(value);
    }

    /// Gets the value of LogFileName
    pub fn get_log_file_name(&self) -> Option<&u32> {
        self.log_file_name.as_ref()
    }

    /// Sets the value of LogFileNameString
    pub fn set_log_file_name_string(&mut self, value: String) {
        self.log_file_name_string = Some(value);
    }

    /// Gets the value of LogFileNameString
    pub fn get_log_file_name_string(&self) -> Option<&String> {
        self.log_file_name_string.as_ref()
    }

    /// Sets the value of LoggerName
    pub fn set_logger_name(&mut self, value: u32) {
        self.logger_name = Some(value);
    }

    /// Gets the value of LoggerName
    pub fn get_logger_name(&self) -> Option<&u32> {
        self.logger_name.as_ref()
    }

    /// Sets the value of MaxFileSize
    pub fn set_max_file_size(&mut self, value: u32) {
        self.max_file_size = Some(value);
    }

    /// Gets the value of MaxFileSize
    pub fn get_max_file_size(&self) -> Option<&u32> {
        self.max_file_size.as_ref()
    }

    /// Sets the value of NumberOfProcessors
    pub fn set_number_of_processors(&mut self, value: u32) {
        self.number_of_processors = Some(value);
    }

    /// Gets the value of NumberOfProcessors
    pub fn get_number_of_processors(&self) -> Option<&u32> {
        self.number_of_processors.as_ref()
    }

    /// Sets the value of PerfFreq
    pub fn set_perf_freq(&mut self, value: u64) {
        self.perf_freq = Some(value);
    }

    /// Gets the value of PerfFreq
    pub fn get_perf_freq(&self) -> Option<&u64> {
        self.perf_freq.as_ref()
    }

    /// Sets the value of PointerSize
    pub fn set_pointer_size(&mut self, value: u32) {
        self.pointer_size = Some(value);
    }

    /// Gets the value of PointerSize
    pub fn get_pointer_size(&self) -> Option<&u32> {
        self.pointer_size.as_ref()
    }

    /// Sets the value of ProviderVersion
    pub fn set_provider_version(&mut self, value: u32) {
        self.provider_version = Some(value);
    }

    /// Gets the value of ProviderVersion
    pub fn get_provider_version(&self) -> Option<&u32> {
        self.provider_version.as_ref()
    }

    /// Sets the value of ReservedFlags
    pub fn set_reserved_flags(&mut self, value: u32) {
        self.reserved_flags = Some(value);
    }

    /// Gets the value of ReservedFlags
    pub fn get_reserved_flags(&self) -> Option<&u32> {
        self.reserved_flags.as_ref()
    }

    /// Sets the value of SessionNameString
    pub fn set_session_name_string(&mut self, value: String) {
        self.session_name_string = Some(value);
    }

    /// Gets the value of SessionNameString
    pub fn get_session_name_string(&self) -> Option<&String> {
        self.session_name_string.as_ref()
    }

    /// Sets the value of StartBuffers
    pub fn set_start_buffers(&mut self, value: u32) {
        self.start_buffers = Some(value);
    }

    /// Gets the value of StartBuffers
    pub fn get_start_buffers(&self) -> Option<&u32> {
        self.start_buffers.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: u64) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&u64> {
        self.start_time.as_ref()
    }

    /// Sets the value of TimerResolution
    pub fn set_timer_resolution(&mut self, value: u32) {
        self.timer_resolution = Some(value);
    }

    /// Gets the value of TimerResolution
    pub fn get_timer_resolution(&self) -> Option<&u32> {
        self.timer_resolution.as_ref()
    }

    /// Sets the value of TimeZoneInformation
    pub fn set_time_zone_information(&mut self, value: Vec<u8>) {
        self.time_zone_information = value;
    }

    /// Gets the value of TimeZoneInformation
    pub fn get_time_zone_information(&self) -> &Vec<u8> {
        &self.time_zone_information
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: u32) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&u32> {
        self.version.as_ref()
    }
}

