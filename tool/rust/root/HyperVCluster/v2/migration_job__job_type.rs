// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MigrationJob_JobType
//////////////////////////////////////////////

/// MigrationJob_JobType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MigrationJob_JobType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Creating_Remote_Virtual_Machine
    #[serde(rename = "Creating_Remote_Virtual_Machine")]
    CreatingRemoteVirtualMachine = 300,
    /// Checking_Virtual_Machine_Compatibility
    #[serde(rename = "Checking_Virtual_Machine_Compatibility")]
    CheckingVirtualMachineCompatibility = 301,
    /// Checking_Virtual_Machine_and_Storage_Compatibility
    #[serde(rename = "Checking_Virtual_Machine_and_Storage_Compatibility")]
    CheckingVirtualMachineAndStorageCompatibility = 302,
    /// Checking_Storage_Compatibility
    #[serde(rename = "Checking_Storage_Compatibility")]
    CheckingStorageCompatibility = 303,
    /// Checking_Storage_Migration
    #[serde(rename = "Checking_Storage_Migration")]
    CheckingStorageMigration = 304,
    /// Moving_Virtual_Machine
    #[serde(rename = "Moving_Virtual_Machine")]
    MovingVirtualMachine = 305,
    /// Moving_Virtual_Machine_and_Storage
    #[serde(rename = "Moving_Virtual_Machine_and_Storage")]
    MovingVirtualMachineAndStorage = 306,
    /// Moving_Storage
    #[serde(rename = "Moving_Storage")]
    MovingStorage = 307,
}

impl Default for MigrationJob_JobType {
    fn default() -> Self {
        Self::Unknown
    }
}

