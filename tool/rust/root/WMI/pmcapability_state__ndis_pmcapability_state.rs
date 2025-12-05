// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PMCapabilityState_NdisPMCapabilityState
//////////////////////////////////////////////

/// PMCapabilityState_NdisPMCapabilityState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PMCapabilityState_NdisPMCapabilityState {
    /// NdisPMAdminConfigUnsupported
    #[serde(rename = "NdisPMAdminConfigUnsupported")]
    NdisPMAdminConfigUnsupported = 0,
    /// NdisPMAdminConfigInactive
    #[serde(rename = "NdisPMAdminConfigInactive")]
    NdisPMAdminConfigInactive = 1,
    /// NdisPMAdminConfigActive
    #[serde(rename = "NdisPMAdminConfigActive")]
    NdisPMAdminConfigActive = 2,
}

impl Default for PMCapabilityState_NdisPMCapabilityState {
    fn default() -> Self {
        Self::NdisPMAdminConfigUnsupported
    }
}

