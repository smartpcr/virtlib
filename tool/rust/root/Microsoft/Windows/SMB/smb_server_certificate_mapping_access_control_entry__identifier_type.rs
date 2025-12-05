// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbServerCertificateMappingAccessControlEntry_IdentifierType
//////////////////////////////////////////////

/// SmbServerCertificateMappingAccessControlEntry_IdentifierType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbServerCertificateMappingAccessControlEntry_IdentifierType {
    /// _74
    #[serde(rename = "_74")]
    V74 = 0,
    /// _75
    #[serde(rename = "_75")]
    V75 = 1,
}

impl Default for SmbServerCertificateMappingAccessControlEntry_IdentifierType {
    fn default() -> Self {
        Self::V74
    }
}

