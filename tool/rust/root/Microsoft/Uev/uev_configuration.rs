// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// UevConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UevConfiguration {
}

impl UevConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// Enable UEV.
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// Disable UEV.
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// Check if UEV is enabled.

    /// * `return_value` -  (bool)
    pub fn is_enabled(&self) -> Result<(), WmiError> {
        self.invoke_method("IsEnabled", &[])

    }


/// Check if UEV is in reboot required state.

    /// * `return_value` -  (bool)
    pub fn is_reboot_required(&self) -> Result<(), WmiError> {
        self.invoke_method("IsRebootRequired", &[])

    }

}

