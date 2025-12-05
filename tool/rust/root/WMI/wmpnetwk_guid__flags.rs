// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WMPNetwkGuid_Flags
//////////////////////////////////////////////

/// WMPNetwkGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WMPNetwkGuid_Flags {
    /// EntryExit
    #[serde(rename = "EntryExit")]
    EntryExit = 1,
    /// Service
    #[serde(rename = "Service")]
    Service = 2,
    /// Security
    #[serde(rename = "Security")]
    Security = 3,
    /// QueryParser
    #[serde(rename = "QueryParser")]
    QueryParser = 4,
    /// Search
    #[serde(rename = "Search")]
    Search = 5,
    /// HttpServer
    #[serde(rename = "HttpServer")]
    HttpServer = 6,
}

impl Default for WMPNetwkGuid_Flags {
    fn default() -> Self {
        Self::EntryExit
    }
}

