// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TLBError_MemoryHierarchyLevel
//////////////////////////////////////////////

/// TLBError_MemoryHierarchyLevel enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TLBError_MemoryHierarchyLevel {
    /// Level_0
    #[serde(rename = "Level_0")]
    Level0 = 0,
    /// Level_1
    #[serde(rename = "Level_1")]
    Level1 = 1,
    /// Level_2
    #[serde(rename = "Level_2")]
    Level2 = 2,
    /// Generic
    #[serde(rename = "Generic")]
    Generic = 3,
}

impl Default for TLBError_MemoryHierarchyLevel {
    fn default() -> Self {
        Self::Level0
    }
}

