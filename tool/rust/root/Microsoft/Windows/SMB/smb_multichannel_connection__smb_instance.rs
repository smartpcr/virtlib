// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbMultichannelConnection_SmbInstance
//////////////////////////////////////////////

/// SmbMultichannelConnection_SmbInstance enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbMultichannelConnection_SmbInstance {
    /// _9
    #[serde(rename = "_9")]
    V9 = 0,
    /// _10
    #[serde(rename = "_10")]
    V10 = 1,
    /// _56
    #[serde(rename = "_56")]
    V56 = 2,
    /// _57
    #[serde(rename = "_57")]
    V57 = 3,
}

impl Default for SmbMultichannelConnection_SmbInstance {
    fn default() -> Self {
        Self::V9
    }
}

