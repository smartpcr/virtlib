// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source DOUploadUsage_MonthlyUploadRestriction
//////////////////////////////////////////////

/// DOUploadUsage_MonthlyUploadRestriction enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum DOUploadUsage_MonthlyUploadRestriction {
    /// _29
    #[serde(rename = "_29")]
    V29 = 0,
    /// _30
    #[serde(rename = "_30")]
    V30 = 1,
    /// _31
    #[serde(rename = "_31")]
    V31 = 2,
}

impl Default for DOUploadUsage_MonthlyUploadRestriction {
    fn default() -> Self {
        Self::V29
    }
}

