// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSPublishedApplication_CommandLineSetting
//////////////////////////////////////////////

/// TSPublishedApplication_CommandLineSetting enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSPublishedApplication_CommandLineSetting {
    /// DoNotAllow
    #[serde(rename = "DoNotAllow")]
    DoNotAllow = 0,
    /// Allow
    #[serde(rename = "Allow")]
    Allow = 1,
    /// Require
    #[serde(rename = "Require")]
    Require = 2,
}

impl Default for TSPublishedApplication_CommandLineSetting {
    fn default() -> Self {
        Self::DoNotAllow
    }
}

