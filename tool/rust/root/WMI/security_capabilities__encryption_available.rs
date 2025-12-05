// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SecurityCapabilities_EncryptionAvailable
//////////////////////////////////////////////

/// SecurityCapabilities_EncryptionAvailable enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SecurityCapabilities_EncryptionAvailable {
    /// No_Encryption_Authentication
    #[serde(rename = "No_Encryption_Authentication")]
    NoEncryptionAuthentication = 0,
    /// _3DES_HMAC_SHA1
    #[serde(rename = "_3DES_HMAC_SHA1")]
    V3DESHMACSHA1 = 1,
    /// AES_CTR_CBC_MAC_with_XCBC
    #[serde(rename = "AES_CTR_CBC_MAC_with_XCBC")]
    AESCTRCBCMACWithXCBC = 2,
}

impl Default for SecurityCapabilities_EncryptionAvailable {
    fn default() -> Self {
        Self::NoEncryptionAuthentication
    }
}

