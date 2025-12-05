// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionSettingsLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionSettingsLink {

/// 
    #[serde(rename = "connectionSettings")]
    pub connection_settings: Option<RSOP_IEConnectionSettings>,

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,
}

impl RSOP_IEConnectionSettingsLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_settings: None,
            policy_setting: None,
        }
    }


    /// Sets the value of connectionSettings
    pub fn set_connection_settings(&mut self, value: RSOP_IEConnectionSettings) {
        self.connection_settings = Some(value);
    }

    /// Gets the value of connectionSettings
    pub fn get_connection_settings(&self) -> Option<&RSOP_IEConnectionSettings> {
        self.connection_settings.as_ref()
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

