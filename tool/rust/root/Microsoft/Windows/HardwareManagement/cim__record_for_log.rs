// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RecordForLog struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RecordForLog {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "Locale")]
    pub locale: Option<String>,

/// 
    #[serde(rename = "PerceivedSeverity")]
    pub perceived_severity: Option<u16>,

/// 
    #[serde(rename = "RecordData")]
    pub record_data: Option<String>,

/// 
    #[serde(rename = "RecordFormat")]
    pub record_format: Option<String>,
}

impl CIM_RecordForLog {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            locale: None,
            perceived_severity: None,
            record_data: None,
            record_format: None,
        }
    }


    /// Sets the value of Locale
    pub fn set_locale(&mut self, value: String) {
        self.locale = Some(value);
    }

    /// Gets the value of Locale
    pub fn get_locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    /// Sets the value of PerceivedSeverity
    pub fn set_perceived_severity(&mut self, value: u16) {
        self.perceived_severity = Some(value);
    }

    /// Gets the value of PerceivedSeverity
    pub fn get_perceived_severity(&self) -> Option<&u16> {
        self.perceived_severity.as_ref()
    }

    /// Sets the value of RecordData
    pub fn set_record_data(&mut self, value: String) {
        self.record_data = Some(value);
    }

    /// Gets the value of RecordData
    pub fn get_record_data(&self) -> Option<&String> {
        self.record_data.as_ref()
    }

    /// Sets the value of RecordFormat
    pub fn set_record_format(&mut self, value: String) {
        self.record_format = Some(value);
    }

    /// Gets the value of RecordFormat
    pub fn get_record_format(&self) -> Option<&String> {
        self.record_format.as_ref()
    }
}

