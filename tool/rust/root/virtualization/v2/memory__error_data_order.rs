// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Memory_ErrorDataOrder
//////////////////////////////////////////////

/// Memory_ErrorDataOrder enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Memory_ErrorDataOrder {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Least_Significant_Byte_First
    #[serde(rename = "Least_Significant_Byte_First")]
    LeastSignificantByteFirst = 1,
    /// Most_Significant_Byte_First
    #[serde(rename = "Most_Significant_Byte_First")]
    MostSignificantByteFirst = 2,
}

impl Default for Memory_ErrorDataOrder {
    fn default() -> Self {
        Self::Unknown
    }
}

