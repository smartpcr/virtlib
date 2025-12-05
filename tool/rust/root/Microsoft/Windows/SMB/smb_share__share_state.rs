// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_ShareState
//////////////////////////////////////////////

/// SmbShare_ShareState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_ShareState {
    /// _23
    #[serde(rename = "_23")]
    V23 = 0,
    /// _24
    #[serde(rename = "_24")]
    V24 = 1,
    /// _25
    #[serde(rename = "_25")]
    V25 = 2,
}

impl Default for SmbShare_ShareState {
    fn default() -> Self {
        Self::V23
    }
}

