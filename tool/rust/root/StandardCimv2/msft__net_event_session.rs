// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetEventSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetEventSession {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "CaptureMode")]
    pub capture_mode: Option<u8>,

/// 
    #[serde(rename = "Guid")]
    pub guid: Option<String>,

/// 
    #[serde(rename = "LocalFilePath")]
    pub local_file_path: Option<String>,

/// 
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<u32>,

/// 
    #[serde(rename = "MaxNumberOfBuffers")]
    pub max_number_of_buffers: Option<u8>,

/// 
    #[serde(rename = "SessionStatus")]
    pub session_status: Option<u8>,

/// 
    #[serde(rename = "TraceBufferSize")]
    pub trace_buffer_size: Option<u32>,
}

impl MSFT_NetEventSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            capture_mode: None,
            guid: None,
            local_file_path: None,
            max_file_size: None,
            max_number_of_buffers: None,
            session_status: None,
            trace_buffer_size: None,
        }
    }


    /// Sets the value of CaptureMode
    pub fn set_capture_mode(&mut self, value: u8) {
        self.capture_mode = Some(value);
    }

    /// Gets the value of CaptureMode
    pub fn get_capture_mode(&self) -> Option<&u8> {
        self.capture_mode.as_ref()
    }

    /// Sets the value of Guid
    pub fn set_guid(&mut self, value: String) {
        self.guid = Some(value);
    }

    /// Gets the value of Guid
    pub fn get_guid(&self) -> Option<&String> {
        self.guid.as_ref()
    }

    /// Sets the value of LocalFilePath
    pub fn set_local_file_path(&mut self, value: String) {
        self.local_file_path = Some(value);
    }

    /// Gets the value of LocalFilePath
    pub fn get_local_file_path(&self) -> Option<&String> {
        self.local_file_path.as_ref()
    }

    /// Sets the value of MaxFileSize
    pub fn set_max_file_size(&mut self, value: u32) {
        self.max_file_size = Some(value);
    }

    /// Gets the value of MaxFileSize
    pub fn get_max_file_size(&self) -> Option<&u32> {
        self.max_file_size.as_ref()
    }

    /// Sets the value of MaxNumberOfBuffers
    pub fn set_max_number_of_buffers(&mut self, value: u8) {
        self.max_number_of_buffers = Some(value);
    }

    /// Gets the value of MaxNumberOfBuffers
    pub fn get_max_number_of_buffers(&self) -> Option<&u8> {
        self.max_number_of_buffers.as_ref()
    }

    /// Sets the value of SessionStatus
    pub fn set_session_status(&mut self, value: u8) {
        self.session_status = Some(value);
    }

    /// Gets the value of SessionStatus
    pub fn get_session_status(&self) -> Option<&u8> {
        self.session_status.as_ref()
    }

    /// Sets the value of TraceBufferSize
    pub fn set_trace_buffer_size(&mut self, value: u32) {
        self.trace_buffer_size = Some(value);
    }

    /// Gets the value of TraceBufferSize
    pub fn get_trace_buffer_size(&self) -> Option<&u32> {
        self.trace_buffer_size.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn start(&self) -> Result<(), WmiError> {
        self.invoke_method("Start", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn stop(&self) -> Result<(), WmiError> {
        self.invoke_method("Stop", &[])

    }

}

