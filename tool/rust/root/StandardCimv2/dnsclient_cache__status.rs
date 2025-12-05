// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DNSClientCache_Status
//////////////////////////////////////////////

/// DNSClientCache_Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DNSClientCache_Status {
    /// _114
    #[serde(rename = "_114")]
    V114 = 0,
    /// _682
    #[serde(rename = "_682")]
    V682 = 9003,
    /// _683
    #[serde(rename = "_683")]
    V683 = 9701,
}

impl Default for DNSClientCache_Status {
    fn default() -> Self {
        Self::V114
    }
}

