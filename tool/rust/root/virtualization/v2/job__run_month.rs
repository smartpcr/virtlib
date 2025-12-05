// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Job_RunMonth
//////////////////////////////////////////////

/// Job_RunMonth enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Job_RunMonth {
    /// January
    #[serde(rename = "January")]
    January = 0,
    /// February
    #[serde(rename = "February")]
    February = 1,
    /// March
    #[serde(rename = "March")]
    March = 2,
    /// April
    #[serde(rename = "April")]
    April = 3,
    /// May
    #[serde(rename = "May")]
    May = 4,
    /// June
    #[serde(rename = "June")]
    June = 5,
    /// July
    #[serde(rename = "July")]
    July = 6,
    /// August
    #[serde(rename = "August")]
    August = 7,
    /// September
    #[serde(rename = "September")]
    September = 8,
    /// October
    #[serde(rename = "October")]
    October = 9,
    /// November
    #[serde(rename = "November")]
    November = 10,
    /// December
    #[serde(rename = "December")]
    December = 11,
}

impl Default for Job_RunMonth {
    fn default() -> Self {
        Self::January
    }
}

