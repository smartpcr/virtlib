// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SoftwareFeatureCheck struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SoftwareFeatureCheck {

/// 
    #[serde(rename = "Check")]
    pub check: Option<CIM_Check>,

/// 
    #[serde(rename = "Element")]
    pub element: Option<Win32_SoftwareFeature>,
}

impl Win32_SoftwareFeatureCheck {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            check: None,
            element: None,
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

    /// Sets the value of Element
    pub fn set_element(&mut self, value: Win32_SoftwareFeature) {
        self.element = Some(value);
    }

    /// Gets the value of Element
    pub fn get_element(&self) -> Option<&Win32_SoftwareFeature> {
        self.element.as_ref()
    }
}

