// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_NetworkInfrastructure struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_NetworkInfrastructure {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Ndis80211NetworkInfrastructure")]
    pub ndis80211_network_infrastructure: Option<NetworkInfrastructure_Ndis80211NetworkInfrastructure>,
}

impl MSNdis_80211_NetworkInfrastructure {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ndis80211_network_infrastructure: None,
        }
    }


    /// Sets the value of Ndis80211NetworkInfrastructure
    pub fn set_ndis80211_network_infrastructure(&mut self, value: NetworkInfrastructure_Ndis80211NetworkInfrastructure) {
        self.ndis80211_network_infrastructure = Some(value);
    }

    /// Gets the value of Ndis80211NetworkInfrastructure
    pub fn get_ndis80211_network_infrastructure(&self) -> Option<&NetworkInfrastructure_Ndis80211NetworkInfrastructure> {
        self.ndis80211_network_infrastructure.as_ref()
    }
}

