// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAURun struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAURun {

/// 
    #[serde(rename = "OrchestratorGuid")]
    pub orchestrator_guid: Option<String>,
}

impl MSFT_CAURun {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            orchestrator_guid: None,
        }
    }


    /// Sets the value of OrchestratorGuid
    pub fn set_orchestrator_guid(&mut self, value: String) {
        self.orchestrator_guid = Some(value);
    }

    /// Gets the value of OrchestratorGuid
    pub fn get_orchestrator_guid(&self) -> Option<&String> {
        self.orchestrator_guid.as_ref()
    }
}

