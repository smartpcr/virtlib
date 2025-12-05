// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_BindingState
//////////////////////////////////////////////

/// EncryptableVolume_BindingState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_BindingState {
    /// NotPossible
    #[serde(rename = "NotPossible")]
    NotPossible = 0,
    /// DisabledByPolicy
    #[serde(rename = "DisabledByPolicy")]
    DisabledByPolicy = 1,
    /// Possible
    #[serde(rename = "Possible")]
    Possible = 2,
    /// Bound
    #[serde(rename = "Bound")]
    Bound = 3,
}

impl Default for EncryptableVolume_BindingState {
    fn default() -> Self {
        Self::NotPossible
    }
}

