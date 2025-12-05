// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Win32Provider_ImpersonationLevel
//////////////////////////////////////////////

/// Win32Provider_ImpersonationLevel enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Win32Provider_ImpersonationLevel {
    /// None
    #[serde(rename = "None")]
    None = 0,
}

impl Default for Win32Provider_ImpersonationLevel {
    fn default() -> Self {
        Self::None
    }
}

