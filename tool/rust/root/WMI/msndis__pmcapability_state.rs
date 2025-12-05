// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PMCapabilityState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PMCapabilityState {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "NdisPMCapabilityState")]
    pub ndis_pmcapability_state: Option<PMCapabilityState_NdisPMCapabilityState>,
}

impl MSNdis_PMCapabilityState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ndis_pmcapability_state: None,
        }
    }


    /// Sets the value of NdisPMCapabilityState
    pub fn set_ndis_pmcapability_state(&mut self, value: PMCapabilityState_NdisPMCapabilityState) {
        self.ndis_pmcapability_state = Some(value);
    }

    /// Gets the value of NdisPMCapabilityState
    pub fn get_ndis_pmcapability_state(&self) -> Option<&PMCapabilityState_NdisPMCapabilityState> {
        self.ndis_pmcapability_state.as_ref()
    }
}

