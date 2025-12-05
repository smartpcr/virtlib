// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AlertIndication_Trending
//////////////////////////////////////////////

/// AlertIndication_Trending enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AlertIndication_Trending {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 1,
    /// Trending_Up
    #[serde(rename = "Trending_Up")]
    TrendingUp = 2,
    /// Trending_Down
    #[serde(rename = "Trending_Down")]
    TrendingDown = 3,
    /// No_Change
    #[serde(rename = "No_Change")]
    NoChange = 4,
}

impl Default for AlertIndication_Trending {
    fn default() -> Self {
        Self::Unknown
    }
}

