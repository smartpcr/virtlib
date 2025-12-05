// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DNSClientCache_Section
//////////////////////////////////////////////

/// DNSClientCache_Section enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DNSClientCache_Section {
    /// _677
    #[serde(rename = "_677")]
    V677 = 1,
    /// _678
    #[serde(rename = "_678")]
    V678 = 2,
    /// _679
    #[serde(rename = "_679")]
    V679 = 3,
}

impl Default for DNSClientCache_Section {
    fn default() -> Self {
        Self::V677
    }
}

