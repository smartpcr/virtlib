// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_LoadOrderGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_LoadOrderGroup {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "DriverEnabled")]
    pub driver_enabled: Option<bool>,

/// 
    #[serde(rename = "GroupOrder")]
    pub group_order: Option<u32>,
}

impl Win32_LoadOrderGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            driver_enabled: None,
            group_order: None,
        }
    }


    /// Sets the value of DriverEnabled
    pub fn set_driver_enabled(&mut self, value: bool) {
        self.driver_enabled = Some(value);
    }

    /// Gets the value of DriverEnabled
    pub fn get_driver_enabled(&self) -> Option<&bool> {
        self.driver_enabled.as_ref()
    }

    /// Sets the value of GroupOrder
    pub fn set_group_order(&mut self, value: u32) {
        self.group_order = Some(value);
    }

    /// Gets the value of GroupOrder
    pub fn get_group_order(&self) -> Option<&u32> {
        self.group_order.as_ref()
    }
}

