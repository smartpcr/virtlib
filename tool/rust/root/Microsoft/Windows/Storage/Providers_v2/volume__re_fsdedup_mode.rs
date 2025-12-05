// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Volume_ReFSDedupMode
//////////////////////////////////////////////

/// Volume_ReFSDedupMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Volume_ReFSDedupMode {
    /// Disabled
    #[serde(rename = "Disabled")]
    Disabled = 0,
    /// Dedup
    #[serde(rename = "Dedup")]
    Dedup = 1,
    /// DedupAndCompress
    #[serde(rename = "DedupAndCompress")]
    DedupAndCompress = 2,
    /// Compress
    #[serde(rename = "Compress")]
    Compress = 3,
    /// NotAvailable
    #[serde(rename = "NotAvailable")]
    NotAvailable = 4,
}

impl Default for Volume_ReFSDedupMode {
    fn default() -> Self {
        Self::Disabled
    }
}

