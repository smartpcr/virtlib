// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DedupProperties_ReFSDedupCompressionFormat
//////////////////////////////////////////////

/// DedupProperties_ReFSDedupCompressionFormat enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DedupProperties_ReFSDedupCompressionFormat {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Uncompressed
    #[serde(rename = "Uncompressed")]
    Uncompressed = 1,
    /// LZ4
    #[serde(rename = "LZ4")]
    LZ4 = 2,
    /// ZSTD
    #[serde(rename = "ZSTD")]
    ZSTD = 3,
}

impl Default for DedupProperties_ReFSDedupCompressionFormat {
    fn default() -> Self {
        Self::Unknown
    }
}

