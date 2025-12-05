// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_RemoteAccess_RASTotal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_RemoteAccess_RASTotal {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AlignmentErrors")]
    pub alignment_errors: Option<u32>,

/// 
    #[serde(rename = "BufferOverrunErrors")]
    pub buffer_overrun_errors: Option<u32>,

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Option<u64>,

/// 
    #[serde(rename = "BytesReceivedPerSec")]
    pub bytes_received_per_sec: Option<u32>,

/// 
    #[serde(rename = "BytesTransmitted")]
    pub bytes_transmitted: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedPerSec")]
    pub bytes_transmitted_per_sec: Option<u32>,

/// 
    #[serde(rename = "CRCErrors")]
    pub crcerrors: Option<u32>,

/// 
    #[serde(rename = "FramesReceived")]
    pub frames_received: Option<u32>,

/// 
    #[serde(rename = "FramesReceivedPerSec")]
    pub frames_received_per_sec: Option<u32>,

/// 
    #[serde(rename = "FramesTransmitted")]
    pub frames_transmitted: Option<u32>,

/// 
    #[serde(rename = "FramesTransmittedPerSec")]
    pub frames_transmitted_per_sec: Option<u32>,

/// 
    #[serde(rename = "PercentCompressionIn")]
    pub percent_compression_in: Option<u32>,

/// 
    #[serde(rename = "PercentCompressionOut")]
    pub percent_compression_out: Option<u32>,

/// 
    #[serde(rename = "SerialOverrunErrors")]
    pub serial_overrun_errors: Option<u32>,

/// 
    #[serde(rename = "TimeoutErrors")]
    pub timeout_errors: Option<u32>,

/// 
    #[serde(rename = "TotalConnections")]
    pub total_connections: Option<u32>,

/// 
    #[serde(rename = "TotalErrors")]
    pub total_errors: Option<u32>,

/// 
    #[serde(rename = "TotalErrorsPerSec")]
    pub total_errors_per_sec: Option<u32>,
}

impl Win32_PerfRawData_RemoteAccess_RASTotal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            alignment_errors: None,
            buffer_overrun_errors: None,
            bytes_received: None,
            bytes_received_per_sec: None,
            bytes_transmitted: None,
            bytes_transmitted_per_sec: None,
            crcerrors: None,
            frames_received: None,
            frames_received_per_sec: None,
            frames_transmitted: None,
            frames_transmitted_per_sec: None,
            percent_compression_in: None,
            percent_compression_out: None,
            serial_overrun_errors: None,
            timeout_errors: None,
            total_connections: None,
            total_errors: None,
            total_errors_per_sec: None,
        }
    }


    /// Sets the value of AlignmentErrors
    pub fn set_alignment_errors(&mut self, value: u32) {
        self.alignment_errors = Some(value);
    }

    /// Gets the value of AlignmentErrors
    pub fn get_alignment_errors(&self) -> Option<&u32> {
        self.alignment_errors.as_ref()
    }

    /// Sets the value of BufferOverrunErrors
    pub fn set_buffer_overrun_errors(&mut self, value: u32) {
        self.buffer_overrun_errors = Some(value);
    }

    /// Gets the value of BufferOverrunErrors
    pub fn get_buffer_overrun_errors(&self) -> Option<&u32> {
        self.buffer_overrun_errors.as_ref()
    }

    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: u64) {
        self.bytes_received = Some(value);
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> Option<&u64> {
        self.bytes_received.as_ref()
    }

    /// Sets the value of BytesReceivedPerSec
    pub fn set_bytes_received_per_sec(&mut self, value: u32) {
        self.bytes_received_per_sec = Some(value);
    }

    /// Gets the value of BytesReceivedPerSec
    pub fn get_bytes_received_per_sec(&self) -> Option<&u32> {
        self.bytes_received_per_sec.as_ref()
    }

    /// Sets the value of BytesTransmitted
    pub fn set_bytes_transmitted(&mut self, value: u64) {
        self.bytes_transmitted = Some(value);
    }

    /// Gets the value of BytesTransmitted
    pub fn get_bytes_transmitted(&self) -> Option<&u64> {
        self.bytes_transmitted.as_ref()
    }

    /// Sets the value of BytesTransmittedPerSec
    pub fn set_bytes_transmitted_per_sec(&mut self, value: u32) {
        self.bytes_transmitted_per_sec = Some(value);
    }

    /// Gets the value of BytesTransmittedPerSec
    pub fn get_bytes_transmitted_per_sec(&self) -> Option<&u32> {
        self.bytes_transmitted_per_sec.as_ref()
    }

    /// Sets the value of CRCErrors
    pub fn set_crcerrors(&mut self, value: u32) {
        self.crcerrors = Some(value);
    }

    /// Gets the value of CRCErrors
    pub fn get_crcerrors(&self) -> Option<&u32> {
        self.crcerrors.as_ref()
    }

    /// Sets the value of FramesReceived
    pub fn set_frames_received(&mut self, value: u32) {
        self.frames_received = Some(value);
    }

    /// Gets the value of FramesReceived
    pub fn get_frames_received(&self) -> Option<&u32> {
        self.frames_received.as_ref()
    }

    /// Sets the value of FramesReceivedPerSec
    pub fn set_frames_received_per_sec(&mut self, value: u32) {
        self.frames_received_per_sec = Some(value);
    }

    /// Gets the value of FramesReceivedPerSec
    pub fn get_frames_received_per_sec(&self) -> Option<&u32> {
        self.frames_received_per_sec.as_ref()
    }

    /// Sets the value of FramesTransmitted
    pub fn set_frames_transmitted(&mut self, value: u32) {
        self.frames_transmitted = Some(value);
    }

    /// Gets the value of FramesTransmitted
    pub fn get_frames_transmitted(&self) -> Option<&u32> {
        self.frames_transmitted.as_ref()
    }

    /// Sets the value of FramesTransmittedPerSec
    pub fn set_frames_transmitted_per_sec(&mut self, value: u32) {
        self.frames_transmitted_per_sec = Some(value);
    }

    /// Gets the value of FramesTransmittedPerSec
    pub fn get_frames_transmitted_per_sec(&self) -> Option<&u32> {
        self.frames_transmitted_per_sec.as_ref()
    }

    /// Sets the value of PercentCompressionIn
    pub fn set_percent_compression_in(&mut self, value: u32) {
        self.percent_compression_in = Some(value);
    }

    /// Gets the value of PercentCompressionIn
    pub fn get_percent_compression_in(&self) -> Option<&u32> {
        self.percent_compression_in.as_ref()
    }

    /// Sets the value of PercentCompressionOut
    pub fn set_percent_compression_out(&mut self, value: u32) {
        self.percent_compression_out = Some(value);
    }

    /// Gets the value of PercentCompressionOut
    pub fn get_percent_compression_out(&self) -> Option<&u32> {
        self.percent_compression_out.as_ref()
    }

    /// Sets the value of SerialOverrunErrors
    pub fn set_serial_overrun_errors(&mut self, value: u32) {
        self.serial_overrun_errors = Some(value);
    }

    /// Gets the value of SerialOverrunErrors
    pub fn get_serial_overrun_errors(&self) -> Option<&u32> {
        self.serial_overrun_errors.as_ref()
    }

    /// Sets the value of TimeoutErrors
    pub fn set_timeout_errors(&mut self, value: u32) {
        self.timeout_errors = Some(value);
    }

    /// Gets the value of TimeoutErrors
    pub fn get_timeout_errors(&self) -> Option<&u32> {
        self.timeout_errors.as_ref()
    }

    /// Sets the value of TotalConnections
    pub fn set_total_connections(&mut self, value: u32) {
        self.total_connections = Some(value);
    }

    /// Gets the value of TotalConnections
    pub fn get_total_connections(&self) -> Option<&u32> {
        self.total_connections.as_ref()
    }

    /// Sets the value of TotalErrors
    pub fn set_total_errors(&mut self, value: u32) {
        self.total_errors = Some(value);
    }

    /// Gets the value of TotalErrors
    pub fn get_total_errors(&self) -> Option<&u32> {
        self.total_errors.as_ref()
    }

    /// Sets the value of TotalErrorsPerSec
    pub fn set_total_errors_per_sec(&mut self, value: u32) {
        self.total_errors_per_sec = Some(value);
    }

    /// Gets the value of TotalErrorsPerSec
    pub fn get_total_errors_per_sec(&self) -> Option<&u32> {
        self.total_errors_per_sec.as_ref()
    }
}

