// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuid_Flags
//////////////////////////////////////////////

/// CtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuid_Flags {
    /// WPD_TRACE_LEVEL_CRITICAL
    #[serde(rename = "WPD_TRACE_LEVEL_CRITICAL")]
    WPDTRACELEVELCRITICAL = 1,
    /// WPD_TRACE_LEVEL_ERROR
    #[serde(rename = "WPD_TRACE_LEVEL_ERROR")]
    WPDTRACELEVELERROR = 2,
    /// WPD_TRACE_LEVEL_WARNING
    #[serde(rename = "WPD_TRACE_LEVEL_WARNING")]
    WPDTRACELEVELWARNING = 3,
    /// WPD_TRACE_LEVEL_ASSERT
    #[serde(rename = "WPD_TRACE_LEVEL_ASSERT")]
    WPDTRACELEVELASSERT = 4,
    /// WPD_TRACE_LEVEL_FUNCTRACE
    #[serde(rename = "WPD_TRACE_LEVEL_FUNCTRACE")]
    WPDTRACELEVELFUNCTRACE = 5,
    /// WPD_TRACE_LEVEL_INFORMATION
    #[serde(rename = "WPD_TRACE_LEVEL_INFORMATION")]
    WPDTRACELEVELINFORMATION = 6,
    /// WPD_TRACE_LEVEL_VERBOSE
    #[serde(rename = "WPD_TRACE_LEVEL_VERBOSE")]
    WPDTRACELEVELVERBOSE = 7,
}

impl Default for CtlGuid_Flags {
    fn default() -> Self {
        Self::WPDTRACELEVELCRITICAL
    }
}

