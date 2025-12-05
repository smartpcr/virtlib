// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WinRM_Flags
//////////////////////////////////////////////

/// WinRM_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WinRM_Flags {
    /// SOAPTraceClient
    #[serde(rename = "SOAPTraceClient")]
    SOAPTraceClient = 1,
    /// SOAPTraceListener
    #[serde(rename = "SOAPTraceListener")]
    SOAPTraceListener = 2,
    /// Reserved2
    #[serde(rename = "Reserved2")]
    Reserved2 = 3,
    /// Reserved3
    #[serde(rename = "Reserved3")]
    Reserved3 = 4,
    /// Reserved4
    #[serde(rename = "Reserved4")]
    Reserved4 = 5,
    /// Reserved5
    #[serde(rename = "Reserved5")]
    Reserved5 = 6,
    /// Performance
    #[serde(rename = "Performance")]
    Performance = 7,
    /// Assert
    #[serde(rename = "Assert")]
    Assert = 8,
    /// RemotingError
    #[serde(rename = "RemotingError")]
    RemotingError = 9,
    /// RemotingDebug
    #[serde(rename = "RemotingDebug")]
    RemotingDebug = 10,
    /// ClientError
    #[serde(rename = "ClientError")]
    ClientError = 11,
    /// ClientDebug
    #[serde(rename = "ClientDebug")]
    ClientDebug = 12,
    /// PSLError
    #[serde(rename = "PSLError")]
    PSLError = 13,
    /// PSLDebug
    #[serde(rename = "PSLDebug")]
    PSLDebug = 14,
    /// SubscriptionError
    #[serde(rename = "SubscriptionError")]
    SubscriptionError = 15,
    /// SubscriptionDebug
    #[serde(rename = "SubscriptionDebug")]
    SubscriptionDebug = 16,
    /// CatalogDebug
    #[serde(rename = "CatalogDebug")]
    CatalogDebug = 17,
    /// CatalogError
    #[serde(rename = "CatalogError")]
    CatalogError = 18,
    /// WSManWmiProvDebug
    #[serde(rename = "WSManWmiProvDebug")]
    WSManWmiProvDebug = 19,
    /// WSManWmiProvError
    #[serde(rename = "WSManWmiProvError")]
    WSManWmiProvError = 20,
    /// ConfigDebug
    #[serde(rename = "ConfigDebug")]
    ConfigDebug = 21,
    /// ConfigError
    #[serde(rename = "ConfigError")]
    ConfigError = 22,
    /// UtilError
    #[serde(rename = "UtilError")]
    UtilError = 23,
    /// UtilDebug
    #[serde(rename = "UtilDebug")]
    UtilDebug = 24,
    /// WSManAutoError
    #[serde(rename = "WSManAutoError")]
    WSManAutoError = 25,
    /// WSManAutoDebug
    #[serde(rename = "WSManAutoDebug")]
    WSManAutoDebug = 26,
    /// WSManAutoFlow
    #[serde(rename = "WSManAutoFlow")]
    WSManAutoFlow = 27,
    /// EventCount
    #[serde(rename = "EventCount")]
    EventCount = 28,
    /// SecurityDebug
    #[serde(rename = "SecurityDebug")]
    SecurityDebug = 29,
    /// Unittest
    #[serde(rename = "Unittest")]
    Unittest = 30,
    /// MemoryDebug
    #[serde(rename = "MemoryDebug")]
    MemoryDebug = 31,
}

impl Default for WinRM_Flags {
    fn default() -> Self {
        Self::SOAPTraceClient
    }
}

