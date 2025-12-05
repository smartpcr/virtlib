// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CredentialRoaming_Flags
//////////////////////////////////////////////

/// CredentialRoaming_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CredentialRoaming_Flags {
    /// TRC_FATAL
    #[serde(rename = "TRC_FATAL")]
    TRCFATAL = 1,
    /// TRC_ERROR
    #[serde(rename = "TRC_ERROR")]
    TRCERROR = 2,
    /// TRC_WARNING
    #[serde(rename = "TRC_WARNING")]
    TRCWARNING = 3,
    /// TRC_INFO
    #[serde(rename = "TRC_INFO")]
    TRCINFO = 4,
    /// TRC_DETAIL
    #[serde(rename = "TRC_DETAIL")]
    TRCDETAIL = 5,
}

impl Default for CredentialRoaming_Flags {
    fn default() -> Self {
        Self::TRCFATAL
    }
}

