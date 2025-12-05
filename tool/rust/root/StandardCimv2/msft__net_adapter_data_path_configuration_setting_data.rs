// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterDataPathConfigurationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterDataPathConfigurationSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<String>,

/// 
    #[serde(rename = "ProfileSource")]
    pub profile_source: Option<u32>,
}

impl MSFT_NetAdapterDataPathConfigurationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            profile: None,
            profile_source: None,
        }
    }


    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: String) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&String> {
        self.profile.as_ref()
    }

    /// Sets the value of ProfileSource
    pub fn set_profile_source(&mut self, value: u32) {
        self.profile_source = Some(value);
    }

    /// Gets the value of ProfileSource
    pub fn get_profile_source(&self) -> Option<&u32> {
        self.profile_source.as_ref()
    }
}

