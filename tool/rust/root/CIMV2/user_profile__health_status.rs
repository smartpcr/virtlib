// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source UserProfile_HealthStatus
//////////////////////////////////////////////

/// UserProfile_HealthStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum UserProfile_HealthStatus {
    /// Healthy
    #[serde(rename = "Healthy")]
    Healthy = 0,
    /// Unhealthy
    #[serde(rename = "Unhealthy")]
    Unhealthy = 1,
    /// Caution
    #[serde(rename = "Caution")]
    Caution = 2,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 3,
}

impl Default for UserProfile_HealthStatus {
    fn default() -> Self {
        Self::Healthy
    }
}

