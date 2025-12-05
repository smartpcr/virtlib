// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Job_RunDayOfWeek
//////////////////////////////////////////////

/// Job_RunDayOfWeek enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Job_RunDayOfWeek {
    /// _Saturday
    #[serde(rename = "_Saturday")]
    Saturday = -7,
    /// _Friday
    #[serde(rename = "_Friday")]
    Friday = -6,
    /// _Thursday
    #[serde(rename = "_Thursday")]
    Thursday = -5,
    /// _Wednesday
    #[serde(rename = "_Wednesday")]
    Wednesday = -4,
    /// _Tuesday
    #[serde(rename = "_Tuesday")]
    Tuesday = -3,
    /// _Monday
    #[serde(rename = "_Monday")]
    Monday = -2,
    /// _Sunday
    #[serde(rename = "_Sunday")]
    Sunday = -1,
    /// ExactDayOfMonth
    #[serde(rename = "ExactDayOfMonth")]
    ExactDayOfMonth = 0,
    /// Sunday
    #[serde(rename = "Sunday")]
    Sunday = 1,
    /// Monday
    #[serde(rename = "Monday")]
    Monday = 2,
    /// Tuesday
    #[serde(rename = "Tuesday")]
    Tuesday = 3,
    /// Wednesday
    #[serde(rename = "Wednesday")]
    Wednesday = 4,
    /// Thursday
    #[serde(rename = "Thursday")]
    Thursday = 5,
    /// Friday
    #[serde(rename = "Friday")]
    Friday = 6,
    /// Saturday
    #[serde(rename = "Saturday")]
    Saturday = 7,
}

impl Default for Job_RunDayOfWeek {
    fn default() -> Self {
        Self::Saturday
    }
}

