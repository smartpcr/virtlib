// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RedundancySet_LoadBalanceAlgorithm
//////////////////////////////////////////////

/// RedundancySet_LoadBalanceAlgorithm enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RedundancySet_LoadBalanceAlgorithm {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// No_Load_Balancing
    #[serde(rename = "No_Load_Balancing")]
    NoLoadBalancing = 2,
    /// Round_Robin
    #[serde(rename = "Round_Robin")]
    RoundRobin = 3,
    /// Least_Blocks
    #[serde(rename = "Least_Blocks")]
    LeastBlocks = 4,
    /// Least_IO
    #[serde(rename = "Least_IO")]
    LeastIO = 5,
    /// Address_Region
    #[serde(rename = "Address_Region")]
    AddressRegion = 6,
    /// Product_Specific
    #[serde(rename = "Product_Specific")]
    ProductSpecific = 7,
}

impl Default for RedundancySet_LoadBalanceAlgorithm {
    fn default() -> Self {
        Self::Unknown
    }
}

