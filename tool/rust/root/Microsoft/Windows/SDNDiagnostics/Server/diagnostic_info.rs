// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics.Server
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DiagnosticInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticInfo {

/// 
    #[serde(rename = "DeviceType")]
    pub device_type: Option<u32>,

/// 
    #[serde(rename = "IsSDNCtlrPrimaryNode")]
    pub is_sdnctlr_primary_node: Option<bool>,

/// 
    #[serde(rename = "LogLevel")]
    pub log_level: Option<u8>,

/// 
    #[serde(rename = "LogLocation")]
    pub log_location: Option<String>,

/// 
    #[serde(rename = "LogSizeLimit")]
    pub log_size_limit: Option<u32>,

/// 
    #[serde(rename = "LogTimeLimit")]
    pub log_time_limit: Option<u32>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl DiagnosticInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_type: None,
            is_sdnctlr_primary_node: None,
            log_level: None,
            log_location: None,
            log_size_limit: None,
            log_time_limit: None,
            password: None,
            user_name: None,
        }
    }


    /// Sets the value of DeviceType
    pub fn set_device_type(&mut self, value: u32) {
        self.device_type = Some(value);
    }

    /// Gets the value of DeviceType
    pub fn get_device_type(&self) -> Option<&u32> {
        self.device_type.as_ref()
    }

    /// Sets the value of IsSDNCtlrPrimaryNode
    pub fn set_is_sdnctlr_primary_node(&mut self, value: bool) {
        self.is_sdnctlr_primary_node = Some(value);
    }

    /// Gets the value of IsSDNCtlrPrimaryNode
    pub fn get_is_sdnctlr_primary_node(&self) -> Option<&bool> {
        self.is_sdnctlr_primary_node.as_ref()
    }

    /// Sets the value of LogLevel
    pub fn set_log_level(&mut self, value: u8) {
        self.log_level = Some(value);
    }

    /// Gets the value of LogLevel
    pub fn get_log_level(&self) -> Option<&u8> {
        self.log_level.as_ref()
    }

    /// Sets the value of LogLocation
    pub fn set_log_location(&mut self, value: String) {
        self.log_location = Some(value);
    }

    /// Gets the value of LogLocation
    pub fn get_log_location(&self) -> Option<&String> {
        self.log_location.as_ref()
    }

    /// Sets the value of LogSizeLimit
    pub fn set_log_size_limit(&mut self, value: u32) {
        self.log_size_limit = Some(value);
    }

    /// Gets the value of LogSizeLimit
    pub fn get_log_size_limit(&self) -> Option<&u32> {
        self.log_size_limit.as_ref()
    }

    /// Sets the value of LogTimeLimit
    pub fn set_log_time_limit(&mut self, value: u32) {
        self.log_time_limit = Some(value);
    }

    /// Gets the value of LogTimeLimit
    pub fn get_log_time_limit(&self) -> Option<&u32> {
        self.log_time_limit.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

