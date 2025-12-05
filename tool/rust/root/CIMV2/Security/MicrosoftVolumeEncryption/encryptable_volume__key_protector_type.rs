// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_KeyProtectorType
//////////////////////////////////////////////

/// EncryptableVolume_KeyProtectorType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_KeyProtectorType {
    /// All
    #[serde(rename = "All")]
    All = 0,
    /// TPM
    #[serde(rename = "TPM")]
    TPM = 1,
    /// ExternalKey
    #[serde(rename = "ExternalKey")]
    ExternalKey = 2,
    /// NumericPassword
    #[serde(rename = "NumericPassword")]
    NumericPassword = 3,
    /// TPM_PIN
    #[serde(rename = "TPM_PIN")]
    TPMPIN = 4,
    /// TPM_StartupKey
    #[serde(rename = "TPM_StartupKey")]
    TPMStartupKey = 5,
    /// TPM_PIN_StartupKey
    #[serde(rename = "TPM_PIN_StartupKey")]
    TPMPINStartupKey = 6,
    /// Certificate
    #[serde(rename = "Certificate")]
    Certificate = 7,
    /// PassPhrase
    #[serde(rename = "PassPhrase")]
    PassPhrase = 8,
    /// TPM_Certificate
    #[serde(rename = "TPM_Certificate")]
    TPMCertificate = 9,
    /// Identity
    #[serde(rename = "Identity")]
    Identity = 10,
}

impl Default for EncryptableVolume_KeyProtectorType {
    fn default() -> Self {
        Self::All
    }
}

