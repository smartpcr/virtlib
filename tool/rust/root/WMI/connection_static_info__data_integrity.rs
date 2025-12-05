// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ConnectionStaticInfo_DataIntegrity
//////////////////////////////////////////////

/// ConnectionStaticInfo_DataIntegrity enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ConnectionStaticInfo_DataIntegrity {
    /// None
    #[serde(rename = "None")]
    None = 0,
    /// crc32c
    #[serde(rename = "crc32c")]
    Crc32c = 1,
}

impl Default for ConnectionStaticInfo_DataIntegrity {
    fn default() -> Self {
        Self::None
    }
}

