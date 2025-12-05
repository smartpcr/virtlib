// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageQoSPolicyToFlow struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageQoSPolicyToFlow {

/// 
    #[serde(rename = "Flow")]
    pub flow: Option<MSFT_StorageQoSFlow>,

/// 
    #[serde(rename = "Policy")]
    pub policy: Option<MSFT_StorageQoSPolicy>,
}

impl MSFT_StorageQoSPolicyToFlow {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            flow: None,
            policy: None,
        }
    }


    /// Sets the value of Flow
    pub fn set_flow(&mut self, value: MSFT_StorageQoSFlow) {
        self.flow = Some(value);
    }

    /// Gets the value of Flow
    pub fn get_flow(&self) -> Option<&MSFT_StorageQoSFlow> {
        self.flow.as_ref()
    }

    /// Sets the value of Policy
    pub fn set_policy(&mut self, value: MSFT_StorageQoSPolicy) {
        self.policy = Some(value);
    }

    /// Gets the value of Policy
    pub fn get_policy(&self) -> Option<&MSFT_StorageQoSPolicy> {
        self.policy.as_ref()
    }
}

