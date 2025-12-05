// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_ProtectionStatus
//////////////////////////////////////////////

/// EncryptableVolume_ProtectionStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_ProtectionStatus {
    /// Unprotected
    #[serde(rename = "Unprotected")]
    Unprotected = 0,
    /// Protected
    #[serde(rename = "Protected")]
    Protected = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
}

impl Default for EncryptableVolume_ProtectionStatus {
    fn default() -> Self {
        Self::Unprotected
    }
}

