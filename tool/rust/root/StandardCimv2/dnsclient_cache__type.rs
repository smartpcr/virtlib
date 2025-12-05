// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DNSClientCache_Type
//////////////////////////////////////////////

/// DNSClientCache_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DNSClientCache_Type {
    /// _666
    #[serde(rename = "_666")]
    V666 = 1,
    /// _667
    #[serde(rename = "_667")]
    V667 = 2,
    /// _668
    #[serde(rename = "_668")]
    V668 = 5,
    /// _669
    #[serde(rename = "_669")]
    V669 = 6,
    /// _670
    #[serde(rename = "_670")]
    V670 = 12,
    /// _671
    #[serde(rename = "_671")]
    V671 = 15,
    /// _672
    #[serde(rename = "_672")]
    V672 = 28,
    /// _673
    #[serde(rename = "_673")]
    V673 = 33,
}

impl Default for DNSClientCache_Type {
    fn default() -> Self {
        Self::V666
    }
}

