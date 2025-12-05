// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ReliabilityRecords struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ReliabilityRecords {
    #[serde(flatten)]
    pub base: Win32_Reliability,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "EventIdentifier")]
    pub event_identifier: Option<u32>,

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
    #[serde(rename = "ProductName")]
    pub product_name: Option<String>,

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
    #[serde(rename = "User")]
    pub user: Option<String>,
}

impl Win32_ReliabilityRecords {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_Reliability::new(),
            computer_name: None,
            event_identifier: None,
            insertion_strings: Vec::new(),
            logfile: None,
            message: None,
            product_name: None,
            record_number: None,
            source_name: None,
            time_generated: None,
            user: None,
        }
    }


    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of EventIdentifier
    pub fn set_event_identifier(&mut self, value: u32) {
        self.event_identifier = Some(value);
    }

    /// Gets the value of EventIdentifier
    pub fn get_event_identifier(&self) -> Option<&u32> {
        self.event_identifier.as_ref()
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

    /// Sets the value of ProductName
    pub fn set_product_name(&mut self, value: String) {
        self.product_name = Some(value);
    }

    /// Gets the value of ProductName
    pub fn get_product_name(&self) -> Option<&String> {
        self.product_name.as_ref()
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

    /// Sets the value of User
    pub fn set_user(&mut self, value: String) {
        self.user = Some(value);
    }

    /// Gets the value of User
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }

/// 

    /// * `record_count` -  (u32)
    /// * `return_value` -  (u32)
    pub fn get_record_count(&self, record_count: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetRecordCount", &[])?;
        let record_count = result.get_value("RecordCount")?;
        Ok(result.return_value)

    }

}

