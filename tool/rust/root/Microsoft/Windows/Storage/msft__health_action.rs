// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HealthAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HealthAction {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "ErrorCode")]
    pub error_code: Option<u32>,

/// 
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// 
    #[serde(rename = "MessageParameters")]
    pub message_parameters: Vec<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u16>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

/// 
    #[serde(rename = "ReportingObjectId")]
    pub reporting_object_id: Option<String>,

/// 
    #[serde(rename = "ReportingObjectType")]
    pub reporting_object_type: Option<String>,

/// 
    #[serde(rename = "ReportingObjectUniqueId")]
    pub reporting_object_unique_id: Option<String>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u16>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl MSFT_HealthAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
            description: None,
            error_code: None,
            error_description: None,
            message_parameters: Vec::new(),
            percent_complete: None,
            reason: None,
            reporting_object_id: None,
            reporting_object_type: None,
            reporting_object_unique_id: None,
            start_time: None,
            state: None,
            status: None,
            type: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of ErrorCode
    pub fn set_error_code(&mut self, value: u32) {
        self.error_code = Some(value);
    }

    /// Gets the value of ErrorCode
    pub fn get_error_code(&self) -> Option<&u32> {
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

    /// Sets the value of MessageParameters
    pub fn set_message_parameters(&mut self, value: Vec<String>) {
        self.message_parameters = value;
    }

    /// Gets the value of MessageParameters
    pub fn get_message_parameters(&self) -> &Vec<String> {
        &self.message_parameters
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: u16) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&u16> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: String) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    /// Sets the value of ReportingObjectId
    pub fn set_reporting_object_id(&mut self, value: String) {
        self.reporting_object_id = Some(value);
    }

    /// Gets the value of ReportingObjectId
    pub fn get_reporting_object_id(&self) -> Option<&String> {
        self.reporting_object_id.as_ref()
    }

    /// Sets the value of ReportingObjectType
    pub fn set_reporting_object_type(&mut self, value: String) {
        self.reporting_object_type = Some(value);
    }

    /// Gets the value of ReportingObjectType
    pub fn get_reporting_object_type(&self) -> Option<&String> {
        self.reporting_object_type.as_ref()
    }

    /// Sets the value of ReportingObjectUniqueId
    pub fn set_reporting_object_unique_id(&mut self, value: String) {
        self.reporting_object_unique_id = Some(value);
    }

    /// Gets the value of ReportingObjectUniqueId
    pub fn get_reporting_object_unique_id(&self) -> Option<&String> {
        self.reporting_object_unique_id.as_ref()
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

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

