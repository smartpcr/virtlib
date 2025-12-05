// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_NetworkType struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_NetworkType {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Ndis80211NetworkType")]
    pub ndis80211_network_type: Option<u32>,
}

impl MSNdis_80211_NetworkType {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ndis80211_network_type: None,
        }
    }


    /// Sets the value of Ndis80211NetworkType
    pub fn set_ndis80211_network_type(&mut self, value: u32) {
        self.ndis80211_network_type = Some(value);
    }

    /// Gets the value of Ndis80211NetworkType
    pub fn get_ndis80211_network_type(&self) -> Option<&u32> {
        self.ndis80211_network_type.as_ref()
    }
}

