// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogRecord struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogRecord {
    #[serde(flatten)]
    pub base: CIM_RecordForLog,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "DataFormat")]
    pub data_format: Option<String>,

/// 
    #[serde(rename = "LogCreationClassName")]
    pub log_creation_class_name: Option<String>,

/// 
    #[serde(rename = "LogName")]
    pub log_name: Option<String>,

/// 
    #[serde(rename = "MessageTimestamp")]
    pub message_timestamp: Option<String>,

/// 
    #[serde(rename = "RecordID")]
    pub record_id: Option<String>,
}

impl CIM_LogRecord {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RecordForLog::new(),
            creation_class_name: None,
            data_format: None,
            log_creation_class_name: None,
            log_name: None,
            message_timestamp: None,
            record_id: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of DataFormat
    pub fn set_data_format(&mut self, value: String) {
        self.data_format = Some(value);
    }

    /// Gets the value of DataFormat
    pub fn get_data_format(&self) -> Option<&String> {
        self.data_format.as_ref()
    }

    /// Sets the value of LogCreationClassName
    pub fn set_log_creation_class_name(&mut self, value: String) {
        self.log_creation_class_name = Some(value);
    }

    /// Gets the value of LogCreationClassName
    pub fn get_log_creation_class_name(&self) -> Option<&String> {
        self.log_creation_class_name.as_ref()
    }

    /// Sets the value of LogName
    pub fn set_log_name(&mut self, value: String) {
        self.log_name = Some(value);
    }

    /// Gets the value of LogName
    pub fn get_log_name(&self) -> Option<&String> {
        self.log_name.as_ref()
    }

    /// Sets the value of MessageTimestamp
    pub fn set_message_timestamp(&mut self, value: String) {
        self.message_timestamp = Some(value);
    }

    /// Gets the value of MessageTimestamp
    pub fn get_message_timestamp(&self) -> Option<&String> {
        self.message_timestamp.as_ref()
    }

    /// Sets the value of RecordID
    pub fn set_record_id(&mut self, value: String) {
        self.record_id = Some(value);
    }

    /// Gets the value of RecordID
    pub fn get_record_id(&self) -> Option<&String> {
        self.record_id.as_ref()
    }
}

