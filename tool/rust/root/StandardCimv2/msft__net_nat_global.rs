// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetNatGlobal struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetNatGlobal {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "InterRoutingDomainHairpinningMode")]
    pub inter_routing_domain_hairpinning_mode: Option<u32>,
}

impl MSFT_NetNatGlobal {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            inter_routing_domain_hairpinning_mode: None,
        }
    }


    /// Sets the value of InterRoutingDomainHairpinningMode
    pub fn set_inter_routing_domain_hairpinning_mode(&mut self, value: u32) {
        self.inter_routing_domain_hairpinning_mode = Some(value);
    }

    /// Gets the value of InterRoutingDomainHairpinningMode
    pub fn get_inter_routing_domain_hairpinning_mode(&self) -> Option<&u32> {
        self.inter_routing_domain_hairpinning_mode.as_ref()
    }
}

