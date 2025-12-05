// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionDialUpCredentialsLink struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionDialUpCredentialsLink {

/// 
    #[serde(rename = "dialUpCredentials")]
    pub dial_up_credentials: Option<RSOP_IEConnectionDialUpCredentials>,

/// 
    #[serde(rename = "policySetting")]
    pub policy_setting: Option<RSOP_IEAKPolicySetting>,
}

impl RSOP_IEConnectionDialUpCredentialsLink {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dial_up_credentials: None,
            policy_setting: None,
        }
    }


    /// Sets the value of dialUpCredentials
    pub fn set_dial_up_credentials(&mut self, value: RSOP_IEConnectionDialUpCredentials) {
        self.dial_up_credentials = Some(value);
    }

    /// Gets the value of dialUpCredentials
    pub fn get_dial_up_credentials(&self) -> Option<&RSOP_IEConnectionDialUpCredentials> {
        self.dial_up_credentials.as_ref()
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

