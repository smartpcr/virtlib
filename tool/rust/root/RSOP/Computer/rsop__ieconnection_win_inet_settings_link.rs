// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionWinINetSettingsLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionWinINetSettingsLink {

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,

/// 
    #[serde(rename = "winINetSettings")]
    pub win_inet_settings: Option<RSOP_IEConnectionWinINetSettings>,
}

impl RSOP_IEConnectionWinINetSettingsLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            policy_setting: None,
            win_inet_settings: None,
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

    /// Sets the value of winINetSettings
    pub fn set_win_inet_settings(&mut self, value: RSOP_IEConnectionWinINetSettings) {
        self.win_inet_settings = Some(value);
    }

    /// Gets the value of winINetSettings
    pub fn get_win_inet_settings(&self) -> Option<&RSOP_IEConnectionWinINetSettings> {
        self.win_inet_settings.as_ref()
    }
}

