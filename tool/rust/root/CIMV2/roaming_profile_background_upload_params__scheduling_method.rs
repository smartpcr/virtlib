// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RoamingProfileBackgroundUploadParams_SchedulingMethod
//////////////////////////////////////////////

/// RoamingProfileBackgroundUploadParams_SchedulingMethod enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RoamingProfileBackgroundUploadParams_SchedulingMethod {
    /// SpecificTime
    #[serde(rename = "SpecificTime")]
    SpecificTime = 1,
    /// SetInterval
    #[serde(rename = "SetInterval")]
    SetInterval = 2,
}

impl Default for RoamingProfileBackgroundUploadParams_SchedulingMethod {
    fn default() -> Self {
        Self::SpecificTime
    }
}

