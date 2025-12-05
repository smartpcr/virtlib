// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbGlobalMapping_TransportType
//////////////////////////////////////////////

/// SmbGlobalMapping_TransportType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbGlobalMapping_TransportType {
    /// _18
    #[serde(rename = "_18")]
    V18 = 0,
    /// _68
    #[serde(rename = "_68")]
    V68 = 1,
    /// _61
    #[serde(rename = "_61")]
    V61 = 2,
}

impl Default for SmbGlobalMapping_TransportType {
    fn default() -> Self {
        Self::V18
    }
}

