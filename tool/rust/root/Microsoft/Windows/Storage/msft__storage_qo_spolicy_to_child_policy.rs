// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageQoSPolicyToChildPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageQoSPolicyToChildPolicy {

/// 
    #[serde(rename = "ChildPolicy")]
    pub child_policy: Option<MSFT_StorageQoSPolicy>,

/// 
    #[serde(rename = "ParentPolicy")]
    pub parent_policy: Option<MSFT_StorageQoSPolicy>,
}

impl MSFT_StorageQoSPolicyToChildPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            child_policy: None,
            parent_policy: None,
        }
    }


    /// Sets the value of ChildPolicy
    pub fn set_child_policy(&mut self, value: MSFT_StorageQoSPolicy) {
        self.child_policy = Some(value);
    }

    /// Gets the value of ChildPolicy
    pub fn get_child_policy(&self) -> Option<&MSFT_StorageQoSPolicy> {
        self.child_policy.as_ref()
    }

    /// Sets the value of ParentPolicy
    pub fn set_parent_policy(&mut self, value: MSFT_StorageQoSPolicy) {
        self.parent_policy = Some(value);
    }

    /// Gets the value of ParentPolicy
    pub fn get_parent_policy(&self) -> Option<&MSFT_StorageQoSPolicy> {
        self.parent_policy.as_ref()
    }
}

