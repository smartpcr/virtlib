// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Appv
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AppvPublishingServer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppvPublishingServer {

/// 
    #[serde(rename = "GlobalRefreshEnabled")]
    pub global_refresh_enabled: Option<bool>,

/// 
    #[serde(rename = "GlobalRefreshInterval")]
    pub global_refresh_interval: Option<u32>,

/// 
    #[serde(rename = "GlobalRefreshIntervalUnit")]
    pub global_refresh_interval_unit: Option<String>,

/// 
    #[serde(rename = "GlobalRefreshOnLogon")]
    pub global_refresh_on_logon: Option<bool>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<u32>,

/// 
    #[serde(rename = "SetByGroupPolicy")]
    pub set_by_group_policy: Option<bool>,

/// 
    #[serde(rename = "Url")]
    pub url: Option<String>,

/// 
    #[serde(rename = "UserRefreshEnabled")]
    pub user_refresh_enabled: Option<bool>,

/// 
    #[serde(rename = "UserRefreshInterval")]
    pub user_refresh_interval: Option<u32>,

/// 
    #[serde(rename = "UserRefreshIntervalUnit")]
    pub user_refresh_interval_unit: Option<String>,

/// 
    #[serde(rename = "UserRefreshOnLogon")]
    pub user_refresh_on_logon: Option<bool>,
}

impl AppvPublishingServer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            global_refresh_enabled: None,
            global_refresh_interval: None,
            global_refresh_interval_unit: None,
            global_refresh_on_logon: None,
            id: None,
            set_by_group_policy: None,
            url: None,
            user_refresh_enabled: None,
            user_refresh_interval: None,
            user_refresh_interval_unit: None,
            user_refresh_on_logon: None,
        }
    }


    /// Sets the value of GlobalRefreshEnabled
    pub fn set_global_refresh_enabled(&mut self, value: bool) {
        self.global_refresh_enabled = Some(value);
    }

    /// Gets the value of GlobalRefreshEnabled
    pub fn get_global_refresh_enabled(&self) -> Option<&bool> {
        self.global_refresh_enabled.as_ref()
    }

    /// Sets the value of GlobalRefreshInterval
    pub fn set_global_refresh_interval(&mut self, value: u32) {
        self.global_refresh_interval = Some(value);
    }

    /// Gets the value of GlobalRefreshInterval
    pub fn get_global_refresh_interval(&self) -> Option<&u32> {
        self.global_refresh_interval.as_ref()
    }

    /// Sets the value of GlobalRefreshIntervalUnit
    pub fn set_global_refresh_interval_unit(&mut self, value: String) {
        self.global_refresh_interval_unit = Some(value);
    }

    /// Gets the value of GlobalRefreshIntervalUnit
    pub fn get_global_refresh_interval_unit(&self) -> Option<&String> {
        self.global_refresh_interval_unit.as_ref()
    }

    /// Sets the value of GlobalRefreshOnLogon
    pub fn set_global_refresh_on_logon(&mut self, value: bool) {
        self.global_refresh_on_logon = Some(value);
    }

    /// Gets the value of GlobalRefreshOnLogon
    pub fn get_global_refresh_on_logon(&self) -> Option<&bool> {
        self.global_refresh_on_logon.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }

    /// Sets the value of SetByGroupPolicy
    pub fn set_set_by_group_policy(&mut self, value: bool) {
        self.set_by_group_policy = Some(value);
    }

    /// Gets the value of SetByGroupPolicy
    pub fn get_set_by_group_policy(&self) -> Option<&bool> {
        self.set_by_group_policy.as_ref()
    }

    /// Sets the value of Url
    pub fn set_url(&mut self, value: String) {
        self.url = Some(value);
    }

    /// Gets the value of Url
    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }

    /// Sets the value of UserRefreshEnabled
    pub fn set_user_refresh_enabled(&mut self, value: bool) {
        self.user_refresh_enabled = Some(value);
    }

    /// Gets the value of UserRefreshEnabled
    pub fn get_user_refresh_enabled(&self) -> Option<&bool> {
        self.user_refresh_enabled.as_ref()
    }

    /// Sets the value of UserRefreshInterval
    pub fn set_user_refresh_interval(&mut self, value: u32) {
        self.user_refresh_interval = Some(value);
    }

    /// Gets the value of UserRefreshInterval
    pub fn get_user_refresh_interval(&self) -> Option<&u32> {
        self.user_refresh_interval.as_ref()
    }

    /// Sets the value of UserRefreshIntervalUnit
    pub fn set_user_refresh_interval_unit(&mut self, value: String) {
        self.user_refresh_interval_unit = Some(value);
    }

    /// Gets the value of UserRefreshIntervalUnit
    pub fn get_user_refresh_interval_unit(&self) -> Option<&String> {
        self.user_refresh_interval_unit.as_ref()
    }

    /// Sets the value of UserRefreshOnLogon
    pub fn set_user_refresh_on_logon(&mut self, value: bool) {
        self.user_refresh_on_logon = Some(value);
    }

    /// Gets the value of UserRefreshOnLogon
    pub fn get_user_refresh_on_logon(&self) -> Option<&bool> {
        self.user_refresh_on_logon.as_ref()
    }
}

