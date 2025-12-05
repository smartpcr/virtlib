// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageTier_MediaType
//////////////////////////////////////////////

/// StorageTier_MediaType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageTier_MediaType {
    /// HDD
    #[serde(rename = "HDD")]
    HDD = 3,
    /// SSD
    #[serde(rename = "SSD")]
    SSD = 4,
}

impl Default for StorageTier_MediaType {
    fn default() -> Self {
        Self::HDD
    }
}

