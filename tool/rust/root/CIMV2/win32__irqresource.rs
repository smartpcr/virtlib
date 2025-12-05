// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_IRQResource struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_IRQResource {
    #[serde(flatten)]
    pub base: CIM_IRQ,

/// 
    #[serde(rename = "Hardware")]
    pub hardware: Option<bool>,

/// 
    #[serde(rename = "Vector")]
    pub vector: Option<u32>,
}

impl Win32_IRQResource {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_IRQ::new(),
            hardware: None,
            vector: None,
        }
    }


    /// Sets the value of Hardware
    pub fn set_hardware(&mut self, value: bool) {
        self.hardware = Some(value);
    }

    /// Gets the value of Hardware
    pub fn get_hardware(&self) -> Option<&bool> {
        self.hardware.as_ref()
    }

    /// Sets the value of Vector
    pub fn set_vector(&mut self, value: u32) {
        self.vector = Some(value);
    }

    /// Gets the value of Vector
    pub fn get_vector(&self) -> Option<&u32> {
        self.vector.as_ref()
    }
}

