// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source GuestCommunicationServiceSettingData_EnabledStatePolicy
//////////////////////////////////////////////

/// GuestCommunicationServiceSettingData_EnabledStatePolicy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum GuestCommunicationServiceSettingData_EnabledStatePolicy {
    /// Enabled
    #[serde(rename = "Enabled")]
    Enabled = 2,
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 3,
    /// Deferred
    #[serde(rename = "Deferred")]
    Deferred = 8,
}

impl Default for GuestCommunicationServiceSettingData_EnabledStatePolicy {
    fn default() -> Self {
        Self::Enabled
    }
}

