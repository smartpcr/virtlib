// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SensorClassExtensionControlGuid_Flags
//////////////////////////////////////////////

/// SensorClassExtensionControlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SensorClassExtensionControlGuid_Flags {
    /// TRACE_NONE
    #[serde(rename = "TRACE_NONE")]
    TRACENONE = 1,
    /// TRACE_FATAL
    #[serde(rename = "TRACE_FATAL")]
    TRACEFATAL = 2,
    /// TRACE_ERROR
    #[serde(rename = "TRACE_ERROR")]
    TRACEERROR = 3,
    /// TRACE_WARNING
    #[serde(rename = "TRACE_WARNING")]
    TRACEWARNING = 4,
    /// TRACE_INFORMATION
    #[serde(rename = "TRACE_INFORMATION")]
    TRACEINFORMATION = 5,
    /// TRACE_VERBOSE
    #[serde(rename = "TRACE_VERBOSE")]
    TRACEVERBOSE = 6,
    /// TRACE_RESERVED6
    #[serde(rename = "TRACE_RESERVED6")]
    TRACERESERVED6 = 7,
    /// TRACE_RESERVED7
    #[serde(rename = "TRACE_RESERVED7")]
    TRACERESERVED7 = 8,
    /// TRACE_RESERVED8
    #[serde(rename = "TRACE_RESERVED8")]
    TRACERESERVED8 = 9,
    /// TRACE_RESERVED9
    #[serde(rename = "TRACE_RESERVED9")]
    TRACERESERVED9 = 10,
    /// TRACE_STACK
    #[serde(rename = "TRACE_STACK")]
    TRACESTACK = 11,
    /// TRACE_RESERVED10
    #[serde(rename = "TRACE_RESERVED10")]
    TRACERESERVED10 = 12,
    /// TRACE_UNITTEST
    #[serde(rename = "TRACE_UNITTEST")]
    TRACEUNITTEST = 13,
    /// TRACE_VERIFICATION
    #[serde(rename = "TRACE_VERIFICATION")]
    TRACEVERIFICATION = 14,
}

impl Default for SensorClassExtensionControlGuid_Flags {
    fn default() -> Self {
        Self::TRACENONE
    }
}

