// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageFaultEvent_ChangeType
//////////////////////////////////////////////

/// StorageFaultEvent_ChangeType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageFaultEvent_ChangeType {
    /// Creation
    #[serde(rename = "Creation")]
    Creation = 0,
    /// Deletion
    #[serde(rename = "Deletion")]
    Deletion = 1,
    /// Modification
    #[serde(rename = "Modification")]
    Modification = 2,
}

impl Default for StorageFaultEvent_ChangeType {
    fn default() -> Self {
        Self::Creation
    }
}

