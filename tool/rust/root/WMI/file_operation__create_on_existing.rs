// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileOperation_CreateOnExisting
//////////////////////////////////////////////

/// FileOperation_CreateOnExisting enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileOperation_CreateOnExisting {
    /// _False
    #[serde(rename = "_False")]
    FalseValue = 0,
    /// _True
    #[serde(rename = "_True")]
    TrueValue = 1,
    /// NA
    #[serde(rename = "NA")]
    NA = 2,
}

impl Default for FileOperation_CreateOnExisting {
    fn default() -> Self {
        Self::FalseValue
    }
}

