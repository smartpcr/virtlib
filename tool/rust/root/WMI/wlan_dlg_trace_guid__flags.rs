// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WlanDlgTraceGuid_Flags
//////////////////////////////////////////////

/// WlanDlgTraceGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WlanDlgTraceGuid_Flags {
    /// WLANDLG_TRACE
    #[serde(rename = "WLANDLG_TRACE")]
    WLANDLGTRACE = 1,
}

impl Default for WlanDlgTraceGuid_Flags {
    fn default() -> Self {
        Self::WLANDLGTRACE
    }
}

