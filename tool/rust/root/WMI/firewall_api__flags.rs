// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FirewallAPI_Flags
//////////////////////////////////////////////

/// FirewallAPI_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FirewallAPI_Flags {
    /// TL_ERROR
    #[serde(rename = "TL_ERROR")]
    TLERROR = 1,
    /// TL_WARN
    #[serde(rename = "TL_WARN")]
    TLWARN = 2,
    /// TL_INFO
    #[serde(rename = "TL_INFO")]
    TLINFO = 3,
    /// TL_FUNC
    #[serde(rename = "TL_FUNC")]
    TLFUNC = 4,
}

impl Default for FirewallAPI_Flags {
    fn default() -> Self {
        Self::TLERROR
    }
}

