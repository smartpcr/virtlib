// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSoP_PolicySettingStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSoP_PolicySettingStatus {

/// 
    #[serde(rename = "errorCode")]
    pub error_code: Option<u32>,

/// 
    #[serde(rename = "eventID")]
    pub event_id: Option<u32>,

/// 
    #[serde(rename = "eventLogName")]
    pub event_log_name: Option<String>,

/// 
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,

/// 
    #[serde(rename = "eventTime")]
    pub event_time: Option<String>,

/// 
    #[serde(rename = "id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "status")]
    pub status: Option<i32>,
}

impl RSoP_PolicySettingStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            error_code: None,
            event_id: None,
            event_log_name: None,
            event_source: None,
            event_time: None,
            id: None,
            status: None,
        }
    }


    /// Sets the value of errorCode
    pub fn set_error_code(&mut self, value: u32) {
        self.error_code = Some(value);
    }

    /// Gets the value of errorCode
    pub fn get_error_code(&self) -> Option<&u32> {
        self.error_code.as_ref()
    }

    /// Sets the value of eventID
    pub fn set_event_id(&mut self, value: u32) {
        self.event_id = Some(value);
    }

    /// Gets the value of eventID
    pub fn get_event_id(&self) -> Option<&u32> {
        self.event_id.as_ref()
    }

    /// Sets the value of eventLogName
    pub fn set_event_log_name(&mut self, value: String) {
        self.event_log_name = Some(value);
    }

    /// Gets the value of eventLogName
    pub fn get_event_log_name(&self) -> Option<&String> {
        self.event_log_name.as_ref()
    }

    /// Sets the value of eventSource
    pub fn set_event_source(&mut self, value: String) {
        self.event_source = Some(value);
    }

    /// Gets the value of eventSource
    pub fn get_event_source(&self) -> Option<&String> {
        self.event_source.as_ref()
    }

    /// Sets the value of eventTime
    pub fn set_event_time(&mut self, value: String) {
        self.event_time = Some(value);
    }

    /// Gets the value of eventTime
    pub fn get_event_time(&self) -> Option<&String> {
        self.event_time.as_ref()
    }

    /// Sets the value of id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of status
    pub fn set_status(&mut self, value: i32) {
        self.status = Some(value);
    }

    /// Gets the value of status
    pub fn get_status(&self) -> Option<&i32> {
        self.status.as_ref()
    }
}

