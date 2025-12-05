// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Error struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Error {

/// 
    #[serde(rename = "CIMStatusCode")]
    pub cimstatus_code: Option<u32>,

/// 
    #[serde(rename = "CIMStatusCodeDescription")]
    pub cimstatus_code_description: Option<String>,

/// 
    #[serde(rename = "ErrorSource")]
    pub error_source: Option<String>,

/// 
    #[serde(rename = "ErrorSourceFormat")]
    pub error_source_format: Option<u16>,

/// 
    #[serde(rename = "ErrorType")]
    pub error_type: Option<u16>,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,

/// 
    #[serde(rename = "MessageArguments")]
    pub message_arguments: Vec<String>,

/// 
    #[serde(rename = "MessageID")]
    pub message_id: Option<String>,

/// 
    #[serde(rename = "OtherErrorSourceFormat")]
    pub other_error_source_format: Option<String>,

/// 
    #[serde(rename = "OtherErrorType")]
    pub other_error_type: Option<String>,

/// 
    #[serde(rename = "OwningEntity")]
    pub owning_entity: Option<String>,

/// 
    #[serde(rename = "PerceivedSeverity")]
    pub perceived_severity: Option<u16>,

/// 
    #[serde(rename = "ProbableCause")]
    pub probable_cause: Option<u16>,

/// 
    #[serde(rename = "ProbableCauseDescription")]
    pub probable_cause_description: Option<String>,

/// 
    #[serde(rename = "RecommendedActions")]
    pub recommended_actions: Vec<String>,
}

impl CIM_Error {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cimstatus_code: None,
            cimstatus_code_description: None,
            error_source: None,
            error_source_format: None,
            error_type: None,
            message: None,
            message_arguments: Vec::new(),
            message_id: None,
            other_error_source_format: None,
            other_error_type: None,
            owning_entity: None,
            perceived_severity: None,
            probable_cause: None,
            probable_cause_description: None,
            recommended_actions: Vec::new(),
        }
    }


    /// Sets the value of CIMStatusCode
    pub fn set_cimstatus_code(&mut self, value: u32) {
        self.cimstatus_code = Some(value);
    }

    /// Gets the value of CIMStatusCode
    pub fn get_cimstatus_code(&self) -> Option<&u32> {
        self.cimstatus_code.as_ref()
    }

    /// Sets the value of CIMStatusCodeDescription
    pub fn set_cimstatus_code_description(&mut self, value: String) {
        self.cimstatus_code_description = Some(value);
    }

    /// Gets the value of CIMStatusCodeDescription
    pub fn get_cimstatus_code_description(&self) -> Option<&String> {
        self.cimstatus_code_description.as_ref()
    }

    /// Sets the value of ErrorSource
    pub fn set_error_source(&mut self, value: String) {
        self.error_source = Some(value);
    }

    /// Gets the value of ErrorSource
    pub fn get_error_source(&self) -> Option<&String> {
        self.error_source.as_ref()
    }

    /// Sets the value of ErrorSourceFormat
    pub fn set_error_source_format(&mut self, value: u16) {
        self.error_source_format = Some(value);
    }

    /// Gets the value of ErrorSourceFormat
    pub fn get_error_source_format(&self) -> Option<&u16> {
        self.error_source_format.as_ref()
    }

    /// Sets the value of ErrorType
    pub fn set_error_type(&mut self, value: u16) {
        self.error_type = Some(value);
    }

    /// Gets the value of ErrorType
    pub fn get_error_type(&self) -> Option<&u16> {
        self.error_type.as_ref()
    }

    /// Sets the value of Message
    pub fn set_message(&mut self, value: String) {
        self.message = Some(value);
    }

    /// Gets the value of Message
    pub fn get_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets the value of MessageArguments
    pub fn set_message_arguments(&mut self, value: Vec<String>) {
        self.message_arguments = value;
    }

    /// Gets the value of MessageArguments
    pub fn get_message_arguments(&self) -> &Vec<String> {
        &self.message_arguments
    }

    /// Sets the value of MessageID
    pub fn set_message_id(&mut self, value: String) {
        self.message_id = Some(value);
    }

    /// Gets the value of MessageID
    pub fn get_message_id(&self) -> Option<&String> {
        self.message_id.as_ref()
    }

    /// Sets the value of OtherErrorSourceFormat
    pub fn set_other_error_source_format(&mut self, value: String) {
        self.other_error_source_format = Some(value);
    }

    /// Gets the value of OtherErrorSourceFormat
    pub fn get_other_error_source_format(&self) -> Option<&String> {
        self.other_error_source_format.as_ref()
    }

    /// Sets the value of OtherErrorType
    pub fn set_other_error_type(&mut self, value: String) {
        self.other_error_type = Some(value);
    }

    /// Gets the value of OtherErrorType
    pub fn get_other_error_type(&self) -> Option<&String> {
        self.other_error_type.as_ref()
    }

    /// Sets the value of OwningEntity
    pub fn set_owning_entity(&mut self, value: String) {
        self.owning_entity = Some(value);
    }

    /// Gets the value of OwningEntity
    pub fn get_owning_entity(&self) -> Option<&String> {
        self.owning_entity.as_ref()
    }

    /// Sets the value of PerceivedSeverity
    pub fn set_perceived_severity(&mut self, value: u16) {
        self.perceived_severity = Some(value);
    }

    /// Gets the value of PerceivedSeverity
    pub fn get_perceived_severity(&self) -> Option<&u16> {
        self.perceived_severity.as_ref()
    }

    /// Sets the value of ProbableCause
    pub fn set_probable_cause(&mut self, value: u16) {
        self.probable_cause = Some(value);
    }

    /// Gets the value of ProbableCause
    pub fn get_probable_cause(&self) -> Option<&u16> {
        self.probable_cause.as_ref()
    }

    /// Sets the value of ProbableCauseDescription
    pub fn set_probable_cause_description(&mut self, value: String) {
        self.probable_cause_description = Some(value);
    }

    /// Gets the value of ProbableCauseDescription
    pub fn get_probable_cause_description(&self) -> Option<&String> {
        self.probable_cause_description.as_ref()
    }

    /// Sets the value of RecommendedActions
    pub fn set_recommended_actions(&mut self, value: Vec<String>) {
        self.recommended_actions = value;
    }

    /// Gets the value of RecommendedActions
    pub fn get_recommended_actions(&self) -> &Vec<String> {
        &self.recommended_actions
    }
}

