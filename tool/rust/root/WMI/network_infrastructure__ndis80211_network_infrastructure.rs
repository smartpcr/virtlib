// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NetworkInfrastructure_Ndis80211NetworkInfrastructure
//////////////////////////////////////////////

/// NetworkInfrastructure_Ndis80211NetworkInfrastructure enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NetworkInfrastructure_Ndis80211NetworkInfrastructure {
    /// Ndis802_11IBSS
    #[serde(rename = "Ndis802_11IBSS")]
    Ndis80211IBSS = 0,
    /// Ndis802_11Infrastructure
    #[serde(rename = "Ndis802_11Infrastructure")]
    Ndis80211Infrastructure = 1,
    /// Ndis802_11AutoUnknown
    #[serde(rename = "Ndis802_11AutoUnknown")]
    Ndis80211AutoUnknown = 2,
}

impl Default for NetworkInfrastructure_Ndis80211NetworkInfrastructure {
    fn default() -> Self {
        Self::Ndis80211IBSS
    }
}

