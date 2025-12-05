// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LUIDandAttributes struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LUIDandAttributes {

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Option<u32>,

/// 
    #[serde(rename = "LUID")]
    pub luid: Option<Win32_LUID>,
}

impl Win32_LUIDandAttributes {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            attributes: None,
            luid: None,
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

    /// Sets the value of LUID
    pub fn set_luid(&mut self, value: Win32_LUID) {
        self.luid = Some(value);
    }

    /// Gets the value of LUID
    pub fn get_luid(&self) -> Option<&Win32_LUID> {
        self.luid.as_ref()
    }
}

