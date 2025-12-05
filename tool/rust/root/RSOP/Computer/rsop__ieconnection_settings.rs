// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionSettings {
    #[serde(flatten)]
    pub base: RSOP_IEProxySettings,

/// 
    #[serde(rename = "autoConfigEnable")]
    pub auto_config_enable: Option<bool>,

/// 
    #[serde(rename = "autoConfigTime")]
    pub auto_config_time: Option<i32>,

/// 
    #[serde(rename = "autoConfigURL")]
    pub auto_config_url: Option<String>,

/// 
    #[serde(rename = "autoConfigUseLocal")]
    pub auto_config_use_local: Option<bool>,

/// 
    #[serde(rename = "autoDetectConfigSettings")]
    pub auto_detect_config_settings: Option<bool>,

/// 
    #[serde(rename = "autoProxyURL")]
    pub auto_proxy_url: Option<String>,

/// 
    #[serde(rename = "defaultDialUpConnection")]
    pub default_dial_up_connection: Option<String>,

/// 
    #[serde(rename = "deleteExistingConnSettings")]
    pub delete_existing_conn_settings: Option<bool>,

/// 
    #[serde(rename = "dialUpConnections")]
    pub dial_up_connections: Vec<String>,

/// 
    #[serde(rename = "dialUpState")]
    pub dial_up_state: Option<u8>,

/// 
    #[serde(rename = "importCurrentConnSettings")]
    pub import_current_conn_settings: Option<bool>,
}

impl RSOP_IEConnectionSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: RSOP_IEProxySettings::new(),
            auto_config_enable: None,
            auto_config_time: None,
            auto_config_url: None,
            auto_config_use_local: None,
            auto_detect_config_settings: None,
            auto_proxy_url: None,
            default_dial_up_connection: None,
            delete_existing_conn_settings: None,
            dial_up_connections: Vec::new(),
            dial_up_state: None,
            import_current_conn_settings: None,
        }
    }


    /// Sets the value of autoConfigEnable
    pub fn set_auto_config_enable(&mut self, value: bool) {
        self.auto_config_enable = Some(value);
    }

    /// Gets the value of autoConfigEnable
    pub fn get_auto_config_enable(&self) -> Option<&bool> {
        self.auto_config_enable.as_ref()
    }

    /// Sets the value of autoConfigTime
    pub fn set_auto_config_time(&mut self, value: i32) {
        self.auto_config_time = Some(value);
    }

    /// Gets the value of autoConfigTime
    pub fn get_auto_config_time(&self) -> Option<&i32> {
        self.auto_config_time.as_ref()
    }

    /// Sets the value of autoConfigURL
    pub fn set_auto_config_url(&mut self, value: String) {
        self.auto_config_url = Some(value);
    }

    /// Gets the value of autoConfigURL
    pub fn get_auto_config_url(&self) -> Option<&String> {
        self.auto_config_url.as_ref()
    }

    /// Sets the value of autoConfigUseLocal
    pub fn set_auto_config_use_local(&mut self, value: bool) {
        self.auto_config_use_local = Some(value);
    }

    /// Gets the value of autoConfigUseLocal
    pub fn get_auto_config_use_local(&self) -> Option<&bool> {
        self.auto_config_use_local.as_ref()
    }

    /// Sets the value of autoDetectConfigSettings
    pub fn set_auto_detect_config_settings(&mut self, value: bool) {
        self.auto_detect_config_settings = Some(value);
    }

    /// Gets the value of autoDetectConfigSettings
    pub fn get_auto_detect_config_settings(&self) -> Option<&bool> {
        self.auto_detect_config_settings.as_ref()
    }

    /// Sets the value of autoProxyURL
    pub fn set_auto_proxy_url(&mut self, value: String) {
        self.auto_proxy_url = Some(value);
    }

    /// Gets the value of autoProxyURL
    pub fn get_auto_proxy_url(&self) -> Option<&String> {
        self.auto_proxy_url.as_ref()
    }

    /// Sets the value of defaultDialUpConnection
    pub fn set_default_dial_up_connection(&mut self, value: String) {
        self.default_dial_up_connection = Some(value);
    }

    /// Gets the value of defaultDialUpConnection
    pub fn get_default_dial_up_connection(&self) -> Option<&String> {
        self.default_dial_up_connection.as_ref()
    }

    /// Sets the value of deleteExistingConnSettings
    pub fn set_delete_existing_conn_settings(&mut self, value: bool) {
        self.delete_existing_conn_settings = Some(value);
    }

    /// Gets the value of deleteExistingConnSettings
    pub fn get_delete_existing_conn_settings(&self) -> Option<&bool> {
        self.delete_existing_conn_settings.as_ref()
    }

    /// Sets the value of dialUpConnections
    pub fn set_dial_up_connections(&mut self, value: Vec<String>) {
        self.dial_up_connections = value;
    }

    /// Gets the value of dialUpConnections
    pub fn get_dial_up_connections(&self) -> &Vec<String> {
        &self.dial_up_connections
    }

    /// Sets the value of dialUpState
    pub fn set_dial_up_state(&mut self, value: u8) {
        self.dial_up_state = Some(value);
    }

    /// Gets the value of dialUpState
    pub fn get_dial_up_state(&self) -> Option<&u8> {
        self.dial_up_state.as_ref()
    }

    /// Sets the value of importCurrentConnSettings
    pub fn set_import_current_conn_settings(&mut self, value: bool) {
        self.import_current_conn_settings = Some(value);
    }

    /// Gets the value of importCurrentConnSettings
    pub fn get_import_current_conn_settings(&self) -> Option<&bool> {
        self.import_current_conn_settings.as_ref()
    }
}

