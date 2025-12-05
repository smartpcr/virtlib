// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ValidationDiskInfo_DiskIdType
//////////////////////////////////////////////

/// ValidationDiskInfo_DiskIdType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ValidationDiskInfo_DiskIdType {
    /// _3
    #[serde(rename = "_3")]
    V3 = 0,
    /// _4
    #[serde(rename = "_4")]
    V4 = 1,
}

impl Default for ValidationDiskInfo_DiskIdType {
    fn default() -> Self {
        Self::V3
    }
}

