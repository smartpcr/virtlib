// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RemoteAppChangeEvent_OperationType
//////////////////////////////////////////////

/// RemoteAppChangeEvent_OperationType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RemoteAppChangeEvent_OperationType {
    /// Create
    #[serde(rename = "Create")]
    Create = 0,
    /// Delete
    #[serde(rename = "Delete")]
    Delete = 1,
    /// Modify
    #[serde(rename = "Modify")]
    Modify = 2,
}

impl Default for RemoteAppChangeEvent_OperationType {
    fn default() -> Self {
        Self::Create
    }
}

