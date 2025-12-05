// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source KerberosDebugTrace_Flags
//////////////////////////////////////////////

/// KerberosDebugTrace_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum KerberosDebugTrace_Flags {
    /// Error
    #[serde(rename = "Error")]
    Error = 1,
    /// Warning
    #[serde(rename = "Warning")]
    Warning = 2,
    /// Trace
    #[serde(rename = "Trace")]
    Trace = 3,
    /// API
    #[serde(rename = "API")]
    API = 4,
    /// Cred
    #[serde(rename = "Cred")]
    Cred = 5,
    /// Context
    #[serde(rename = "Context")]
    Context = 6,
    /// LSess
    #[serde(rename = "LSess")]
    LSess = 7,
    /// Referral
    #[serde(rename = "Referral")]
    Referral = 8,
    /// Logon
    #[serde(rename = "Logon")]
    Logon = 9,
    /// KDC
    #[serde(rename = "KDC")]
    KDC = 10,
    /// Context2
    #[serde(rename = "Context2")]
    Context2 = 11,
    /// Time
    #[serde(rename = "Time")]
    Time = 12,
    /// User
    #[serde(rename = "User")]
    User = 13,
    /// Leaks
    #[serde(rename = "Leaks")]
    Leaks = 14,
    /// Socket
    #[serde(rename = "Socket")]
    Socket = 15,
    /// Spn
    #[serde(rename = "Spn")]
    Spn = 16,
    /// S4uErr
    #[serde(rename = "S4uErr")]
    S4uErr = 17,
    /// S4U
    #[serde(rename = "S4U")]
    S4U = 18,
    /// Bnd
    #[serde(rename = "Bnd")]
    Bnd = 19,
    /// Loopback
    #[serde(rename = "Loopback")]
    Loopback = 20,
    /// Renew
    #[serde(rename = "Renew")]
    Renew = 21,
    /// U2U
    #[serde(rename = "U2U")]
    U2U = 22,
}

impl Default for KerberosDebugTrace_Flags {
    fn default() -> Self {
        Self::Error
    }
}

