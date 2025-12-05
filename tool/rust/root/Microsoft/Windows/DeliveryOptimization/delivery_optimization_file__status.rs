// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DeliveryOptimizationFile_Status
//////////////////////////////////////////////

/// DeliveryOptimizationFile_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DeliveryOptimizationFile_Status {
    /// _7
    #[serde(rename = "_7")]
    V7 = 0,
    /// _8
    #[serde(rename = "_8")]
    V8 = 1,
    /// _9
    #[serde(rename = "_9")]
    V9 = 2,
    /// _10
    #[serde(rename = "_10")]
    V10 = 3,
}

impl Default for DeliveryOptimizationFile_Status {
    fn default() -> Self {
        Self::V7
    }
}

