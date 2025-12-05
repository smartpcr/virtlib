// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.EventTracingManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_EtwTraceSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_EtwTraceSession {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "BufferSize")]
    pub buffer_size: Option<u32>,

/// 
    #[serde(rename = "ClockType")]
    pub clock_type: Option<u32>,

/// 
    #[serde(rename = "FlushTimer")]
    pub flush_timer: Option<u32>,

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
}

impl MSFT_EtwTraceSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            buffer_size: None,
            clock_type: None,
            flush_timer: None,
            local_file_path: None,
            log_file_mode: None,
            maximum_buffers: None,
            maximum_file_size: None,
            minimum_buffers: None,
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

    /// Sets the value of FlushTimer
    pub fn set_flush_timer(&mut self, value: u32) {
        self.flush_timer = Some(value);
    }

    /// Gets the value of FlushTimer
    pub fn get_flush_timer(&self) -> Option<&u32> {
        self.flush_timer.as_ref()
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

/// 

    /// * `delete_file` -  (bool)
    /// * `destination_folder` -  (String)

    /// * `error_code` -  (u32)
    /// * `return_value` -  (u32)
    /// * `source_file_path` -  (String)
    pub fn send(&self, destination_folder: &String, delete_file: bool, source_file_path: &mut String, error_code: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DestinationFolder".to_string(), value: destination_folder.into() });
        args.push(MethodParameter { name: "DeleteFile".to_string(), value: delete_file.into() });

        let result = self.invoke_method("Send", &args)?;
        let error_code = result.get_value("ErrorCode")?;
        let source_file_path = result.get_value("SourceFilePath")?;
        Ok(result.return_value)

    }

}

