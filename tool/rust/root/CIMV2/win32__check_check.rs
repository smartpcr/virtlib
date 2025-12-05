// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CheckCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CheckCheck {

/// 
    #[serde(rename = "Check")]
    pub check: Option<CIM_Check>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<CIM_Check>,
}

impl Win32_CheckCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            check: None,
            location: None,
        }
    }


    /// Sets the value of Check
    pub fn set_check(&mut self, value: CIM_Check) {
        self.check = Some(value);
    }

    /// Gets the value of Check
    pub fn get_check(&self) -> Option<&CIM_Check> {
        self.check.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: CIM_Check) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&CIM_Check> {
        self.location.as_ref()
    }
}

