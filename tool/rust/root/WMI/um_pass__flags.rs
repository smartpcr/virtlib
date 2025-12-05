// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source UmPass_Flags
//////////////////////////////////////////////

/// UmPass_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum UmPass_Flags {
    /// TraceEntryExit
    #[serde(rename = "TraceEntryExit")]
    TraceEntryExit = 1,
    /// TraceIoctl
    #[serde(rename = "TraceIoctl")]
    TraceIoctl = 2,
    /// TraceParameter
    #[serde(rename = "TraceParameter")]
    TraceParameter = 3,
    /// TraceInit
    #[serde(rename = "TraceInit")]
    TraceInit = 4,
    /// TraceInfo
    #[serde(rename = "TraceInfo")]
    TraceInfo = 5,
    /// TraceDriver
    #[serde(rename = "TraceDriver")]
    TraceDriver = 6,
    /// TraceCreateClose
    #[serde(rename = "TraceCreateClose")]
    TraceCreateClose = 7,
    /// TracePower
    #[serde(rename = "TracePower")]
    TracePower = 8,
    /// TracePnP
    #[serde(rename = "TracePnP")]
    TracePnP = 9,
}

impl Default for UmPass_Flags {
    fn default() -> Self {
        Self::TraceEntryExit
    }
}

