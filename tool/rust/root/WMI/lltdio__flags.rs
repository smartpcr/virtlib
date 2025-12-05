// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LLTDIO_Flags
//////////////////////////////////////////////

/// LLTDIO_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LLTDIO_Flags {
    /// TRACE_CLASS_CALL
    #[serde(rename = "TRACE_CLASS_CALL")]
    TRACECLASSCALL = 1,
    /// TRACE_CLASS_DISPATCH
    #[serde(rename = "TRACE_CLASS_DISPATCH")]
    TRACECLASSDISPATCH = 2,
    /// TRACE_CLASS_BINDING
    #[serde(rename = "TRACE_CLASS_BINDING")]
    TRACECLASSBINDING = 3,
    /// TRACE_CLASS_CONTEXT
    #[serde(rename = "TRACE_CLASS_CONTEXT")]
    TRACECLASSCONTEXT = 4,
    /// TRACE_CLASS_QOS
    #[serde(rename = "TRACE_CLASS_QOS")]
    TRACECLASSQOS = 5,
    /// TRACE_CLASS_REQUEST
    #[serde(rename = "TRACE_CLASS_REQUEST")]
    TRACECLASSREQUEST = 6,
}

impl Default for LLTDIO_Flags {
    fn default() -> Self {
        Self::TRACECLASSCALL
    }
}

