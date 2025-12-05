// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FileOperation_IsDirectory
//////////////////////////////////////////////

/// FileOperation_IsDirectory enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FileOperation_IsDirectory {
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

impl Default for FileOperation_IsDirectory {
    fn default() -> Self {
        Self::FalseValue
    }
}

