// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CoolingDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CoolingDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "ActiveCooling")]
    pub active_cooling: Option<bool>,
}

impl CIM_CoolingDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            active_cooling: None,
        }
    }


    /// Sets the value of ActiveCooling
    pub fn set_active_cooling(&mut self, value: bool) {
        self.active_cooling = Some(value);
    }

    /// Gets the value of ActiveCooling
    pub fn get_active_cooling(&self) -> Option<&bool> {
        self.active_cooling.as_ref()
    }
}

