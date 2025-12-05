// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSDeploymentSettings_ColorBitDepth
//////////////////////////////////////////////

/// TSDeploymentSettings_ColorBitDepth enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSDeploymentSettings_ColorBitDepth {
    /// _4
    #[serde(rename = "_4")]
    V4 = 0,
    /// _8
    #[serde(rename = "_8")]
    V8 = 1,
    /// _15
    #[serde(rename = "_15")]
    V15 = 2,
    /// _16
    #[serde(rename = "_16")]
    V16 = 3,
    /// _24
    #[serde(rename = "_24")]
    V24 = 4,
    /// _32
    #[serde(rename = "_32")]
    V32 = 5,
}

impl Default for TSDeploymentSettings_ColorBitDepth {
    fn default() -> Self {
        Self::V4
    }
}

