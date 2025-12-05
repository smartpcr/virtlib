// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AuthenticationMode_Ndis80211AuthenticationMode
//////////////////////////////////////////////

/// AuthenticationMode_Ndis80211AuthenticationMode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AuthenticationMode_Ndis80211AuthenticationMode {
    /// Ndis802_11AuthModeOpen
    #[serde(rename = "Ndis802_11AuthModeOpen")]
    Ndis80211AuthModeOpen = 0,
    /// Ndis802_11AuthModeShared
    #[serde(rename = "Ndis802_11AuthModeShared")]
    Ndis80211AuthModeShared = 1,
    /// Ndis802_11AuthModeAutoSwitch
    #[serde(rename = "Ndis802_11AuthModeAutoSwitch")]
    Ndis80211AuthModeAutoSwitch = 2,
}

impl Default for AuthenticationMode_Ndis80211AuthenticationMode {
    fn default() -> Self {
        Self::Ndis80211AuthModeOpen
    }
}

