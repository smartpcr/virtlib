// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RegistryValue_Type
//////////////////////////////////////////////

/// RegistryValue_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RegistryValue_Type {
    /// REG_SZ
    #[serde(rename = "REG_SZ")]
    REGSZ = 1,
    /// REG_EXPAND_SZ
    #[serde(rename = "REG_EXPAND_SZ")]
    REGEXPANDSZ = 2,
    /// REG_BINARY
    #[serde(rename = "REG_BINARY")]
    REGBINARY = 3,
    /// REG_DWORD
    #[serde(rename = "REG_DWORD")]
    REGDWORD = 4,
    /// REG_MULTI_SZ
    #[serde(rename = "REG_MULTI_SZ")]
    REGMULTISZ = 7,
}

impl Default for RegistryValue_Type {
    fn default() -> Self {
        Self::REGSZ
    }
}

