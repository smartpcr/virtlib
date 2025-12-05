// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_PMAdminConfigState struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_PMAdminConfigState {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "NdisPMAdminConfigState")]
    pub ndis_pmadmin_config_state: Option<PMAdminConfigState_NdisPMAdminConfigState>,
}

impl MSNdis_PMAdminConfigState {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            ndis_pmadmin_config_state: None,
        }
    }


    /// Sets the value of NdisPMAdminConfigState
    pub fn set_ndis_pmadmin_config_state(&mut self, value: PMAdminConfigState_NdisPMAdminConfigState) {
        self.ndis_pmadmin_config_state = Some(value);
    }

    /// Gets the value of NdisPMAdminConfigState
    pub fn get_ndis_pmadmin_config_state(&self) -> Option<&PMAdminConfigState_NdisPMAdminConfigState> {
        self.ndis_pmadmin_config_state.as_ref()
    }
}

