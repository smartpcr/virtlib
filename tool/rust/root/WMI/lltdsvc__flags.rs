// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source LLTDSVC_Flags
//////////////////////////////////////////////

/// LLTDSVC_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum LLTDSVC_Flags {
    /// TRACE_CLASS_CALL
    #[serde(rename = "TRACE_CLASS_CALL")]
    TRACECLASSCALL = 1,
    /// TRACE_CLASS_INIT
    #[serde(rename = "TRACE_CLASS_INIT")]
    TRACECLASSINIT = 2,
    /// TRACE_CLASS_ENGINE
    #[serde(rename = "TRACE_CLASS_ENGINE")]
    TRACECLASSENGINE = 3,
    /// TRACE_CLASS_PACKET
    #[serde(rename = "TRACE_CLASS_PACKET")]
    TRACECLASSPACKET = 4,
    /// TRACE_CLASS_PROTOCOL
    #[serde(rename = "TRACE_CLASS_PROTOCOL")]
    TRACECLASSPROTOCOL = 5,
    /// TRACE_CLASS_ALGORITHM
    #[serde(rename = "TRACE_CLASS_ALGORITHM")]
    TRACECLASSALGORITHM = 6,
    /// TRACE_CLASS_SESSION
    #[serde(rename = "TRACE_CLASS_SESSION")]
    TRACECLASSSESSION = 7,
    /// TRACE_CLASS_TRANSACTION
    #[serde(rename = "TRACE_CLASS_TRANSACTION")]
    TRACECLASSTRANSACTION = 8,
    /// TRACE_CLASS_MAPPING
    #[serde(rename = "TRACE_CLASS_MAPPING")]
    TRACECLASSMAPPING = 9,
}

impl Default for LLTDSVC_Flags {
    fn default() -> Self {
        Self::TRACECLASSCALL
    }
}

