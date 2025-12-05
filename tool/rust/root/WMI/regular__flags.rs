// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Regular_Flags
//////////////////////////////////////////////

/// Regular_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Regular_Flags {
    /// ERROR
    #[serde(rename = "ERROR")]
    ERROR = 1,
    /// WARNING
    #[serde(rename = "WARNING")]
    WARNING = 2,
    /// TRACE
    #[serde(rename = "TRACE")]
    TRACE = 3,
    /// INFO
    #[serde(rename = "INFO")]
    INFO = 4,
    /// SECURITY
    #[serde(rename = "SECURITY")]
    SECURITY = 5,
    /// CONFIG
    #[serde(rename = "CONFIG")]
    CONFIG = 6,
    /// DEPEND
    #[serde(rename = "DEPEND")]
    DEPEND = 7,
    /// DEPEND_DUMP
    #[serde(rename = "DEPEND_DUMP")]
    DEPENDDUMP = 8,
    /// CONFIG_API
    #[serde(rename = "CONFIG_API")]
    CONFIGAPI = 9,
    /// LOCK_API
    #[serde(rename = "LOCK_API")]
    LOCKAPI = 10,
    /// ACCOUNT
    #[serde(rename = "ACCOUNT")]
    ACCOUNT = 11,
    /// USECOUNT
    #[serde(rename = "USECOUNT")]
    USECOUNT = 12,
    /// NETBIOS
    #[serde(rename = "NETBIOS")]
    NETBIOS = 13,
    /// THREADS
    #[serde(rename = "THREADS")]
    THREADS = 14,
    /// BSM
    #[serde(rename = "BSM")]
    BSM = 15,
    /// SHUTDOWN
    #[serde(rename = "SHUTDOWN")]
    SHUTDOWN = 16,
    /// WHY
    #[serde(rename = "WHY")]
    WHY = 17,
    /// BOOT
    #[serde(rename = "BOOT")]
    BOOT = 18,
    /// HANDLE
    #[serde(rename = "HANDLE")]
    HANDLE = 19,
    /// LOCKS
    #[serde(rename = "LOCKS")]
    LOCKS = 20,
    /// CONTROL
    #[serde(rename = "CONTROL")]
    CONTROL = 21,
}

impl Default for Regular_Flags {
    fn default() -> Self {
        Self::ERROR
    }
}

