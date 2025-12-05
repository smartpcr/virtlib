// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageExtent_DataOrganization
//////////////////////////////////////////////

/// StorageExtent_DataOrganization enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageExtent_DataOrganization {
    /// Other
    #[serde(rename = "Other")]
    Other = 0,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 1,
    /// Fixed_Block
    #[serde(rename = "Fixed_Block")]
    FixedBlock = 2,
    /// Variable_Block
    #[serde(rename = "Variable_Block")]
    VariableBlock = 3,
    /// Count_Key_Data
    #[serde(rename = "Count_Key_Data")]
    CountKeyData = 4,
}

impl Default for StorageExtent_DataOrganization {
    fn default() -> Self {
        Self::Other
    }
}

