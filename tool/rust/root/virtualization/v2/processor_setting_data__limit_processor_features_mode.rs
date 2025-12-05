// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProcessorSettingData_LimitProcessorFeaturesMode
//////////////////////////////////////////////

/// ProcessorSettingData_LimitProcessorFeaturesMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProcessorSettingData_LimitProcessorFeaturesMode {
    /// DefaultMinimumFeatures
    #[serde(rename = "DefaultMinimumFeatures")]
    DefaultMinimumFeatures = 0,
    /// ClusterNodeCommonFeatures
    #[serde(rename = "ClusterNodeCommonFeatures")]
    ClusterNodeCommonFeatures = 1,
}

impl Default for ProcessorSettingData_LimitProcessorFeaturesMode {
    fn default() -> Self {
        Self::DefaultMinimumFeatures
    }
}

