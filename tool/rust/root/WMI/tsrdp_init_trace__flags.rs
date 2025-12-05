// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source TSRdpInitTrace_Flags
//////////////////////////////////////////////

/// TSRdpInitTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum TSRdpInitTrace_Flags {
    /// TS_TRACE_LEVEL_NRM
    #[serde(rename = "TS_TRACE_LEVEL_NRM")]
    TSTRACELEVELNRM = 1,
    /// TS_TRACE_LEVEL_ERROR
    #[serde(rename = "TS_TRACE_LEVEL_ERROR")]
    TSTRACELEVELERROR = 2,
    /// TS_TRACE_LEVEL_ASSERT
    #[serde(rename = "TS_TRACE_LEVEL_ASSERT")]
    TSTRACELEVELASSERT = 3,
}

impl Default for TSRdpInitTrace_Flags {
    fn default() -> Self {
        Self::TSTRACELEVELNRM
    }
}

