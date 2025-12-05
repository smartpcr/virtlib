// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BcdObject_PartitionStyle
//////////////////////////////////////////////

/// BcdObject_PartitionStyle enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BcdObject_PartitionStyle {
    /// MBR
    #[serde(rename = "MBR")]
    MBR = 0,
    /// GPT
    #[serde(rename = "GPT")]
    GPT = 1,
}

impl Default for BcdObject_PartitionStyle {
    fn default() -> Self {
        Self::MBR
    }
}

