// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ObjectsGuid_Flags
//////////////////////////////////////////////

/// ObjectsGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ObjectsGuid_Flags {
    /// Generic_Exception
    #[serde(rename = "Generic_Exception")]
    GenericException = 1,
    /// Generic_Error
    #[serde(rename = "Generic_Error")]
    GenericError = 2,
    /// Generic_Warn
    #[serde(rename = "Generic_Warn")]
    GenericWarn = 3,
    /// PipeMgr_Trace
    #[serde(rename = "PipeMgr_Trace")]
    PipeMgrTrace = 4,
    /// PipeMgr_Reach
    #[serde(rename = "PipeMgr_Reach")]
    PipeMgrReach = 5,
    /// PipeBuf_Trace
    #[serde(rename = "PipeBuf_Trace")]
    PipeBufTrace = 6,
    /// PTConv_Trace
    #[serde(rename = "PTConv_Trace")]
    PTConvTrace = 7,
    /// Framework_Trace
    #[serde(rename = "Framework_Trace")]
    FrameworkTrace = 8,
    /// Filter_Trace
    #[serde(rename = "Filter_Trace")]
    FilterTrace = 9,
}

impl Default for ObjectsGuid_Flags {
    fn default() -> Self {
        Self::GenericException
    }
}

