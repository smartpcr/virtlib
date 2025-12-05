// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_TpmProtectorVersion
//////////////////////////////////////////////

/// EncryptableVolume_TpmProtectorVersion enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_TpmProtectorVersion {
    /// TpmProtectorVersion1
    #[serde(rename = "TpmProtectorVersion1")]
    TpmProtectorVersion1 = 1,
    /// TpmProtectorVersion2
    #[serde(rename = "TpmProtectorVersion2")]
    TpmProtectorVersion2 = 2,
    /// TpmProtectorVersionMax
    #[serde(rename = "TpmProtectorVersionMax")]
    TpmProtectorVersionMax = 3,
}

impl Default for EncryptableVolume_TpmProtectorVersion {
    fn default() -> Self {
        Self::TpmProtectorVersion1
    }
}

