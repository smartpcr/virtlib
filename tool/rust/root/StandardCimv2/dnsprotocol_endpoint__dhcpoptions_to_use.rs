// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DNSProtocolEndpoint_DHCPOptionsToUse
//////////////////////////////////////////////

/// DNSProtocolEndpoint_DHCPOptionsToUse enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DNSProtocolEndpoint_DHCPOptionsToUse {
    /// _651
    #[serde(rename = "_651")]
    V651 = 8,
    /// _652
    #[serde(rename = "_652")]
    V652 = 14,
    /// _653
    #[serde(rename = "_653")]
    V653 = 17,
    /// _29
    #[serde(rename = "_29")]
    V29 = 18,
    /// _30
    #[serde(rename = "_30")]
    V30 = 19,
}

impl Default for DNSProtocolEndpoint_DHCPOptionsToUse {
    fn default() -> Self {
        Self::V651
    }
}

