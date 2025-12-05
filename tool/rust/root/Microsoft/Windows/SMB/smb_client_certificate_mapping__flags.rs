// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbClientCertificateMapping_Flags
//////////////////////////////////////////////

/// SmbClientCertificateMapping_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbClientCertificateMapping_Flags {
    /// _18
    #[serde(rename = "_18")]
    V18 = 0,
    /// _67
    #[serde(rename = "_67")]
    V67 = 1,
    /// _65
    #[serde(rename = "_65")]
    V65 = 2,
}

impl Default for SmbClientCertificateMapping_Flags {
    fn default() -> Self {
        Self::V18
    }
}

