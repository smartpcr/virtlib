// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_UserDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_UserDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// An indication of whether the Device is locked, preventing user input or output.
    #[serde(rename = "IsLocked")]
    pub is_locked: Option<bool>,
}

impl CIM_UserDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            is_locked: None,
        }
    }


    /// Sets the value of IsLocked
    pub fn set_is_locked(&mut self, value: bool) {
        self.is_locked = Some(value);
    }

    /// Gets the value of IsLocked
    pub fn get_is_locked(&self) -> Option<&bool> {
        self.is_locked.as_ref()
    }
}

