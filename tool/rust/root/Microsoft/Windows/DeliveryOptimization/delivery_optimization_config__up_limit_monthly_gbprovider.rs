// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DeliveryOptimizationConfig_UpLimitMonthlyGBProvider
//////////////////////////////////////////////

/// DeliveryOptimizationConfig_UpLimitMonthlyGBProvider enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DeliveryOptimizationConfig_UpLimitMonthlyGBProvider {
    /// _17
    #[serde(rename = "_17")]
    V17 = 5,
    /// _18
    #[serde(rename = "_18")]
    V18 = 7,
    /// _19
    #[serde(rename = "_19")]
    V19 = 8,
    /// _20
    #[serde(rename = "_20")]
    V20 = 9,
    /// _21
    #[serde(rename = "_21")]
    V21 = 99,
}

impl Default for DeliveryOptimizationConfig_UpLimitMonthlyGBProvider {
    fn default() -> Self {
        Self::V17
    }
}

