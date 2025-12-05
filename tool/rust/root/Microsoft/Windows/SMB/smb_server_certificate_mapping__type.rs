// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbServerCertificateMapping_Type
//////////////////////////////////////////////

/// SmbServerCertificateMapping_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbServerCertificateMapping_Type {
    /// _61
    #[serde(rename = "_61")]
    V61 = 0,
}

impl Default for SmbServerCertificateMapping_Type {
    fn default() -> Self {
        Self::V61
    }
}

