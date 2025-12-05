// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuidWPDSP_Flags
//////////////////////////////////////////////

/// CtlGuidWPDSP_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuidWPDSP_Flags {
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
    /// WPD_TRACE_LEVEL_RESERVED
    #[serde(rename = "WPD_TRACE_LEVEL_RESERVED")]
    WPDTRACELEVELRESERVED = 8,
    /// Enumeration
    #[serde(rename = "Enumeration")]
    Enumeration = 9,
    /// DataXfer
    #[serde(rename = "DataXfer")]
    DataXfer = 10,
    /// Properties
    #[serde(rename = "Properties")]
    Properties = 11,
    /// Perf
    #[serde(rename = "Perf")]
    Perf = 12,
    /// PerfVerbose
    #[serde(rename = "PerfVerbose")]
    PerfVerbose = 13,
    /// WPD_F6
    #[serde(rename = "WPD_F6")]
    WPDF6 = 14,
    /// WPD_F7
    #[serde(rename = "WPD_F7")]
    WPDF7 = 15,
    /// WPD_F8
    #[serde(rename = "WPD_F8")]
    WPDF8 = 16,
    /// Janus
    #[serde(rename = "Janus")]
    Janus = 17,
    /// Extension
    #[serde(rename = "Extension")]
    Extension = 18,
}

impl Default for CtlGuidWPDSP_Flags {
    fn default() -> Self {
        Self::WPDTRACELEVELCRITICAL
    }
}

