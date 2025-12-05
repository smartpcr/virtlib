// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbMapping_Status
//////////////////////////////////////////////

/// SmbMapping_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbMapping_Status {
    /// _36
    #[serde(rename = "_36")]
    V36 = 0,
    /// _37
    #[serde(rename = "_37")]
    V37 = 1,
    /// _38
    #[serde(rename = "_38")]
    V38 = 2,
    /// _39
    #[serde(rename = "_39")]
    V39 = 3,
    /// _40
    #[serde(rename = "_40")]
    V40 = 4,
    /// _41
    #[serde(rename = "_41")]
    V41 = 5,
    /// _42
    #[serde(rename = "_42")]
    V42 = 6,
}

impl Default for SmbMapping_Status {
    fn default() -> Self {
        Self::V36
    }
}

