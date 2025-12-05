// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RegistryKey_Mode
//////////////////////////////////////////////

/// RegistryKey_Mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RegistryKey_Mode {
    /// Inherit
    #[serde(rename = "Inherit")]
    Inherit = 0,
    /// Ignore
    #[serde(rename = "Ignore")]
    Ignore = 1,
    /// Overwrite
    #[serde(rename = "Overwrite")]
    Overwrite = 2,
}

impl Default for RegistryKey_Mode {
    fn default() -> Self {
        Self::Inherit
    }
}

