// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_AutomaticRecoveryAction
//////////////////////////////////////////////

/// VirtualSystemSettingData_AutomaticRecoveryAction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_AutomaticRecoveryAction {
    /// None
    #[serde(rename = "None")]
    None = 2,
    /// Restart
    #[serde(rename = "Restart")]
    Restart = 3,
    /// Revert_to_snapshot
    #[serde(rename = "Revert_to_snapshot")]
    RevertToSnapshot = 4,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 5,
}

impl Default for VirtualSystemSettingData_AutomaticRecoveryAction {
    fn default() -> Self {
        Self::None
    }
}

