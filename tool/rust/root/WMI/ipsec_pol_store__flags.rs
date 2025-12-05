// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source IpsecPolStore_Flags
//////////////////////////////////////////////

/// IpsecPolStore_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum IpsecPolStore_Flags {
    /// TRC_FN_ENTER
    #[serde(rename = "TRC_FN_ENTER")]
    TRCFNENTER = 1,
    /// TRC_FN_LEAVE
    #[serde(rename = "TRC_FN_LEAVE")]
    TRCFNLEAVE = 2,
    /// TRC_INFORMATION
    #[serde(rename = "TRC_INFORMATION")]
    TRCINFORMATION = 3,
    /// TRC_WARNING
    #[serde(rename = "TRC_WARNING")]
    TRCWARNING = 4,
    /// TRC_ERROR
    #[serde(rename = "TRC_ERROR")]
    TRCERROR = 5,
}

impl Default for IpsecPolStore_Flags {
    fn default() -> Self {
        Self::TRCFNENTER
    }
}

