// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IELinkItemLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IELinkItemLink {

/// 
    #[serde(rename = "linkItem")]
    pub link_item: Option<RSOP_IELinkItem>,

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,
}

impl RSOP_IELinkItemLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            link_item: None,
            policy_setting: None,
        }
    }


    /// Sets the value of linkItem
    pub fn set_link_item(&mut self, value: RSOP_IELinkItem) {
        self.link_item = Some(value);
    }

    /// Gets the value of linkItem
    pub fn get_link_item(&self) -> Option<&RSOP_IELinkItem> {
        self.link_item.as_ref()
    }

    /// Sets the value of policySetting
    pub fn set_policy_setting(&mut self, value: RSOP_IEAKPolicySetting) {
        self.policy_setting = Some(value);
    }

    /// Gets the value of policySetting
    pub fn get_policy_setting(&self) -> Option<&RSOP_IEAKPolicySetting> {
        self.policy_setting.as_ref()
    }
}

