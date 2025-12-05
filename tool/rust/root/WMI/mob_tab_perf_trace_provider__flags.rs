// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MobTabPerfTraceProvider_Flags
//////////////////////////////////////////////

/// MobTabPerfTraceProvider_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MobTabPerfTraceProvider_Flags {
    /// TRACE_PERFORMANCE
    #[serde(rename = "TRACE_PERFORMANCE")]
    TRACEPERFORMANCE = 1,
}

impl Default for MobTabPerfTraceProvider_Flags {
    fn default() -> Self {
        Self::TRACEPERFORMANCE
    }
}

