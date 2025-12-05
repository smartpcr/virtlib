// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.DEFAULT
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// LogFileEventConsumer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogFileEventConsumer {
    #[serde(flatten)]
    pub base: __EventConsumer,

/// 
    #[serde(rename = "Filename")]
    pub filename: Option<String>,

/// 
    #[serde(rename = "IsUnicode")]
    pub is_unicode: Option<bool>,

/// 
    #[serde(rename = "MaximumFileSize")]
    pub maximum_file_size: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Text")]
    pub text: Option<String>,
}

impl LogFileEventConsumer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __EventConsumer::new(),
            filename: None,
            is_unicode: None,
            maximum_file_size: None,
            name: None,
            text: None,
        }
    }


    /// Sets the value of Filename
    pub fn set_filename(&mut self, value: String) {
        self.filename = Some(value);
    }

    /// Gets the value of Filename
    pub fn get_filename(&self) -> Option<&String> {
        self.filename.as_ref()
    }

    /// Sets the value of IsUnicode
    pub fn set_is_unicode(&mut self, value: bool) {
        self.is_unicode = Some(value);
    }

    /// Gets the value of IsUnicode
    pub fn get_is_unicode(&self) -> Option<&bool> {
        self.is_unicode.as_ref()
    }

    /// Sets the value of MaximumFileSize
    pub fn set_maximum_file_size(&mut self, value: u64) {
        self.maximum_file_size = Some(value);
    }

    /// Gets the value of MaximumFileSize
    pub fn get_maximum_file_size(&self) -> Option<&u64> {
        self.maximum_file_size.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Text
    pub fn set_text(&mut self, value: String) {
        self.text = Some(value);
    }

    /// Gets the value of Text
    pub fn get_text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}

