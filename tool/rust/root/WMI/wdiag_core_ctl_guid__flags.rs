// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WDiagCoreCtlGuid_Flags
//////////////////////////////////////////////

/// WDiagCoreCtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WDiagCoreCtlGuid_Flags {
    /// WD_LOG_FLAG_INIT
    #[serde(rename = "WD_LOG_FLAG_INIT")]
    WDLOGFLAGINIT = 1,
    /// WD_LOG_FLAG_RPC
    #[serde(rename = "WD_LOG_FLAG_RPC")]
    WDLOGFLAGRPC = 2,
    /// WD_LOG_FLAG_EVENT
    #[serde(rename = "WD_LOG_FLAG_EVENT")]
    WDLOGFLAGEVENT = 3,
    /// WD_LOG_FLAG_INTERFACE
    #[serde(rename = "WD_LOG_FLAG_INTERFACE")]
    WDLOGFLAGINTERFACE = 4,
    /// WD_LOG_FLAG_CONNECTION
    #[serde(rename = "WD_LOG_FLAG_CONNECTION")]
    WDLOGFLAGCONNECTION = 5,
    /// WD_LOG_FLAG_CONTROL
    #[serde(rename = "WD_LOG_FLAG_CONTROL")]
    WDLOGFLAGCONTROL = 6,
    /// WD_LOG_FLAG_LOCKS
    #[serde(rename = "WD_LOG_FLAG_LOCKS")]
    WDLOGFLAGLOCKS = 7,
    /// WD_LOG_FLAG_MEMORY
    #[serde(rename = "WD_LOG_FLAG_MEMORY")]
    WDLOGFLAGMEMORY = 8,
    /// WD_LOG_FLAG_REFERENCES
    #[serde(rename = "WD_LOG_FLAG_REFERENCES")]
    WDLOGFLAGREFERENCES = 9,
    /// WD_LOG_FLAG_FUNCTION_TRACE
    #[serde(rename = "WD_LOG_FLAG_FUNCTION_TRACE")]
    WDLOGFLAGFUNCTIONTRACE = 10,
    /// WD_LOG_FLAG_ASSERT
    #[serde(rename = "WD_LOG_FLAG_ASSERT")]
    WDLOGFLAGASSERT = 11,
}

impl Default for WDiagCoreCtlGuid_Flags {
    fn default() -> Self {
        Self::WDLOGFLAGINIT
    }
}

