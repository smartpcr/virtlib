// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ParallelPort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ParallelPort {
    #[serde(flatten)]
    pub base: CIM_ParallelController,

/// 
    #[serde(rename = "OSAutoDiscovered")]
    pub osauto_discovered: Option<bool>,
}

impl Win32_ParallelPort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ParallelController::new(),
            osauto_discovered: None,
        }
    }


    /// Sets the value of OSAutoDiscovered
    pub fn set_osauto_discovered(&mut self, value: bool) {
        self.osauto_discovered = Some(value);
    }

    /// Gets the value of OSAutoDiscovered
    pub fn get_osauto_discovered(&self) -> Option<&bool> {
        self.osauto_discovered.as_ref()
    }
}

