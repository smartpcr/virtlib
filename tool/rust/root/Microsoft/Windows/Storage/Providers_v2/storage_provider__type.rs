// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageProvider_Type
//////////////////////////////////////////////

/// StorageProvider_Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageProvider_Type {
    /// SMP
    #[serde(rename = "SMP")]
    SMP = 1,
}

impl Default for StorageProvider_Type {
    fn default() -> Self {
        Self::SMP
    }
}

