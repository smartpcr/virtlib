// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RoamingUserHealthConfiguration_HealthStatusForTempProfiles
//////////////////////////////////////////////

/// RoamingUserHealthConfiguration_HealthStatusForTempProfiles enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RoamingUserHealthConfiguration_HealthStatusForTempProfiles {
    /// Healthy
    #[serde(rename = "Healthy")]
    Healthy = 0,
    /// Unhealthy
    #[serde(rename = "Unhealthy")]
    Unhealthy = 1,
    /// Caution
    #[serde(rename = "Caution")]
    Caution = 2,
}

impl Default for RoamingUserHealthConfiguration_HealthStatusForTempProfiles {
    fn default() -> Self {
        Self::Healthy
    }
}

