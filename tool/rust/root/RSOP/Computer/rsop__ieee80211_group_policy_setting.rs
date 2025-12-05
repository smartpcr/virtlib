// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEEE80211GroupPolicySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEEE80211GroupPolicySetting {
    #[serde(flatten)]
    pub base: RSOP_PolicySetting,

/// 
    #[serde(rename = "description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "msieee80211PolicyData")]
    pub msieee80211_policy_data: Option<String>,

/// 
    #[serde(rename = "msieee80211PolicyReserved")]
    pub msieee80211_policy_reserved: Vec<u8>,

/// 
    #[serde(rename = "whenChanged")]
    pub when_changed: Option<u32>,
}

impl RSOP_IEEE80211GroupPolicySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_PolicySetting::new(),
            description: None,
            msieee80211_policy_data: None,
            msieee80211_policy_reserved: Vec::new(),
            when_changed: None,
        }
    }


    /// Sets the value of description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of msieee80211PolicyData
    pub fn set_msieee80211_policy_data(&mut self, value: String) {
        self.msieee80211_policy_data = Some(value);
    }

    /// Gets the value of msieee80211PolicyData
    pub fn get_msieee80211_policy_data(&self) -> Option<&String> {
        self.msieee80211_policy_data.as_ref()
    }

    /// Sets the value of msieee80211PolicyReserved
    pub fn set_msieee80211_policy_reserved(&mut self, value: Vec<u8>) {
        self.msieee80211_policy_reserved = value;
    }

    /// Gets the value of msieee80211PolicyReserved
    pub fn get_msieee80211_policy_reserved(&self) -> &Vec<u8> {
        &self.msieee80211_policy_reserved
    }

    /// Sets the value of whenChanged
    pub fn set_when_changed(&mut self, value: u32) {
        self.when_changed = Some(value);
    }

    /// Gets the value of whenChanged
    pub fn get_when_changed(&self) -> Option<&u32> {
        self.when_changed.as_ref()
    }
}

