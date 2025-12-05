// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BITSServiceTrace_Flags
//////////////////////////////////////////////

/// BITSServiceTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BITSServiceTrace_Flags {
    /// LogFlagInfo
    #[serde(rename = "LogFlagInfo")]
    LogFlagInfo = 1,
    /// LogFlagWarning
    #[serde(rename = "LogFlagWarning")]
    LogFlagWarning = 2,
    /// LogFlagError
    #[serde(rename = "LogFlagError")]
    LogFlagError = 3,
    /// LogFlagFunction
    #[serde(rename = "LogFlagFunction")]
    LogFlagFunction = 4,
    /// LogFlagRefCount
    #[serde(rename = "LogFlagRefCount")]
    LogFlagRefCount = 5,
    /// LogFlagSerialize
    #[serde(rename = "LogFlagSerialize")]
    LogFlagSerialize = 6,
    /// LogFlagDownload
    #[serde(rename = "LogFlagDownload")]
    LogFlagDownload = 7,
    /// LogFlagTask
    #[serde(rename = "LogFlagTask")]
    LogFlagTask = 8,
    /// LogFlagLock
    #[serde(rename = "LogFlagLock")]
    LogFlagLock = 9,
    /// LogFlagService
    #[serde(rename = "LogFlagService")]
    LogFlagService = 10,
    /// LogFlagDataBytes
    #[serde(rename = "LogFlagDataBytes")]
    LogFlagDataBytes = 11,
    /// LogFlagTransferDetails
    #[serde(rename = "LogFlagTransferDetails")]
    LogFlagTransferDetails = 12,
    /// LogFlagPeer
    #[serde(rename = "LogFlagPeer")]
    LogFlagPeer = 13,
}

impl Default for BITSServiceTrace_Flags {
    fn default() -> Self {
        Self::LogFlagInfo
    }
}

