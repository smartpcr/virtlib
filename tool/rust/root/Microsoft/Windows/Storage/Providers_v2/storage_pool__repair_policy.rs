// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StoragePool_RepairPolicy
//////////////////////////////////////////////

/// StoragePool_RepairPolicy enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StoragePool_RepairPolicy {
    /// Sequential
    #[serde(rename = "Sequential")]
    Sequential = 2,
    /// Parallel
    #[serde(rename = "Parallel")]
    Parallel = 3,
}

impl Default for StoragePool_RepairPolicy {
    fn default() -> Self {
        Self::Sequential
    }
}

