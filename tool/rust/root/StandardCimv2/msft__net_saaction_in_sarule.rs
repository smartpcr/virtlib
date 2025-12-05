// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetSAActionInSARule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetSAActionInSARule {
    #[serde(flatten)]
    pub base: CIM_PolicyActionInPolicyRule,
}

impl MSFT_NetSAActionInSARule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyActionInPolicyRule::new(),
        }
    }

}

