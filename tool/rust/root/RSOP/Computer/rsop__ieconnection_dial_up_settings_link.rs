// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionDialUpSettingsLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionDialUpSettingsLink {

/// 
    #[serde(rename = "dialUpSettings")]
    pub dial_up_settings: Option<RSOP_IEConnectionDialUpSettings>,

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,
}

impl RSOP_IEConnectionDialUpSettingsLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dial_up_settings: None,
            policy_setting: None,
        }
    }


    /// Sets the value of dialUpSettings
    pub fn set_dial_up_settings(&mut self, value: RSOP_IEConnectionDialUpSettings) {
        self.dial_up_settings = Some(value);
    }

    /// Gets the value of dialUpSettings
    pub fn get_dial_up_settings(&self) -> Option<&RSOP_IEConnectionDialUpSettings> {
        self.dial_up_settings.as_ref()
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

