// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_Job struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_Job {

/// 
    #[serde(rename = "BytesProcessed")]
    pub bytes_processed: Option<u64>,

/// 
    #[serde(rename = "BytesTotal")]
    pub bytes_total: Option<u64>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "ElapsedTime")]
    pub elapsed_time: Option<String>,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u16>,

/// 
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u32>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u16>,
}

impl SDDC_Job {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bytes_processed: None,
            bytes_total: None,
            description: None,
            elapsed_time: None,
            error_code: None,
            error_description: None,
            id: None,
            percent_complete: None,
            start_time: None,
            state: None,
        }
    }


    /// Sets the value of BytesProcessed
    pub fn set_bytes_processed(&mut self, value: u64) {
        self.bytes_processed = Some(value);
    }

    /// Gets the value of BytesProcessed
    pub fn get_bytes_processed(&self) -> Option<&u64> {
        self.bytes_processed.as_ref()
    }

    /// Sets the value of BytesTotal
    pub fn set_bytes_total(&mut self, value: u64) {
        self.bytes_total = Some(value);
    }

    /// Gets the value of BytesTotal
    pub fn get_bytes_total(&self) -> Option<&u64> {
        self.bytes_total.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ElapsedTime
    pub fn set_elapsed_time(&mut self, value: String) {
        self.elapsed_time = Some(value);
    }

    /// Gets the value of ElapsedTime
    pub fn get_elapsed_time(&self) -> Option<&String> {
        self.elapsed_time.as_ref()
    }

    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: u16) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&u16> {
        self.error_code.as_ref()
    }

    /// Sets the value of ErrorDescription
    pub fn set_error_description(&mut self, value: String) {
        self.error_description = Some(value);
    }

    /// Gets the value of ErrorDescription
    pub fn get_error_description(&self) -> Option<&String> {
        self.error_description.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: u32) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&u32> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: String) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u16) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u16> {
        self.state.as_ref()
    }
}

