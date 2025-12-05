// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_1394Controller struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_1394Controller {
    #[serde(flatten)]
    pub base: CIM_Controller,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,
}

impl Win32_1394Controller {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Controller::new(),
            manufacturer: None,
        }
    }


    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }
}

