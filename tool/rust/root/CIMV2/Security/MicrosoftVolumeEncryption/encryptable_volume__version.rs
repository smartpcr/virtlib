// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_Version
//////////////////////////////////////////////

/// EncryptableVolume_Version enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_Version {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Vista
    #[serde(rename = "Vista")]
    Vista = 1,
    /// Win7
    #[serde(rename = "Win7")]
    Win7 = 2,
}

impl Default for EncryptableVolume_Version {
    fn default() -> Self {
        Self::Unknown
    }
}

