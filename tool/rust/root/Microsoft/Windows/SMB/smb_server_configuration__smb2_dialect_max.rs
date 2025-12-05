// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbServerConfiguration_Smb2DialectMax
//////////////////////////////////////////////

/// SmbServerConfiguration_Smb2DialectMax enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbServerConfiguration_Smb2DialectMax {
    /// _77
    #[serde(rename = "_77")]
    V77 = 514,
    /// _78
    #[serde(rename = "_78")]
    V78 = 528,
    /// _79
    #[serde(rename = "_79")]
    V79 = 768,
    /// _80
    #[serde(rename = "_80")]
    V80 = 770,
    /// _94
    #[serde(rename = "_94")]
    V94 = 785,
    /// _18
    #[serde(rename = "_18")]
    V18 = 65535,
}

impl Default for SmbServerConfiguration_Smb2DialectMax {
    fn default() -> Self {
        Self::V77
    }
}

