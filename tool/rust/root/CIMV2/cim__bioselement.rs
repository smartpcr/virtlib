// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BIOSElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BIOSElement {
    #[serde(flatten)]
    pub base: CIM_SoftwareElement,

/// 
    #[serde(rename = "PrimaryBIOS")]
    pub primary_bios: Option<bool>,
}

impl CIM_BIOSElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareElement::new(),
            primary_bios: None,
        }
    }


    /// Sets the value of PrimaryBIOS
    pub fn set_primary_bios(&mut self, value: bool) {
        self.primary_bios = Some(value);
    }

    /// Gets the value of PrimaryBIOS
    pub fn get_primary_bios(&self) -> Option<&bool> {
        self.primary_bios.as_ref()
    }
}

