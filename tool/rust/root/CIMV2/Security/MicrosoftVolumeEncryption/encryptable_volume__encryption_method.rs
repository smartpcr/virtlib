// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_EncryptionMethod
//////////////////////////////////////////////

/// EncryptableVolume_EncryptionMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_EncryptionMethod {
    /// Unspecified
    #[serde(rename = "Unspecified")]
    Unspecified = 0,
    /// AES_128
    #[serde(rename = "AES_128")]
    AES128 = 3,
    /// AES_256
    #[serde(rename = "AES_256")]
    AES256 = 4,
    /// XTS_AES_128
    #[serde(rename = "XTS_AES_128")]
    XTSAES128 = 6,
    /// XTS_AES_256
    #[serde(rename = "XTS_AES_256")]
    XTSAES256 = 7,
}

impl Default for EncryptableVolume_EncryptionMethod {
    fn default() -> Self {
        Self::Unspecified
    }
}

