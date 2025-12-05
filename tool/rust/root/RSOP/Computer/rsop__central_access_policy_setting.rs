// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_CentralAccessPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_CentralAccessPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "CentralAccessPolicyName")]
    pub central_access_policy_name: Vec<String>,
}

impl RSOP_CentralAccessPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            central_access_policy_name: Vec::new(),
        }
    }


    /// Sets the value of CentralAccessPolicyName
    pub fn set_central_access_policy_name(&mut self, value: Vec<String>) {
        self.central_access_policy_name = value;
    }

    /// Gets the value of CentralAccessPolicyName
    pub fn get_central_access_policy_name(&self) -> &Vec<String> {
        &self.central_access_policy_name
    }
}

