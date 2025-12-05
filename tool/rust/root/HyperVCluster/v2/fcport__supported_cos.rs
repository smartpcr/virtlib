// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FCPort_SupportedCOS
//////////////////////////////////////////////

/// FCPort_SupportedCOS enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FCPort_SupportedCOS {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// _1
    #[serde(rename = "_1")]
    V1 = 1,
    /// _2
    #[serde(rename = "_2")]
    V2 = 2,
    /// _3
    #[serde(rename = "_3")]
    V3 = 3,
    /// _4
    #[serde(rename = "_4")]
    V4 = 4,
    /// _5
    #[serde(rename = "_5")]
    V5 = 5,
    /// _6
    #[serde(rename = "_6")]
    V6 = 6,
    /// F
    #[serde(rename = "F")]
    F = 7,
}

impl Default for FCPort_SupportedCOS {
    fn default() -> Self {
        Self::Unknown
    }
}

