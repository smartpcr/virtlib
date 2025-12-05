// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DeliveryOptimizationConfig_downloadMode
//////////////////////////////////////////////

/// DeliveryOptimizationConfig_downloadMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DeliveryOptimizationConfig_downloadMode {
    /// _1
    #[serde(rename = "_1")]
    V1 = 0,
    /// _2
    #[serde(rename = "_2")]
    V2 = 1,
    /// _4
    #[serde(rename = "_4")]
    V4 = 3,
}

impl Default for DeliveryOptimizationConfig_downloadMode {
    fn default() -> Self {
        Self::V1
    }
}

