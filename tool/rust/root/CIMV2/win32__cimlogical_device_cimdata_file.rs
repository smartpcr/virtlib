// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CIMLogicalDeviceCIMDataFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CIMLogicalDeviceCIMDataFile {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// 
    #[serde(rename = "Purpose")]
    pub purpose: Option<u16>,

/// 
    #[serde(rename = "PurposeDescription")]
    pub purpose_description: Option<String>,
}

impl Win32_CIMLogicalDeviceCIMDataFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            purpose: None,
            purpose_description: None,
        }
    }


    /// Sets the value of Purpose
    pub fn set_purpose(&mut self, value: u16) {
        self.purpose = Some(value);
    }

    /// Gets the value of Purpose
    pub fn get_purpose(&self) -> Option<&u16> {
        self.purpose.as_ref()
    }

    /// Sets the value of PurposeDescription
    pub fn set_purpose_description(&mut self, value: String) {
        self.purpose_description = Some(value);
    }

    /// Gets the value of PurposeDescription
    pub fn get_purpose_description(&self) -> Option<&String> {
        self.purpose_description.as_ref()
    }
}

