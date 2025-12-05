// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Job_LocalOrUtcTime
//////////////////////////////////////////////

/// Job_LocalOrUtcTime enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Job_LocalOrUtcTime {
    /// Local_Time
    #[serde(rename = "Local_Time")]
    LocalTime = 1,
    /// UTC_Time
    #[serde(rename = "UTC_Time")]
    UTCTime = 2,
}

impl Default for Job_LocalOrUtcTime {
    fn default() -> Self {
        Self::LocalTime
    }
}

