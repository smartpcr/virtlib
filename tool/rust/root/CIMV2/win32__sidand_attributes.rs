// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SIDandAttributes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SIDandAttributes {

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u32>,

/// 
    #[serde(rename = "SID")]
    pub sid: Option<Win32_SID>,
}

impl Win32_SIDandAttributes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attributes: None,
            sid: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: u32) {
        self.attributes = Some(value);
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> Option<&u32> {
        self.attributes.as_ref()
    }

    /// Sets the value of SID
    pub fn set_sid(&mut self, value: Win32_SID) {
        self.sid = Some(value);
    }

    /// Gets the value of SID
    pub fn get_sid(&self) -> Option<&Win32_SID> {
        self.sid.as_ref()
    }
}

