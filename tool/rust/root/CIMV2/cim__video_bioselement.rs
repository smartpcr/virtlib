// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VideoBIOSElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VideoBIOSElement {
    #[serde(flatten)]
    pub base: CIM_SoftwareElement,

/// 
    #[serde(rename = "IsShadowed")]
    pub is_shadowed: Option<bool>,
}

impl CIM_VideoBIOSElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareElement::new(),
            is_shadowed: None,
        }
    }


    /// Sets the value of IsShadowed
    pub fn set_is_shadowed(&mut self, value: bool) {
        self.is_shadowed = Some(value);
    }

    /// Gets the value of IsShadowed
    pub fn get_is_shadowed(&self) -> Option<&bool> {
        self.is_shadowed.as_ref()
    }
}

