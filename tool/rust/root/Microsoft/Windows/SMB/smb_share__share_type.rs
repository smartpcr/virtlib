// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_ShareType
//////////////////////////////////////////////

/// SmbShare_ShareType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_ShareType {
    /// _11
    #[serde(rename = "_11")]
    V11 = 0,
    /// _12
    #[serde(rename = "_12")]
    V12 = 1,
    /// _13
    #[serde(rename = "_13")]
    V13 = 2,
    /// _14
    #[serde(rename = "_14")]
    V14 = 3,
    /// _15
    #[serde(rename = "_15")]
    V15 = 4,
}

impl Default for SmbShare_ShareType {
    fn default() -> Self {
        Self::V11
    }
}

