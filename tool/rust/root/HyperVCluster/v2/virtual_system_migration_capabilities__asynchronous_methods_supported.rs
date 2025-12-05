// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported
//////////////////////////////////////////////

/// VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported {
    /// MigrateVirtualSystemToHostSupported
    #[serde(rename = "MigrateVirtualSystemToHostSupported")]
    MigrateVirtualSystemToHostSupported = 2,
    /// MigrateVirtualSystemToSystemSupported
    #[serde(rename = "MigrateVirtualSystemToSystemSupported")]
    MigrateVirtualSystemToSystemSupported = 3,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4,
}

impl Default for VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported {
    fn default() -> Self {
        Self::MigrateVirtualSystemToHostSupported
    }
}

