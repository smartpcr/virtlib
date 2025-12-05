// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ODBCDataSourceSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ODBCDataSourceSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "DataSource")]
    pub data_source: Option<String>,

/// 
    #[serde(rename = "DriverDescription")]
    pub driver_description: Option<String>,

/// 
    #[serde(rename = "Registration")]
    pub registration: Option<String>,
}

impl Win32_ODBCDataSourceSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            data_source: None,
            driver_description: None,
            registration: None,
        }
    }


    /// Sets the value of DataSource
    pub fn set_data_source(&mut self, value: String) {
        self.data_source = Some(value);
    }

    /// Gets the value of DataSource
    pub fn get_data_source(&self) -> Option<&String> {
        self.data_source.as_ref()
    }

    /// Sets the value of DriverDescription
    pub fn set_driver_description(&mut self, value: String) {
        self.driver_description = Some(value);
    }

    /// Gets the value of DriverDescription
    pub fn get_driver_description(&self) -> Option<&String> {
        self.driver_description.as_ref()
    }

    /// Sets the value of Registration
    pub fn set_registration(&mut self, value: String) {
        self.registration = Some(value);
    }

    /// Gets the value of Registration
    pub fn get_registration(&self) -> Option<&String> {
        self.registration.as_ref()
    }
}

