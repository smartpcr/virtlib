// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PageFileSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PageFileSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "InitialSize")]
    pub initial_size: Option<u32>,

/// 
    #[serde(rename = "MaximumSize")]
    pub maximum_size: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl Win32_PageFileSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            initial_size: None,
            maximum_size: None,
            name: None,
        }
    }


    /// Sets the value of InitialSize
    pub fn set_initial_size(&mut self, value: u32) {
        self.initial_size = Some(value);
    }

    /// Gets the value of InitialSize
    pub fn get_initial_size(&self) -> Option<&u32> {
        self.initial_size.as_ref()
    }

    /// Sets the value of MaximumSize
    pub fn set_maximum_size(&mut self, value: u32) {
        self.maximum_size = Some(value);
    }

    /// Gets the value of MaximumSize
    pub fn get_maximum_size(&self) -> Option<&u32> {
        self.maximum_size.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

