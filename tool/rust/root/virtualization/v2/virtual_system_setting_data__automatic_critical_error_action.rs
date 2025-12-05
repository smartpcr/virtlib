// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_AutomaticCriticalErrorAction
//////////////////////////////////////////////

/// VirtualSystemSettingData_AutomaticCriticalErrorAction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_AutomaticCriticalErrorAction {
    /// None
    #[serde(rename = "None")]
    None = 0,
    /// Pause_Resume
    #[serde(rename = "Pause_Resume")]
    PauseResume = 1,
}

impl Default for VirtualSystemSettingData_AutomaticCriticalErrorAction {
    fn default() -> Self {
        Self::None
    }
}

