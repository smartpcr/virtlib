// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PMAdminConfigState_NdisPMAdminConfigState
//////////////////////////////////////////////

/// PMAdminConfigState_NdisPMAdminConfigState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PMAdminConfigState_NdisPMAdminConfigState {
    /// NdisPMAdminConfigStateUnspecified
    #[serde(rename = "NdisPMAdminConfigStateUnspecified")]
    NdisPMAdminConfigStateUnspecified = 0,
    /// NdisPMAdminConfigStateDisabled
    #[serde(rename = "NdisPMAdminConfigStateDisabled")]
    NdisPMAdminConfigStateDisabled = 1,
    /// NdisPMAdminConfigStateEnabled
    #[serde(rename = "NdisPMAdminConfigStateEnabled")]
    NdisPMAdminConfigStateEnabled = 2,
}

impl Default for PMAdminConfigState_NdisPMAdminConfigState {
    fn default() -> Self {
        Self::NdisPMAdminConfigStateUnspecified
    }
}

