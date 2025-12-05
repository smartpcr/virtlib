// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallRuleFilters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallRuleFilters {
    #[serde(flatten)]
    pub base: MSFT_NetPolicyRuleFilters,
}

impl MSFT_NetFirewallRuleFilters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetPolicyRuleFilters::new(),
        }
    }

}

