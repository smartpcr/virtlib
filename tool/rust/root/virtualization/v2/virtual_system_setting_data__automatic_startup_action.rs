// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_AutomaticStartupAction
//////////////////////////////////////////////

/// VirtualSystemSettingData_AutomaticStartupAction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_AutomaticStartupAction {
    /// None
    #[serde(rename = "None")]
    None = 2,
    /// Restart_if_previously_active
    #[serde(rename = "Restart_if_previously_active")]
    RestartIfPreviouslyActive = 3,
    /// Always_startup
    #[serde(rename = "Always_startup")]
    AlwaysStartup = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
}

impl Default for VirtualSystemSettingData_AutomaticStartupAction {
    fn default() -> Self {
        Self::None
    }
}

