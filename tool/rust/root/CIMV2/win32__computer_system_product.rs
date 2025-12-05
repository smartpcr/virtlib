// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ComputerSystemProduct struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ComputerSystemProduct {
    #[serde(flatten)]
    pub base: CIM_Product,

/// 
    #[serde(rename = "UUID")]
    pub uuid: Option<String>,
}

impl Win32_ComputerSystemProduct {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Product::new(),
            uuid: None,
        }
    }


    /// Sets the value of UUID
    pub fn set_uuid(&mut self, value: String) {
        self.uuid = Some(value);
    }

    /// Gets the value of UUID
    pub fn get_uuid(&self) -> Option<&String> {
        self.uuid.as_ref()
    }
}

