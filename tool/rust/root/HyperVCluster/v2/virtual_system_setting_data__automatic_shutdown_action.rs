// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_AutomaticShutdownAction
//////////////////////////////////////////////

/// VirtualSystemSettingData_AutomaticShutdownAction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_AutomaticShutdownAction {
    /// Turn_Off
    #[serde(rename = "Turn_Off")]
    TurnOff = 2,
    /// Save_state
    #[serde(rename = "Save_state")]
    SaveState = 3,
    /// Shutdown
    #[serde(rename = "Shutdown")]
    Shutdown = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
}

impl Default for VirtualSystemSettingData_AutomaticShutdownAction {
    fn default() -> Self {
        Self::TurnOff
    }
}

