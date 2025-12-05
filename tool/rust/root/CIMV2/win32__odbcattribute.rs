// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ODBCAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ODBCAttribute {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "Attribute")]
    pub attribute: Option<String>,

/// 
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

/// 
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl Win32_ODBCAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            attribute: None,
            driver: None,
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

    /// Sets the value of Driver
    pub fn set_driver(&mut self, value: String) {
        self.driver = Some(value);
    }

    /// Gets the value of Driver
    pub fn get_driver(&self) -> Option<&String> {
        self.driver.as_ref()
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

