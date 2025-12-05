// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BaseMetricDefinition_Calculable
//////////////////////////////////////////////

/// BaseMetricDefinition_Calculable enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BaseMetricDefinition_Calculable {
    /// Non_calculable
    #[serde(rename = "Non_calculable")]
    NonCalculable = 1,
    /// Summable
    #[serde(rename = "Summable")]
    Summable = 2,
    /// Non_summable
    #[serde(rename = "Non_summable")]
    NonSummable = 3,
}

impl Default for BaseMetricDefinition_Calculable {
    fn default() -> Self {
        Self::NonCalculable
    }
}

