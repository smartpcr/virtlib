// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ODBCDriverSpecification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ODBCDriverSpecification {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

/// 
    #[serde(rename = "File")]
    pub file: Option<String>,

/// 
    #[serde(rename = "SetupFile")]
    pub setup_file: Option<String>,
}

impl Win32_ODBCDriverSpecification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            driver: None,
            file: None,
            setup_file: None,
        }
    }


    /// Sets the value of Driver
    pub fn set_driver(&mut self, value: String) {
        self.driver = Some(value);
    }

    /// Gets the value of Driver
    pub fn get_driver(&self) -> Option<&String> {
        self.driver.as_ref()
    }

    /// Sets the value of File
    pub fn set_file(&mut self, value: String) {
        self.file = Some(value);
    }

    /// Gets the value of File
    pub fn get_file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    /// Sets the value of SetupFile
    pub fn set_setup_file(&mut self, value: String) {
        self.setup_file = Some(value);
    }

    /// Gets the value of SetupFile
    pub fn get_setup_file(&self) -> Option<&String> {
        self.setup_file.as_ref()
    }
}

