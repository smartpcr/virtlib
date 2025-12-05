// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WDMClassesOfDriver struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WDMClassesOfDriver {

/// 
    #[serde(rename = "ClassName")]
    pub class_name: Option<String>,

/// 
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

/// 
    #[serde(rename = "HighDateTime")]
    pub high_date_time: Option<u32>,

/// 
    #[serde(rename = "LowDateTime")]
    pub low_date_time: Option<u32>,
}

impl WDMClassesOfDriver {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            class_name: None,
            driver: None,
            high_date_time: None,
            low_date_time: None,
        }
    }


    /// Sets the value of ClassName
    pub fn set_class_name(&mut self, value: String) {
        self.class_name = Some(value);
    }

    /// Gets the value of ClassName
    pub fn get_class_name(&self) -> Option<&String> {
        self.class_name.as_ref()
    }

    /// Sets the value of Driver
    pub fn set_driver(&mut self, value: String) {
        self.driver = Some(value);
    }

    /// Gets the value of Driver
    pub fn get_driver(&self) -> Option<&String> {
        self.driver.as_ref()
    }

    /// Sets the value of HighDateTime
    pub fn set_high_date_time(&mut self, value: u32) {
        self.high_date_time = Some(value);
    }

    /// Gets the value of HighDateTime
    pub fn get_high_date_time(&self) -> Option<&u32> {
        self.high_date_time.as_ref()
    }

    /// Sets the value of LowDateTime
    pub fn set_low_date_time(&mut self, value: u32) {
        self.low_date_time = Some(value);
    }

    /// Gets the value of LowDateTime
    pub fn get_low_date_time(&self) -> Option<&u32> {
        self.low_date_time.as_ref()
    }
}

