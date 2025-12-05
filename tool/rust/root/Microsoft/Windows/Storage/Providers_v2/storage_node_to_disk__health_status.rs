// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source StorageNodeToDisk_HealthStatus
//////////////////////////////////////////////

/// StorageNodeToDisk_HealthStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StorageNodeToDisk_HealthStatus {
    /// Healthy
    #[serde(rename = "Healthy")]
    Healthy = 0,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 1,
    /// Unhealthy
    #[serde(rename = "Unhealthy")]
    Unhealthy = 2,
}

impl Default for StorageNodeToDisk_HealthStatus {
    fn default() -> Self {
        Self::Healthy
    }
}

