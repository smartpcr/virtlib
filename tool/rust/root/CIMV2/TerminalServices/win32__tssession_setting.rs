// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSSessionSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSSessionSetting {
    #[serde(flatten)]
    pub base: Win32_TerminalSetting,

/// 
    #[serde(rename = "ActiveSessionLimit")]
    pub active_session_limit: Option<u32>,

/// 
    #[serde(rename = "BrokenConnectionAction")]
    pub broken_connection_action: Option<u32>,

/// 
    #[serde(rename = "BrokenConnectionPolicy")]
    pub broken_connection_policy: Option<u32>,

/// 
    #[serde(rename = "DisconnectedSessionLimit")]
    pub disconnected_session_limit: Option<u32>,

/// 
    #[serde(rename = "EnableTimeoutWarning")]
    pub enable_timeout_warning: Option<u32>,

/// 
    #[serde(rename = "IdleSessionLimit")]
    pub idle_session_limit: Option<u32>,

/// 
    #[serde(rename = "PolicySourceActiveSessionLimit")]
    pub policy_source_active_session_limit: Option<u32>,

/// 
    #[serde(rename = "PolicySourceBrokenConnectionAction")]
    pub policy_source_broken_connection_action: Option<u32>,

/// 
    #[serde(rename = "PolicySourceDisconnectedSessionLimit")]
    pub policy_source_disconnected_session_limit: Option<u32>,

/// 
    #[serde(rename = "PolicySourceIdleSessionLimit")]
    pub policy_source_idle_session_limit: Option<u32>,

/// 
    #[serde(rename = "PolicySourceReconnectionPolicy")]
    pub policy_source_reconnection_policy: Option<u32>,

/// 
    #[serde(rename = "ReconnectionPolicy")]
    pub reconnection_policy: Option<u32>,

/// 
    #[serde(rename = "TimeLimitPolicy")]
    pub time_limit_policy: Option<u32>,
}

impl Win32_TSSessionSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_TerminalSetting::new(),
            active_session_limit: None,
            broken_connection_action: None,
            broken_connection_policy: None,
            disconnected_session_limit: None,
            enable_timeout_warning: None,
            idle_session_limit: None,
            policy_source_active_session_limit: None,
            policy_source_broken_connection_action: None,
            policy_source_disconnected_session_limit: None,
            policy_source_idle_session_limit: None,
            policy_source_reconnection_policy: None,
            reconnection_policy: None,
            time_limit_policy: None,
        }
    }


    /// Sets the value of ActiveSessionLimit
    pub fn set_active_session_limit(&mut self, value: u32) {
        self.active_session_limit = Some(value);
    }

    /// Gets the value of ActiveSessionLimit
    pub fn get_active_session_limit(&self) -> Option<&u32> {
        self.active_session_limit.as_ref()
    }

    /// Sets the value of BrokenConnectionAction
    pub fn set_broken_connection_action(&mut self, value: u32) {
        self.broken_connection_action = Some(value);
    }

    /// Gets the value of BrokenConnectionAction
    pub fn get_broken_connection_action(&self) -> Option<&u32> {
        self.broken_connection_action.as_ref()
    }

    /// Sets the value of BrokenConnectionPolicy
    pub fn set_broken_connection_policy(&mut self, value: u32) {
        self.broken_connection_policy = Some(value);
    }

    /// Gets the value of BrokenConnectionPolicy
    pub fn get_broken_connection_policy(&self) -> Option<&u32> {
        self.broken_connection_policy.as_ref()
    }

    /// Sets the value of DisconnectedSessionLimit
    pub fn set_disconnected_session_limit(&mut self, value: u32) {
        self.disconnected_session_limit = Some(value);
    }

    /// Gets the value of DisconnectedSessionLimit
    pub fn get_disconnected_session_limit(&self) -> Option<&u32> {
        self.disconnected_session_limit.as_ref()
    }

    /// Sets the value of EnableTimeoutWarning
    pub fn set_enable_timeout_warning(&mut self, value: u32) {
        self.enable_timeout_warning = Some(value);
    }

    /// Gets the value of EnableTimeoutWarning
    pub fn get_enable_timeout_warning(&self) -> Option<&u32> {
        self.enable_timeout_warning.as_ref()
    }

    /// Sets the value of IdleSessionLimit
    pub fn set_idle_session_limit(&mut self, value: u32) {
        self.idle_session_limit = Some(value);
    }

    /// Gets the value of IdleSessionLimit
    pub fn get_idle_session_limit(&self) -> Option<&u32> {
        self.idle_session_limit.as_ref()
    }

    /// Sets the value of PolicySourceActiveSessionLimit
    pub fn set_policy_source_active_session_limit(&mut self, value: u32) {
        self.policy_source_active_session_limit = Some(value);
    }

    /// Gets the value of PolicySourceActiveSessionLimit
    pub fn get_policy_source_active_session_limit(&self) -> Option<&u32> {
        self.policy_source_active_session_limit.as_ref()
    }

    /// Sets the value of PolicySourceBrokenConnectionAction
    pub fn set_policy_source_broken_connection_action(&mut self, value: u32) {
        self.policy_source_broken_connection_action = Some(value);
    }

    /// Gets the value of PolicySourceBrokenConnectionAction
    pub fn get_policy_source_broken_connection_action(&self) -> Option<&u32> {
        self.policy_source_broken_connection_action.as_ref()
    }

    /// Sets the value of PolicySourceDisconnectedSessionLimit
    pub fn set_policy_source_disconnected_session_limit(&mut self, value: u32) {
        self.policy_source_disconnected_session_limit = Some(value);
    }

    /// Gets the value of PolicySourceDisconnectedSessionLimit
    pub fn get_policy_source_disconnected_session_limit(&self) -> Option<&u32> {
        self.policy_source_disconnected_session_limit.as_ref()
    }

    /// Sets the value of PolicySourceIdleSessionLimit
    pub fn set_policy_source_idle_session_limit(&mut self, value: u32) {
        self.policy_source_idle_session_limit = Some(value);
    }

    /// Gets the value of PolicySourceIdleSessionLimit
    pub fn get_policy_source_idle_session_limit(&self) -> Option<&u32> {
        self.policy_source_idle_session_limit.as_ref()
    }

    /// Sets the value of PolicySourceReconnectionPolicy
    pub fn set_policy_source_reconnection_policy(&mut self, value: u32) {
        self.policy_source_reconnection_policy = Some(value);
    }

    /// Gets the value of PolicySourceReconnectionPolicy
    pub fn get_policy_source_reconnection_policy(&self) -> Option<&u32> {
        self.policy_source_reconnection_policy.as_ref()
    }

    /// Sets the value of ReconnectionPolicy
    pub fn set_reconnection_policy(&mut self, value: u32) {
        self.reconnection_policy = Some(value);
    }

    /// Gets the value of ReconnectionPolicy
    pub fn get_reconnection_policy(&self) -> Option<&u32> {
        self.reconnection_policy.as_ref()
    }

    /// Sets the value of TimeLimitPolicy
    pub fn set_time_limit_policy(&mut self, value: u32) {
        self.time_limit_policy = Some(value);
    }

    /// Gets the value of TimeLimitPolicy
    pub fn get_time_limit_policy(&self) -> Option<&u32> {
        self.time_limit_policy.as_ref()
    }

/// 

    /// * `session_limit_type` -  (String)
    /// * `value_limit` -  (u32)

    /// * `return_value` -  (u32)
    pub fn time_limit(&self, session_limit_type: &String, value_limit: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SessionLimitType".to_string(), value: session_limit_type.into() });
        args.push(MethodParameter { name: "ValueLimit".to_string(), value: value_limit.into() });
        self.invoke_method("TimeLimit", &args)

    }


/// 

    /// * `broken_connection_action` -  (u32)

    /// * `return_value` -  (u32)
    pub fn broken_connection(&self, broken_connection_action: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "BrokenConnectionAction".to_string(), value: broken_connection_action.into() });
        self.invoke_method("BrokenConnection", &args)

    }

}

