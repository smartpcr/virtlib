// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_ForceEncryptionType
//////////////////////////////////////////////

/// EncryptableVolume_ForceEncryptionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_ForceEncryptionType {
    /// Unspecified
    #[serde(rename = "Unspecified")]
    Unspecified = 0,
    /// Software
    #[serde(rename = "Software")]
    Software = 1,
    /// Hardware
    #[serde(rename = "Hardware")]
    Hardware = 2,
}

impl Default for EncryptableVolume_ForceEncryptionType {
    fn default() -> Self {
        Self::Unspecified
    }
}

