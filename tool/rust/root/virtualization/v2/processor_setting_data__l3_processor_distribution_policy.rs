// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProcessorSettingData_L3ProcessorDistributionPolicy
//////////////////////////////////////////////

/// ProcessorSettingData_L3ProcessorDistributionPolicy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProcessorSettingData_L3ProcessorDistributionPolicy {
    /// SmallToLarge__Default_
    #[serde(rename = "SmallToLarge__Default_")]
    SmallToLargeDefault = 0,
    /// LargeToSmall
    #[serde(rename = "LargeToSmall")]
    LargeToSmall = 1,
    /// EvenSmallToLarge
    #[serde(rename = "EvenSmallToLarge")]
    EvenSmallToLarge = 2,
    /// EvenLargeToSmall
    #[serde(rename = "EvenLargeToSmall")]
    EvenLargeToSmall = 3,
}

impl Default for ProcessorSettingData_L3ProcessorDistributionPolicy {
    fn default() -> Self {
        Self::SmallToLargeDefault
    }
}

