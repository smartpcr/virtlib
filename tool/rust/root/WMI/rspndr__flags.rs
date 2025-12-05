// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source RSPNDR_Flags
//////////////////////////////////////////////

/// RSPNDR_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum RSPNDR_Flags {
    /// TRACE_CLASS_CALL
    #[serde(rename = "TRACE_CLASS_CALL")]
    TRACECLASSCALL = 1,
    /// TRACE_CLASS_DISPATCH
    #[serde(rename = "TRACE_CLASS_DISPATCH")]
    TRACECLASSDISPATCH = 2,
    /// TRACE_CLASS_REQUEST
    #[serde(rename = "TRACE_CLASS_REQUEST")]
    TRACECLASSREQUEST = 3,
    /// TRACE_CLASS_BINDING
    #[serde(rename = "TRACE_CLASS_BINDING")]
    TRACECLASSBINDING = 4,
    /// TRACE_CLASS_SESSION
    #[serde(rename = "TRACE_CLASS_SESSION")]
    TRACECLASSSESSION = 5,
    /// TRACE_CLASS_QUIESCENT
    #[serde(rename = "TRACE_CLASS_QUIESCENT")]
    TRACECLASSQUIESCENT = 6,
    /// TRACE_CLASS_HELLO
    #[serde(rename = "TRACE_CLASS_HELLO")]
    TRACECLASSHELLO = 7,
    /// TRACE_CLASS_COMMAND
    #[serde(rename = "TRACE_CLASS_COMMAND")]
    TRACECLASSCOMMAND = 8,
    /// TRACE_CLASS_EMIT
    #[serde(rename = "TRACE_CLASS_EMIT")]
    TRACECLASSEMIT = 9,
    /// TRACE_CLASS_THREAD
    #[serde(rename = "TRACE_CLASS_THREAD")]
    TRACECLASSTHREAD = 10,
    /// TRACE_CLASS_TIMER
    #[serde(rename = "TRACE_CLASS_TIMER")]
    TRACECLASSTIMER = 11,
    /// TRACE_CLASS_QOS
    #[serde(rename = "TRACE_CLASS_QOS")]
    TRACECLASSQOS = 12,
    /// TRACE_CLASS_NLC
    #[serde(rename = "TRACE_CLASS_NLC")]
    TRACECLASSNLC = 13,
}

impl Default for RSPNDR_Flags {
    fn default() -> Self {
        Self::TRACECLASSCALL
    }
}

