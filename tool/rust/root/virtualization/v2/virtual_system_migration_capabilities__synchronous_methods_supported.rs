// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationCapabilities_SynchronousMethodsSupported
//////////////////////////////////////////////

/// VirtualSystemMigrationCapabilities_SynchronousMethodsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationCapabilities_SynchronousMethodsSupported {
    /// MigrateVirtualSystemToHostSupported
    #[serde(rename = "MigrateVirtualSystemToHostSupported")]
    MigrateVirtualSystemToHostSupported = 2,
    /// MigrateVirtualSystemToSystemSupported
    #[serde(rename = "MigrateVirtualSystemToSystemSupported")]
    MigrateVirtualSystemToSystemSupported = 3,
    /// CheckVirtualSystemIsMigratableToHostSupported
    #[serde(rename = "CheckVirtualSystemIsMigratableToHostSupported")]
    CheckVirtualSystemIsMigratableToHostSupported = 4,
    /// CheckVirtualSystemIsMigratableToSystemSupported
    #[serde(rename = "CheckVirtualSystemIsMigratableToSystemSupported")]
    CheckVirtualSystemIsMigratableToSystemSupported = 5,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 6,
}

impl Default for VirtualSystemMigrationCapabilities_SynchronousMethodsSupported {
    fn default() -> Self {
        Self::MigrateVirtualSystemToHostSupported
    }
}

