// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source qoswmiQOSWMI_Flags
//////////////////////////////////////////////

/// qoswmiQOSWMI_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum qoswmiQOSWMI_Flags {
    /// QOSWMI_TRACE_CALL
    #[serde(rename = "QOSWMI_TRACE_CALL")]
    QOSWMITRACECALL = 0,
    /// QOSWMI_TRACE_POLICY
    #[serde(rename = "QOSWMI_TRACE_POLICY")]
    QOSWMITRACEPOLICY = 1,
    /// QOSWMI_TRACE_UTILS
    #[serde(rename = "QOSWMI_TRACE_UTILS")]
    QOSWMITRACEUTILS = 2,
}

impl Default for qoswmiQOSWMI_Flags {
    fn default() -> Self {
        Self::QOSWMITRACECALL
    }
}

