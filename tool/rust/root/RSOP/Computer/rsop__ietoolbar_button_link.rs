// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEToolbarButtonLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEToolbarButtonLink {

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,

/// 
    #[serde(rename = "toolbarButton")]
    pub toolbar_button: Option<RSOP_IEToolbarButton>,
}

impl RSOP_IEToolbarButtonLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            policy_setting: None,
            toolbar_button: None,
        }
    }


    /// Sets the value of policySetting
    pub fn set_policy_setting(&mut self, value: RSOP_IEAKPolicySetting) {
        self.policy_setting = Some(value);
    }

    /// Gets the value of policySetting
    pub fn get_policy_setting(&self) -> Option<&RSOP_IEAKPolicySetting> {
        self.policy_setting.as_ref()
    }

    /// Sets the value of toolbarButton
    pub fn set_toolbar_button(&mut self, value: RSOP_IEToolbarButton) {
        self.toolbar_button = Some(value);
    }

    /// Gets the value of toolbarButton
    pub fn get_toolbar_button(&self) -> Option<&RSOP_IEToolbarButton> {
        self.toolbar_button.as_ref()
    }
}

