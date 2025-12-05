// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source PrivacyFilter_Ndis80211PrivacyFilter
//////////////////////////////////////////////

/// PrivacyFilter_Ndis80211PrivacyFilter enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum PrivacyFilter_Ndis80211PrivacyFilter {
    /// Ndis802_11PrivFilterAcceptAll
    #[serde(rename = "Ndis802_11PrivFilterAcceptAll")]
    Ndis80211PrivFilterAcceptAll = 0,
    /// Ndis802_11PrivFilter8021xWEP
    #[serde(rename = "Ndis802_11PrivFilter8021xWEP")]
    Ndis80211PrivFilter8021xWEP = 1,
}

impl Default for PrivacyFilter_Ndis80211PrivacyFilter {
    fn default() -> Self {
        Self::Ndis80211PrivFilterAcceptAll
    }
}

