// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetBranchCacheHostedCacheServerSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetBranchCacheHostedCacheServerSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetBranchCacheSettingData,

/// 
    #[serde(rename = "ClientAuthenticationMode")]
    pub client_authentication_mode: Option<u32>,

/// 
    #[serde(rename = "HostedCacheScpRegistrationEnabled")]
    pub hosted_cache_scp_registration_enabled: Option<bool>,

/// 
    #[serde(rename = "HostedCacheServerIsEnabled")]
    pub hosted_cache_server_is_enabled: Option<bool>,
}

impl MSFT_NetBranchCacheHostedCacheServerSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetBranchCacheSettingData::new(),
            client_authentication_mode: None,
            hosted_cache_scp_registration_enabled: None,
            hosted_cache_server_is_enabled: None,
        }
    }


    /// Sets the value of ClientAuthenticationMode
    pub fn set_client_authentication_mode(&mut self, value: u32) {
        self.client_authentication_mode = Some(value);
    }

    /// Gets the value of ClientAuthenticationMode
    pub fn get_client_authentication_mode(&self) -> Option<&u32> {
        self.client_authentication_mode.as_ref()
    }

    /// Sets the value of HostedCacheScpRegistrationEnabled
    pub fn set_hosted_cache_scp_registration_enabled(&mut self, value: bool) {
        self.hosted_cache_scp_registration_enabled = Some(value);
    }

    /// Gets the value of HostedCacheScpRegistrationEnabled
    pub fn get_hosted_cache_scp_registration_enabled(&self) -> Option<&bool> {
        self.hosted_cache_scp_registration_enabled.as_ref()
    }

    /// Sets the value of HostedCacheServerIsEnabled
    pub fn set_hosted_cache_server_is_enabled(&mut self, value: bool) {
        self.hosted_cache_server_is_enabled = Some(value);
    }

    /// Gets the value of HostedCacheServerIsEnabled
    pub fn get_hosted_cache_server_is_enabled(&self) -> Option<&bool> {
        self.hosted_cache_server_is_enabled.as_ref()
    }
}

