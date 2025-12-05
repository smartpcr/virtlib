// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source GuestServiceInterfaceComponentSettingData_EnabledState
//////////////////////////////////////////////

/// GuestServiceInterfaceComponentSettingData_EnabledState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum GuestServiceInterfaceComponentSettingData_EnabledState {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 2,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 3,
}

impl Default for GuestServiceInterfaceComponentSettingData_EnabledState {
    fn default() -> Self {
        Self::Enabled
    }
}

