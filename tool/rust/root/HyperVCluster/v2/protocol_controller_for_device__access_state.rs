// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProtocolControllerForDevice_AccessState
//////////////////////////////////////////////

/// ProtocolControllerForDevice_AccessState enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProtocolControllerForDevice_AccessState {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Active
    #[serde(rename = "Active")]
    Active = 2,
    /// Inactive
    #[serde(rename = "Inactive")]
    Inactive = 3,
    /// Replication_In_Progress
    #[serde(rename = "Replication_In_Progress")]
    ReplicationInProgress = 4,
    /// Mapping_Inconsistency
    #[serde(rename = "Mapping_Inconsistency")]
    MappingInconsistency = 5,
}

impl Default for ProtocolControllerForDevice_AccessState {
    fn default() -> Self {
        Self::Unknown
    }
}

