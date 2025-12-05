// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NTLogEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NTLogEvent {

/// 
    #[serde(rename = "Category")]
    pub category: Option<u16>,

/// 
    #[serde(rename = "CategoryString")]
    pub category_string: Option<String>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "Data")]
    pub data: Vec<u8>,

/// 
    #[serde(rename = "EventCode")]
    pub event_code: Option<u16>,

/// 
    #[serde(rename = "EventIdentifier")]
    pub event_identifier: Option<u32>,

/// 
    #[serde(rename = "EventType")]
    pub event_type: Option<u8>,

/// 
    #[serde(rename = "InsertionStrings")]
    pub insertion_strings: Vec<String>,

/// 
    #[serde(rename = "Logfile")]
    pub logfile: Option<String>,

/// 
    #[serde(rename = "Message")]
    pub message: Option<String>,

/// 
    #[serde(rename = "RecordNumber")]
    pub record_number: Option<u32>,

/// 
    #[serde(rename = "SourceName")]
    pub source_name: Option<String>,

/// 
    #[serde(rename = "TimeGenerated")]
    pub time_generated: Option<String>,

/// 
    #[serde(rename = "TimeWritten")]
    pub time_written: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,

/// 
    #[serde(rename = "User")]
    pub user: Option<String>,
}

impl Win32_NTLogEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            category: None,
            category_string: None,
            computer_name: None,
            data: Vec::new(),
            event_code: None,
            event_identifier: None,
            event_type: None,
            insertion_strings: Vec::new(),
            logfile: None,
            message: None,
            record_number: None,
            source_name: None,
            time_generated: None,
            time_written: None,
            type: None,
            user: None,
        }
    }


    /// Sets the value of Category
    pub fn set_category(&mut self, value: u16) {
        self.category = Some(value);
    }

    /// Gets the value of Category
    pub fn get_category(&self) -> Option<&u16> {
        self.category.as_ref()
    }

    /// Sets the value of CategoryString
    pub fn set_category_string(&mut self, value: String) {
        self.category_string = Some(value);
    }

    /// Gets the value of CategoryString
    pub fn get_category_string(&self) -> Option<&String> {
        self.category_string.as_ref()
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of Data
    pub fn set_data(&mut self, value: Vec<u8>) {
        self.data = value;
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Sets the value of EventCode
    pub fn set_event_code(&mut self, value: u16) {
        self.event_code = Some(value);
    }

    /// Gets the value of EventCode
    pub fn get_event_code(&self) -> Option<&u16> {
        self.event_code.as_ref()
    }

    /// Sets the value of EventIdentifier
    pub fn set_event_identifier(&mut self, value: u32) {
        self.event_identifier = Some(value);
    }

    /// Gets the value of EventIdentifier
    pub fn get_event_identifier(&self) -> Option<&u32> {
        self.event_identifier.as_ref()
    }

    /// Sets the value of EventType
    pub fn set_event_type(&mut self, value: u8) {
        self.event_type = Some(value);
    }

    /// Gets the value of EventType
    pub fn get_event_type(&self) -> Option<&u8> {
        self.event_type.as_ref()
    }

    /// Sets the value of InsertionStrings
    pub fn set_insertion_strings(&mut self, value: Vec<String>) {
        self.insertion_strings = value;
    }

    /// Gets the value of InsertionStrings
    pub fn get_insertion_strings(&self) -> &Vec<String> {
        &self.insertion_strings
    }

    /// Sets the value of Logfile
    pub fn set_logfile(&mut self, value: String) {
        self.logfile = Some(value);
    }

    /// Gets the value of Logfile
    pub fn get_logfile(&self) -> Option<&String> {
        self.logfile.as_ref()
    }

    /// Sets the value of Message
    pub fn set_message(&mut self, value: String) {
        self.message = Some(value);
    }

    /// Gets the value of Message
    pub fn get_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets the value of RecordNumber
    pub fn set_record_number(&mut self, value: u32) {
        self.record_number = Some(value);
    }

    /// Gets the value of RecordNumber
    pub fn get_record_number(&self) -> Option<&u32> {
        self.record_number.as_ref()
    }

    /// Sets the value of SourceName
    pub fn set_source_name(&mut self, value: String) {
        self.source_name = Some(value);
    }

    /// Gets the value of SourceName
    pub fn get_source_name(&self) -> Option<&String> {
        self.source_name.as_ref()
    }

    /// Sets the value of TimeGenerated
    pub fn set_time_generated(&mut self, value: String) {
        self.time_generated = Some(value);
    }

    /// Gets the value of TimeGenerated
    pub fn get_time_generated(&self) -> Option<&String> {
        self.time_generated.as_ref()
    }

    /// Sets the value of TimeWritten
    pub fn set_time_written(&mut self, value: String) {
        self.time_written = Some(value);
    }

    /// Gets the value of TimeWritten
    pub fn get_time_written(&self) -> Option<&String> {
        self.time_written.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }

    /// Sets the value of User
    pub fn set_user(&mut self, value: String) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }
}

