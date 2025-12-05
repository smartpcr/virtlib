// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_Chip struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_Chip {
    #[serde(flatten)]
    pub base: CIM_PhysicalComponent,

/// 
    #[serde(rename = "FormFactor")]
    pub form_factor: Option<u16>,
}

impl CIM_Chip {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalComponent::new(),
            form_factor: None,
        }
    }


    /// Sets the value of FormFactor
    pub fn set_form_factor(&mut self, value: u16) {
        self.form_factor = Some(value);
    }

    /// Gets the value of FormFactor
    pub fn get_form_factor(&self) -> Option<&u16> {
        self.form_factor.as_ref()
    }
}

