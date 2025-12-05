// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WEPStatus_Ndis80211WEPStatus
//////////////////////////////////////////////

/// WEPStatus_Ndis80211WEPStatus enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WEPStatus_Ndis80211WEPStatus {
    /// Ndis802_11WEPEnabled
    #[serde(rename = "Ndis802_11WEPEnabled")]
    Ndis80211WEPEnabled = 0,
    /// Ndis802_11WEPDisabled
    #[serde(rename = "Ndis802_11WEPDisabled")]
    Ndis80211WEPDisabled = 1,
    /// Ndis802_11WEPKeyAbsent
    #[serde(rename = "Ndis802_11WEPKeyAbsent")]
    Ndis80211WEPKeyAbsent = 2,
    /// Ndis802_11WEPNotSupported
    #[serde(rename = "Ndis802_11WEPNotSupported")]
    Ndis80211WEPNotSupported = 3,
}

impl Default for WEPStatus_Ndis80211WEPStatus {
    fn default() -> Self {
        Self::Ndis80211WEPEnabled
    }
}

