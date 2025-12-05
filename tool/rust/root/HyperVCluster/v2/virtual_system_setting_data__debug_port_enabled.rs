// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_DebugPortEnabled
//////////////////////////////////////////////

/// VirtualSystemSettingData_DebugPortEnabled enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_DebugPortEnabled {
    /// _Off
    #[serde(rename = "_Off")]
    Off = 0,
    /// On
    #[serde(rename = "On")]
    On = 1,
    /// OnAutoAssigned
    #[serde(rename = "OnAutoAssigned")]
    OnAutoAssigned = 2,
}

impl Default for VirtualSystemSettingData_DebugPortEnabled {
    fn default() -> Self {
        Self::Off
    }
}

