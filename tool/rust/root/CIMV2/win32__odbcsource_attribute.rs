// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ODBCSourceAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ODBCSourceAttribute {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "Attribute")]
    pub attribute: Option<String>,

/// 
    #[serde(rename = "DataSource")]
    pub data_source: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl Win32_ODBCSourceAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            attribute: None,
            data_source: None,
            value: None,
        }
    }


    /// Sets the value of Attribute
    pub fn set_attribute(&mut self, value: String) {
        self.attribute = Some(value);
    }

    /// Gets the value of Attribute
    pub fn get_attribute(&self) -> Option<&String> {
        self.attribute.as_ref()
    }

    /// Sets the value of DataSource
    pub fn set_data_source(&mut self, value: String) {
        self.data_source = Some(value);
    }

    /// Gets the value of DataSource
    pub fn get_data_source(&self) -> Option<&String> {
        self.data_source.as_ref()
    }

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

