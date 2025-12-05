// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// XmlDictionaryReaderQuotas struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XmlDictionaryReaderQuotas {

/// The maximum allowed array length.
    #[serde(rename = "MaxArrayLength")]
    pub max_array_length: Option<i32>,

/// The maximum allowed bytes returned per read.
    #[serde(rename = "MaxBytesPerRead")]
    pub max_bytes_per_read: Option<i32>,

/// The maximum nested node depth per read.
    #[serde(rename = "MaxDepth")]
    pub max_depth: Option<i32>,

/// The maximum characters allowed in a table name.
    #[serde(rename = "MaxNameTableCharCount")]
    pub max_name_table_char_count: Option<i32>,

/// The maximum characters allowed in XML element content.
    #[serde(rename = "MaxStringContentLength")]
    pub max_string_content_length: Option<i32>,
}

impl XmlDictionaryReaderQuotas {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            max_array_length: None,
            max_bytes_per_read: None,
            max_depth: None,
            max_name_table_char_count: None,
            max_string_content_length: None,
        }
    }


    /// Sets the value of MaxArrayLength
    pub fn set_max_array_length(&mut self, value: i32) {
        self.max_array_length = Some(value);
    }

    /// Gets the value of MaxArrayLength
    pub fn get_max_array_length(&self) -> Option<&i32> {
        self.max_array_length.as_ref()
    }

    /// Sets the value of MaxBytesPerRead
    pub fn set_max_bytes_per_read(&mut self, value: i32) {
        self.max_bytes_per_read = Some(value);
    }

    /// Gets the value of MaxBytesPerRead
    pub fn get_max_bytes_per_read(&self) -> Option<&i32> {
        self.max_bytes_per_read.as_ref()
    }

    /// Sets the value of MaxDepth
    pub fn set_max_depth(&mut self, value: i32) {
        self.max_depth = Some(value);
    }

    /// Gets the value of MaxDepth
    pub fn get_max_depth(&self) -> Option<&i32> {
        self.max_depth.as_ref()
    }

    /// Sets the value of MaxNameTableCharCount
    pub fn set_max_name_table_char_count(&mut self, value: i32) {
        self.max_name_table_char_count = Some(value);
    }

    /// Gets the value of MaxNameTableCharCount
    pub fn get_max_name_table_char_count(&self) -> Option<&i32> {
        self.max_name_table_char_count.as_ref()
    }

    /// Sets the value of MaxStringContentLength
    pub fn set_max_string_content_length(&mut self, value: i32) {
        self.max_string_content_length = Some(value);
    }

    /// Gets the value of MaxStringContentLength
    pub fn get_max_string_content_length(&self) -> Option<&i32> {
        self.max_string_content_length.as_ref()
    }
}

