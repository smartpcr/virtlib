// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WinLogon_Flags
//////////////////////////////////////////////

/// WinLogon_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WinLogon_Flags {
    /// Core
    #[serde(rename = "Core")]
    Core = 0,
    /// Setup
    #[serde(rename = "Setup")]
    Setup = 1,
    /// DataStore
    #[serde(rename = "DataStore")]
    DataStore = 2,
    /// Machine
    #[serde(rename = "Machine")]
    Machine = 3,
    /// Session
    #[serde(rename = "Session")]
    Session = 4,
    /// User
    #[serde(rename = "User")]
    User = 5,
    /// WMsg
    #[serde(rename = "WMsg")]
    WMsg = 6,
    /// Sas
    #[serde(rename = "Sas")]
    Sas = 7,
    /// StateTrace
    #[serde(rename = "StateTrace")]
    StateTrace = 8,
    /// Job
    #[serde(rename = "Job")]
    Job = 9,
    /// Timeout
    #[serde(rename = "Timeout")]
    Timeout = 10,
    /// Power
    #[serde(rename = "Power")]
    Power = 11,
    /// Logon
    #[serde(rename = "Logon")]
    Logon = 12,
    /// Logoff
    #[serde(rename = "Logoff")]
    Logoff = 13,
    /// Lock
    #[serde(rename = "Lock")]
    Lock = 14,
    /// Unlock
    #[serde(rename = "Unlock")]
    Unlock = 15,
    /// ChangePassword
    #[serde(rename = "ChangePassword")]
    ChangePassword = 16,
    /// ScreenSaver
    #[serde(rename = "ScreenSaver")]
    ScreenSaver = 17,
    /// Accessibility
    #[serde(rename = "Accessibility")]
    Accessibility = 18,
    /// SLActivate
    #[serde(rename = "SLActivate")]
    SLActivate = 19,
}

impl Default for WinLogon_Flags {
    fn default() -> Self {
        Self::Core
    }
}

